use crate::{
    instruction::{Instruction, kernel_function::KernelFunction},
    lexer::{Keyword, Token, Type},
};

#[derive(Debug)]
pub enum Statement {
    KernelFunction(KernelFunction),
    // Let { name: String, typ: String },
}

impl Statement {
    pub fn check(&self) {
        match self {
            Statement::KernelFunction(kernel_function) => kernel_function.check(),
            _ => panic!("Expected KernelFunc"),
        }
    }

    pub fn gen_ir(&self) -> String {
        match self {
            Statement::KernelFunction(kernel_function) => kernel_function.gen_ir(),
            _ => panic!("Expected KernelFunc"),
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
        let t = self.peek();
        self.pos += 1;
        t
    }

    pub fn expect(&mut self, expected: &Token) {
        if let Some(token) = self.bump()
            && token != *expected
        {
            panic!("Expected {:?}, got {:?}", expected, token);
        }
    }

    pub fn expect_ident(&mut self) -> String {
        let token = self.bump();
        if let Some(Token::Ident(name)) = token {
            name.clone()
        } else if let Some(token) = token {
            panic!("Expected Identifier, got {:?}", token);
        } else {
            panic!("Reached EOF!");
        }
    }

    pub fn expect_optional_type(&mut self) -> Type {
        let token = self.peek();
        if let Some(Token::ReturnType) = token {
            self.bump();
            if let Some(Token::Type(t)) = self.bump() {
                t
            } else {
                panic!("Expected Type, got {:?}", token);
            }
        } else {
            Type::Void
        }
    }

    pub fn parse(&mut self) -> Vec<Statement> {
        let mut statements = Vec::new();
        while let Some(token) = self.peek() {
            match token {
                Token::Keyword(Keyword::Kernel) => {
                    statements.push(Statement::KernelFunction(KernelFunction::parse(self)))
                }
                other => panic!("Unexpected token: {:?}", other),
            }
        }

        statements
    }
}
