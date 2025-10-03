use anyhow::Result;

use crate::{
    error::{ParseError, TypeCheckError},
    ir_generator::IrGenerator,
    lexer::{TokenKind, Type},
    parser::{Parser, Statement},
    type_checker::TypeChecker,
};

#[derive(Debug, Clone)]
pub struct Block {
    pub body: Vec<Statement>,
    returns: bool,
}

impl super::Instruction for Block {
    fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        parser.bump();
        let mut body = Vec::new();
        while let Some(token) = parser.peek()
            && token.kind != TokenKind::CloseBrace
        {
            body.push(Statement::parse(parser)?);
        }

        let returns = parser.block_returns();

        Ok(Self { body, returns })
    }

    fn check(&self, type_checker: &mut TypeChecker) -> Result<Type, TypeCheckError> {
        let mut last_type = Type::Any;
        for statement in &self.body {
            last_type = statement.check(type_checker)?;
        }
        if self.returns {
            Ok(last_type)
        } else {
            Ok(Type::Void)
        }
    }

    fn gen_ir(&self, ir_generator: &mut IrGenerator) {
        let mut ir = String::new();
        for statement in &self.body {
            statement.gen_ir(ir_generator);
            ir.push_str(&ir_generator.pop_stash());
            ir.push('\n');
        }

        ir_generator.stash = ir
            .split("\n")
            .map(|e| String::from("  ") + e)
            .collect::<Vec<_>>()
            .join("\n")
            + "\n";
    }
}
