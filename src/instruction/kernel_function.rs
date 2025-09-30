use crate::{
    lexer::{Token, Type},
    parser::{Parser, Statement},
};

#[derive(Debug)]
pub struct KernelFunction {
    name: String,
    body: Vec<Statement>,
    return_type: Type,
}

impl Parser {
    pub fn parse_kernel_function(&mut self) -> Statement {
        self.bump();
        let name = self.expect_ident();
        self.expect(&Token::OpenParen);
        self.expect(&Token::CloseParen);
        let return_type = self.expect_optional_type();
        self.expect(&Token::OpenBrace);

        let body = Vec::new();

        self.expect(&Token::CloseBrace);

        Statement::KernelFunction(KernelFunction {
            name,
            body,
            return_type,
        })
    }
}

impl super::Instruction for KernelFunction {
    fn parse(parser: &mut Parser) -> Self {
        parser.bump();
        let name = parser.expect_ident();
        parser.expect(&Token::OpenParen);
        parser.expect(&Token::CloseParen);
        let return_type = parser.expect_optional_type();
        parser.expect(&Token::OpenBrace);

        let body = Vec::new();

        parser.expect(&Token::CloseBrace);

        Self {
            name,
            body,
            return_type,
        }
    }

    fn check(&self) {
        for statement in &self.body {
            statement.check();
        }
    }

    fn gen_ir(&self) -> String {
        let mut ir = String::new();
        ir.push_str(&format!("define void @{}() {{\nentry:\n", self.name));

        for statement in &self.body {
            ir.push_str("  ");
            ir.push_str(&statement.gen_ir());
            ir.push('\n');
        }

        if self.return_type == Type::Void {
            ir.push_str("  ret void\n");
        }
        ir.push('}');
        ir.push('\n');
        ir.push('\n');

        ir.clone()
    }
}
