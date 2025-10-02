use std::collections::HashMap;

use crate::{
    error::{ExpectedToken, ParseError, TypeCheckError},
    instruction::{
        Instruction, block::Block, function_call::FunctionCall,
        function_declaration::FunctionDeclaration, intrinsic::Intrinsic, r#loop::Loop,
    },
    ir_generator::IrGenerator,
    lexer::{Keyword, Location, Token, TokenKind, Type},
    type_checker::TypeChecker,
};

#[derive(Debug, Clone)]
pub struct Statement {
    pub kind: StatementKind,
    pub location: Location,
}

#[derive(Debug, Clone)]
pub enum StatementKind {
    Module(Vec<Statement>),

    FunctionDeclaration(FunctionDeclaration),
    FunctionCall(FunctionCall),

    Intrinsic(Intrinsic),

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
            TokenKind::Keyword(Keyword::Fn) | TokenKind::Keyword(Keyword::Raw) => {
                StatementKind::FunctionDeclaration(FunctionDeclaration::parse(parser)?)
            }
            TokenKind::Keyword(Keyword::Loop) => StatementKind::Loop(Loop::parse(parser)?),
            TokenKind::Intrinsic(_) => StatementKind::Intrinsic(Intrinsic::parse(parser)?),
            TokenKind::Identifier(_) => StatementKind::FunctionCall(FunctionCall::parse(parser)?),
            TokenKind::Type(_) => Err(ParseError::UnexpectedToken {
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

        match kind {
            StatementKind::FunctionDeclaration(_) | StatementKind::Loop(_) => (),
            StatementKind::Block(_) => parser.end_statement(true)?,
            _ => parser.end_statement(false)?,
        }
        Ok(Statement { kind, location })
    }

    fn check(&self, type_checker: &mut TypeChecker) -> Result<Type, TypeCheckError> {
        let result = match &self.kind {
            StatementKind::FunctionDeclaration(function_declaration) => {
                function_declaration.check(type_checker)
            }
            StatementKind::FunctionCall(function_call) => function_call.check(type_checker),
            StatementKind::Intrinsic(intrinsic) => intrinsic.check(type_checker),
            StatementKind::Loop(r#loop) => r#loop.check(type_checker),
            StatementKind::Block(block) => {
                type_checker.functions.push(HashMap::new());
                type_checker.build_symbol_table(self);
                let t = block.check(type_checker)?;
                type_checker.functions.pop();
                Ok(t)
            }
            StatementKind::Empty => Ok(Type::Void),

            StatementKind::Module(ast) => {
                for statement in ast {
                    statement.check(type_checker)?;
                }
                Ok(Type::Void)
            }
        };
        match result {
            Ok(r) => Ok(r),
            Err(e) => match e {
                TypeCheckError::UnsafeUse(token) => Err(TypeCheckError::UnsafeUse(token)),
                TypeCheckError::UnsafeUseNoToken => {
                    Err(TypeCheckError::UnsafeUse(self.location.clone()))
                }
                TypeCheckError::UndeclaredFunction(location) => {
                    Err(TypeCheckError::UndeclaredFunction(location))
                }
                TypeCheckError::UndeclaredFunctionNoToken => {
                    Err(TypeCheckError::UndeclaredFunction(self.location.clone()))
                }
            },
        }
    }

    fn gen_ir(&self, ir_generator: &mut IrGenerator) -> String {
        match &self.kind {
            StatementKind::FunctionDeclaration(function_declaration) => {
                function_declaration.gen_ir(ir_generator)
            }
            StatementKind::FunctionCall(function_call) => function_call.gen_ir(ir_generator),
            StatementKind::Intrinsic(intrinsic) => intrinsic.gen_ir(ir_generator),

            StatementKind::Loop(r#loop) => r#loop.gen_ir(ir_generator),
            StatementKind::Block(block) => block.gen_ir(ir_generator),

            StatementKind::Empty => String::new(),

            StatementKind::Module(ast) => {
                let mut ir = String::new();
                for statement in ast {
                    ir += &statement.gen_ir(ir_generator);
                }
                ir
            }
        }
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
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

    pub fn end_statement(&mut self, block: bool) -> Result<(), ParseError> {
        match self.peek() {
            Some(Token {
                kind: TokenKind::Semicolon,
                ..
            }) => {
                self.bump();
                Ok(())
            }
            Some(Token {
                kind: TokenKind::CloseBrace,
                ..
            }) => {
                if block {
                    self.bump();
                }
                Ok(())
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

    pub fn parse(&mut self) -> Result<Statement, ParseError> {
        let mut ast = Vec::new();
        while self.peek().is_some() {
            ast.push(Statement::parse(self)?);
        }

        Ok(Statement {
            kind: StatementKind::Module(ast),
            location: Location::default(),
        })
    }
}
