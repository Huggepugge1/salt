use anyhow::Result;

use crate::{
    ir_generator::IrGenerator,
    lexer::TokenKind,
    parser::{Parser, Statement},
    type_checker::TypeChecker,
};

#[derive(Debug)]
pub struct Block {
    body: Vec<Statement>,
}

impl super::Instruction for Block {
    fn parse(parser: &mut Parser) -> Result<Self> {
        parser.bump();
        let mut body = Vec::new();
        while let Some(token) = parser.peek()
            && token.kind != TokenKind::CloseBrace
        {
            body.push(Statement::parse(parser)?);
        }

        parser.expect_without_increment(&TokenKind::CloseBrace)?;

        Ok(Self { body })
    }

    fn check(&self, type_checker: &mut TypeChecker) {
        for statement in &self.body {
            statement.check(type_checker);
        }
    }

    fn gen_ir(&self, ir_generator: &mut IrGenerator) -> String {
        let mut ir = String::new();

        for statement in &self.body {
            ir.push_str(&statement.gen_ir(ir_generator));
            ir.push('\n');
        }

        ir
    }
}
