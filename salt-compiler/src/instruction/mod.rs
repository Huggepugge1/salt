use crate::{
    error::{ParseError, TypeCheckError},
    ir_generator::IrGenerator,
    lexer::Type,
    parser::Parser,
    type_checker::TypeChecker,
};

pub mod block;
pub mod function_call;
pub mod function_declaration;
pub mod intrinsic;
pub mod r#loop;

pub trait Instruction
where
    Self: std::marker::Sized,
{
    fn parse(parser: &mut Parser) -> Result<Self, ParseError>;
    fn check(&self, type_checker: &mut TypeChecker) -> Result<Type, TypeCheckError>;
    fn gen_ir(&self, ir_generator: &mut IrGenerator) -> String;
}
