use crate::{
    ir_generator::IrGenerator,
    lexer::Token,
    parser::{Parser, Statement},
};

#[derive(Debug)]
pub struct Block {
    body: Vec<Statement>,
}

impl super::Instruction for Block {
    fn parse(parser: &mut Parser) -> Self {
        parser.bump();
        let mut body = Vec::new();
        while let Some(token) = parser.peek()
            && token != Token::CloseBrace
        {
            body.push(Statement::parse(parser));
        }

        parser.expect(&Token::CloseBrace);

        Self { body }
    }

    fn check(&self) {
        for statement in &self.body {
            statement.check();
        }
    }

    fn gen_ir(&self, ir_generator: &mut IrGenerator) -> String {
        let mut ir = String::new();

        for statement in &self.body {
            ir.push_str(&statement.gen_ir(ir_generator));
            ir.push('\n');
        }

        ir.clone()
    }
}
