use crate::parser::Parser;

pub mod kernel_function;

pub trait Instruction {
    fn parse(parser: &mut Parser) -> Self;
    fn check(&self);
    fn gen_ir(&self) -> String;
}
