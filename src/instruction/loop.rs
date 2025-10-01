use anyhow::Result;

use crate::{
    ir_generator::IrGenerator,
    parser::{Parser, Statement},
    type_checker::TypeChecker,
};

#[derive(Debug)]
pub struct Loop {
    body: Box<Statement>,
}

impl super::Instruction for Loop {
    fn parse(parser: &mut Parser) -> Result<Self> {
        parser.bump();
        let body = Box::new(Statement::parse(parser)?);
        Ok(Self { body })
    }

    fn check(&self, type_checker: &mut TypeChecker) {
        self.body.check(type_checker);
    }

    fn gen_ir(&self, ir_generator: &mut IrGenerator) -> String {
        ir_generator.new_function();
        let mut ir = String::new();
        let loop_nr = ir_generator.new_loop();
        ir.push_str(&format!("br label %loop{}\n", loop_nr));
        ir.push_str(&format!("loop{}:\n", loop_nr));

        ir.push_str(&self.body.gen_ir(ir_generator));
        ir.push_str(&format!("br label %loop{}\n", loop_nr));
        ir.push('\n');

        ir_generator.finish_function();

        ir
    }
}
