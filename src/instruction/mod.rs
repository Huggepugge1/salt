use crate::{ir_generator::IrGenerator, parser::Parser};

pub mod block;
pub mod kernel_function;
pub mod r#loop;

pub trait Instruction {
    fn parse(parser: &mut Parser) -> Self;
    fn check(&self);
    fn gen_ir(&self, ir_generator: &mut IrGenerator) -> String;
}
