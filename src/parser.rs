use anyhow::Result;
use logos::Span;

use crate::{
    error::{ExpectedToken, ParseError, ParseErrorKind},
    instruction::{
        Instruction, block::Block, function_declaration::FunctionDeclaration, intrinsic::Intrinsic,
        r#loop::Loop,
    },
    ir_generator::IrGenerator,
    lexer::{Keyword, Token, TokenKind, Type},
    type_checker::TypeChecker,
};

#[derive(Debug)]
pub struct Statement {
    kind: StatementKind,
    span: Span,
}

#[derive(Debug)]
pub enum StatementKind {
    FunctionDeclaration(FunctionDeclaration),

    Intrinsic(Intrinsic),

    Loop(Loop),

    Block(Block),

    Empty,
}

impl Statement {
    fn throw_parse_error(kind: ParseErrorKind, token: Option<Token>) -> Result<StatementKind> {
        Err(ParseError::new(kind, token).into())
    }
}

impl Instruction for Statement {
    fn parse(parser: &mut Parser) -> Result<Self> {
        let token = if let Some(token) = parser.peek() {
            token
        } else {
            return Err(ParseError {
                kind: ParseErrorKind::UnexpectedEOF,
                token: None,
            }
            .into());
        };
        let span = token.span.clone();

        let kind = match token.kind {
            TokenKind::Keyword(Keyword::Fn) | TokenKind::Keyword(Keyword::Raw) => {
                StatementKind::FunctionDeclaration(FunctionDeclaration::parse(parser)?)
            }
            TokenKind::Keyword(Keyword::Loop) => StatementKind::Loop(Loop::parse(parser)?),
            TokenKind::Intrinsic(_) => StatementKind::Intrinsic(Intrinsic::parse(parser)?),
            TokenKind::Ident(_) => Self::throw_parse_error(
                ParseErrorKind::UnexpectedToken(ExpectedToken::Statement),
                Some(token),
            )?,
            TokenKind::Type(_) => Self::throw_parse_error(
                ParseErrorKind::UnexpectedToken(ExpectedToken::Statement),
                Some(token),
            )?,
            TokenKind::ReturnType => Self::throw_parse_error(
                ParseErrorKind::UnexpectedToken(ExpectedToken::Statement),
                Some(token),
            )?,
            TokenKind::OpenParen => Self::throw_parse_error(
                ParseErrorKind::UnexpectedToken(ExpectedToken::Statement),
                Some(token),
            )?,
            TokenKind::CloseParen => Self::throw_parse_error(
                ParseErrorKind::UnexpectedToken(ExpectedToken::Statement),
                Some(token),
            )?,
            TokenKind::OpenBrace => StatementKind::Block(Block::parse(parser)?),
            TokenKind::CloseBrace => Self::throw_parse_error(
                ParseErrorKind::UnexpectedToken(ExpectedToken::Statement),
                Some(token),
            )?,
            TokenKind::Semicolon => StatementKind::Empty,
        };

        match kind {
            StatementKind::FunctionDeclaration(_) | StatementKind::Loop(_) => (),
            _ => parser.end_statement(),
        }
        Ok(Statement { kind, span })
    }

    fn check(&self, type_checker: &mut TypeChecker) {
        match &self.kind {
            StatementKind::FunctionDeclaration(function_declaration) => {
                function_declaration.check(type_checker)
            }
            StatementKind::Intrinsic(intrinsic) => intrinsic.check(type_checker),
            StatementKind::Loop(r#loop) => r#loop.check(type_checker),
            StatementKind::Block(block) => block.check(type_checker),
            StatementKind::Empty => (),
        }
    }

    fn gen_ir(&self, ir_generator: &mut IrGenerator) -> String {
        match &self.kind {
            StatementKind::FunctionDeclaration(function_declaration) => {
                function_declaration.gen_ir(ir_generator)
            }
            StatementKind::Intrinsic(intrinsic) => intrinsic.gen_ir(ir_generator),
            StatementKind::Loop(r#loop) => r#loop.gen_ir(ir_generator),
            StatementKind::Block(block) => block.gen_ir(ir_generator),
            StatementKind::Empty => String::new(),
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

    pub fn expect(&mut self, expected: &TokenKind) -> Result<()> {
        if let Some(token) = self.bump()
            && token.kind != *expected
        {
            Err(ParseError::new(
                ParseErrorKind::UnexpectedToken(ExpectedToken::Specific(expected.clone())),
                Some(token),
            )
            .into())
        } else {
            Ok(())
        }
    }

    pub fn expect_without_increment(&mut self, expected: &TokenKind) -> Result<()> {
        if let Some(token) = self.peek()
            && token.kind != *expected
        {
            Err(ParseError::new(
                ParseErrorKind::UnexpectedToken(ExpectedToken::Specific(expected.clone())),
                Some(token),
            )
            .into())
        } else {
            Ok(())
        }
    }

    pub fn expect_ident(&mut self) -> String {
        let token = self.bump();
        if let Some(Token {
            kind: TokenKind::Ident(name),
            ..
        }) = token
        {
            name.clone()
        } else if let Some(token) = token {
            panic!("Expected Identifier, got {:?}", token);
        } else {
            panic!("Reached EOF!");
        }
    }

    pub fn expect_optional_type(&mut self) -> Type {
        let token = self.peek();
        if let Some(Token {
            kind: TokenKind::ReturnType,
            ..
        }) = token
        {
            self.bump();
            if let Some(Token {
                kind: TokenKind::Type(t),
                ..
            }) = self.bump()
            {
                t
            } else {
                panic!("Expected Type, got {:?}", token);
            }
        } else {
            Type::Void
        }
    }

    pub fn end_statement(&mut self) {
        match self.bump() {
            Some(Token {
                kind: TokenKind::Semicolon,
                ..
            }) => (),
            Some(Token {
                kind: TokenKind::CloseBrace,
                ..
            }) => (),
            _ => panic!("Statement not terminated"),
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Statement>> {
        let mut statements = Vec::new();
        while self.peek().is_some() {
            statements.push(Statement::parse(self)?);
        }

        Ok(statements)
    }
}
