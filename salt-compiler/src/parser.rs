use crate::{
    error::{ExpectedToken, ParseError, TypeCheckError},
    instruction::{
        Instruction, block::Block, function_call::FunctionCall,
        function_declaration::FunctionDeclaration, intrinsic::Intrinsic, r#loop::Loop,
        string_literal::StringLiteral, r#use::Use,
    },
    ir_generator::IrGenerator,
    lexer::{Keyword, Location, Token, TokenKind, Type},
    type_checker::TypeChecker,
};

#[derive(Debug, Clone)]
pub struct Statement {
    pub kind: StatementKind,
    pub location: Location,
    pub returns: bool,
}

#[derive(Debug, Clone)]
pub enum StatementKind {
    Module { name: String, ast: Vec<Statement> },

    FunctionDeclaration(FunctionDeclaration),
    FunctionCall(FunctionCall),

    Intrinsic(Intrinsic),

    Use(Use),

    StringLiteral(StringLiteral),

    Loop(Loop),
    Block(Block),

    Empty,
}

impl Instruction for Statement {
    fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        let token = if let Some(token) = parser.peek() {
            token
        } else {
            return Err(ParseError::UnexpectedEOF);
        };
        let location = token.location.clone();

        let kind = match token.kind {
            TokenKind::Keyword(Keyword::Use) => StatementKind::Use(Use::parse(parser)?),
            TokenKind::Keyword(Keyword::Fn) | TokenKind::Keyword(Keyword::Raw) => {
                StatementKind::FunctionDeclaration(FunctionDeclaration::parse(parser)?)
            }
            TokenKind::Keyword(Keyword::Loop) => StatementKind::Loop(Loop::parse(parser)?),

            TokenKind::StringLiteral(_) => {
                StatementKind::StringLiteral(StringLiteral::parse(parser)?)
            }

            TokenKind::Intrinsic(_) => StatementKind::Intrinsic(Intrinsic::parse(parser)?),
            TokenKind::Identifier(_) => StatementKind::FunctionCall(FunctionCall::parse(parser)?),
            TokenKind::Type(_) => Err(ParseError::UnexpectedToken {
                actual: Box::new(token),
                expected: ExpectedToken::Statement,
            })?,
            TokenKind::PathSeparator => Err(ParseError::UnexpectedToken {
                actual: Box::new(token),
                expected: ExpectedToken::Statement,
            })?,
            TokenKind::Arrow => Err(ParseError::UnexpectedToken {
                actual: Box::new(token),
                expected: ExpectedToken::Statement,
            })?,
            TokenKind::OpenParen => Err(ParseError::UnexpectedToken {
                actual: Box::new(token),
                expected: ExpectedToken::Statement,
            })?,
            TokenKind::CloseParen => Err(ParseError::UnexpectedToken {
                actual: Box::new(token),
                expected: ExpectedToken::Statement,
            })?,
            TokenKind::OpenBrace => StatementKind::Block(Block::parse(parser)?),
            TokenKind::CloseBrace => Err(ParseError::UnexpectedToken {
                actual: Box::new(token),
                expected: ExpectedToken::Statement,
            })?,
            TokenKind::Semicolon => StatementKind::Empty,
        };

        let returns = match &kind {
            StatementKind::FunctionDeclaration(function_declaration) => {
                function_declaration.body.last().returns
            }
            StatementKind::Loop(r#loop) => r#loop.body.last().returns,
            StatementKind::Block(block) => {
                parser.end_statement(true)?;
                match block.body.last() {
                    Some(statement) => statement.last().returns,
                    None => false,
                }
            }
            _ => parser.end_statement(false)?,
        };
        Ok(Statement {
            kind,
            location,
            returns,
        })
    }

    fn check(&self, type_checker: &mut TypeChecker) -> Result<Type, TypeCheckError> {
        let result = match &self.kind {
            StatementKind::Module { ast, .. } => {
                for statement in ast {
                    statement.check(type_checker)?;
                }
                Ok(Type::Void)
            }

            StatementKind::FunctionDeclaration(function_declaration) => {
                function_declaration.check(type_checker)
            }
            StatementKind::FunctionCall(function_call) => function_call.check(type_checker),
            StatementKind::Intrinsic(intrinsic) => intrinsic.check(type_checker),

            StatementKind::Use(r#use) => r#use.check(type_checker),

            StatementKind::StringLiteral(string_literal) => string_literal.check(type_checker),

            StatementKind::Loop(r#loop) => r#loop.check(type_checker),
            StatementKind::Block(block) => {
                type_checker.build_symbol_table(self);
                block.check(type_checker)
            }

            StatementKind::Empty => Ok(Type::Void),
        };
        match result {
            Ok(r) => Ok(r),
            Err(e) => match e {
                e @ TypeCheckError::UnsafeUse(_) => Err(e),
                TypeCheckError::UnsafeUseNoToken => {
                    Err(TypeCheckError::UnsafeUse(self.location.clone()))
                }
                e @ TypeCheckError::UndeclaredFunction(_) => Err(e),
                TypeCheckError::UndeclaredFunctionNoToken => {
                    Err(TypeCheckError::UndeclaredFunction(self.location.clone()))
                }

                e @ TypeCheckError::MismatchedType { .. } => Err(e),
            },
        }
    }

    fn gen_ir(&self, ir_generator: &mut IrGenerator) {
        match &self.kind {
            StatementKind::Module { ast, .. } => {
                for statement in ast {
                    statement.gen_ir(ir_generator);
                    let stash = &ir_generator.pop_stash();
                    ir_generator.ir += stash;
                }
            }

            StatementKind::FunctionDeclaration(function_declaration) => {
                function_declaration.gen_ir(ir_generator)
            }
            StatementKind::FunctionCall(function_call) => {
                function_call.gen_ir(ir_generator);
                if !self.returns {
                    ir_generator.stash = format!(
                        "%{} = {}",
                        ir_generator.new_value(),
                        ir_generator.pop_stash()
                    );
                }
            }
            StatementKind::Intrinsic(intrinsic) => intrinsic.gen_ir(ir_generator),

            StatementKind::Use(r#use) => r#use.gen_ir(ir_generator),

            StatementKind::StringLiteral(string_literal) => string_literal.gen_ir(ir_generator),

            StatementKind::Loop(r#loop) => r#loop.gen_ir(ir_generator),
            StatementKind::Block(block) => block.gen_ir(ir_generator),

            StatementKind::Empty => (),
        }

        if self.returns && std::ptr::eq(self.last(), self) {
            ir_generator.stash = format!(
                "%{} = {}",
                ir_generator.new_value(),
                ir_generator.pop_stash()
            );
        }
    }
}

impl Statement {
    pub const EMPTY: Self = Self {
        kind: StatementKind::Empty,
        location: Location {
            line: 0,
            col: 0,
            length: 0,
            source: [None, None, None],
        },
        returns: false,
    };

    pub fn last(&self) -> &Statement {
        match &self.kind {
            StatementKind::Module { ast, .. } => match ast.last() {
                Some(statement) => statement.last(),
                None => &Statement::EMPTY,
            },
            StatementKind::FunctionDeclaration(function_declaration) => {
                function_declaration.body.last()
            }
            StatementKind::FunctionCall(_) => self,
            StatementKind::Intrinsic(_) => self,

            StatementKind::Use(_) => self,

            StatementKind::StringLiteral(_) => self,

            StatementKind::Loop(r#loop) => r#loop.body.last(),
            StatementKind::Block(block) => match block.body.last() {
                Some(statement) => statement.last(),
                None => &Statement::EMPTY,
            },
            StatementKind::Empty => self,
        }
    }
}

pub struct Parser {
    module_name: String,
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>, module_name: String) -> Self {
        Self {
            module_name,
            tokens,
            pos: 0,
        }
    }

    pub fn peek(&self) -> Option<Token> {
        self.tokens.get(self.pos).cloned()
    }

    pub fn bump(&mut self) -> Option<Token> {
        let token = self.peek();
        self.pos += 1;
        token
    }

    pub fn expect(&mut self, expected: &TokenKind) -> Result<(), ParseError> {
        if let Some(token) = self.bump()
            && token.kind != *expected
        {
            Err(ParseError::UnexpectedToken {
                actual: Box::new(token),
                expected: ExpectedToken::Specific {
                    kind: expected.clone(),
                },
            })
        } else {
            Ok(())
        }
    }

    pub fn expect_ident(&mut self) -> Result<String, ParseError> {
        let token = self.bump();
        if let Some(Token {
            kind: TokenKind::Identifier(name),
            ..
        }) = token
        {
            Ok(name.clone())
        } else if let Some(token) = token {
            Err(ParseError::UnexpectedToken {
                actual: Box::new(token),
                expected: ExpectedToken::Specific {
                    kind: TokenKind::Identifier(String::new()),
                },
            })
        } else {
            Err(ParseError::UnexpectedEOF)
        }
    }

    pub fn expect_optional_type(&mut self) -> Result<Type, ParseError> {
        let token = self.peek();
        if let Some(Token {
            kind: TokenKind::Arrow,
            ..
        }) = token
        {
            // Skip the arrow
            self.bump();
            match self.bump() {
                Some(Token {
                    kind: TokenKind::Type(t),
                    ..
                }) => Ok(t),
                Some(token) => Err(ParseError::UnexpectedToken {
                    actual: Box::new(token),
                    expected: ExpectedToken::Specific {
                        kind: TokenKind::Type(Type::Any),
                    },
                }),
                _ => Err(ParseError::UnexpectedEOF),
            }
        } else {
            Ok(Type::Void)
        }
    }

    // Returns if the statement returns (does not end in a semicolon)
    pub fn end_statement(&mut self, block: bool) -> Result<bool, ParseError> {
        match self.peek() {
            Some(Token {
                kind: TokenKind::Semicolon,
                ..
            }) => {
                self.bump();
                Ok(false)
            }
            Some(Token {
                kind: TokenKind::CloseBrace,
                ..
            }) => {
                if block {
                    self.bump();
                }
                Ok(true)
            }
            Some(token) => {
                self.bump();
                Err(ParseError::UnexpectedToken {
                    actual: Box::new(token),
                    expected: ExpectedToken::Specific {
                        kind: TokenKind::Semicolon,
                    },
                })
            }
            _ => Err(ParseError::UnexpectedEOF),
        }
    }

    pub fn block_returns(&mut self) -> bool {
        let mut i = 0;
        while self.tokens.get(self.pos - i).unwrap().kind != TokenKind::CloseBrace {
            i += 1;
        }

        self.tokens.get(self.pos - i).unwrap().kind != TokenKind::Semicolon
    }

    pub fn parse(mut self) -> Result<Statement, ParseError> {
        let mut ast = Vec::new();
        while self.peek().is_some() {
            ast.push(Statement::parse(&mut self)?);
        }

        Ok(Statement {
            returns: match ast.last() {
                Some(s) => s.last().returns,
                None => false,
            },
            kind: StatementKind::Module {
                name: self.module_name,
                ast,
            },
            location: Location::default(),
        })
    }
}
