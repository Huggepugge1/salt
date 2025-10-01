use crate::{ir_generator::IrGenerator, parser::Parser, type_checker::TypeChecker};

use anyhow::Result;

pub mod block;
pub mod function_declaration;
pub mod intrinsic;
pub mod r#loop;

pub trait Instruction
where
    Self: std::marker::Sized,
{
    fn parse(parser: &mut Parser) -> Result<Self>;
    fn check(&self, type_checker: &mut TypeChecker);
    fn gen_ir(&self, ir_generator: &mut IrGenerator) -> String;
}
