use crate::{
    instruction::{Instruction, block::Block, kernel_function::KernelFunction, r#loop::Loop},
    ir_generator::IrGenerator,
    lexer::{Keyword, Token, Type},
};

#[derive(Debug)]
pub enum Statement {
    KernelFunction(KernelFunction),

    Loop(Loop),

    Block(Block),
}

impl Instruction for Statement {
    fn parse(parser: &mut Parser) -> Self {
        if let Some(token) = parser.peek() {
            match token {
                Token::Keyword(Keyword::Kernel) => {
                    Statement::KernelFunction(KernelFunction::parse(parser))
                }
                Token::Keyword(Keyword::Loop) => Statement::Loop(Loop::parse(parser)),
                Token::Type(_) => panic!(),
                Token::Ident(_) => panic!(),
                Token::ReturnType => panic!(),
                Token::OpenParen => panic!(),
                Token::CloseParen => panic!(),
                Token::OpenBrace => Statement::Block(Block::parse(parser)),
                Token::CloseBrace => panic!(),
            }
        } else {
            panic!("Preliminary EOF");
        }
    }

    fn check(&self) {
        match self {
            Statement::KernelFunction(kernel_function) => kernel_function.check(),
            Statement::Loop(r#loop) => r#loop.check(),
            Statement::Block(block) => block.check(),
        }
    }

    fn gen_ir(&self, ir_generator: &mut IrGenerator) -> String {
        match self {
            Statement::KernelFunction(kernel_function) => kernel_function.gen_ir(ir_generator),
            Statement::Loop(r#loop) => r#loop.gen_ir(ir_generator),
            Statement::Block(block) => block.gen_ir(ir_generator),
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
        while let Some(_) = self.peek() {
            statements.push(Statement::parse(self));
        }

        statements
    }
}
