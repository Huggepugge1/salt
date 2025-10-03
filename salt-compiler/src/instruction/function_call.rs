use crate::{
    error::{ParseError, TypeCheckError},
    ir_generator::IrGenerator,
    lexer::{TokenKind, Type},
    parser::Parser,
    type_checker::TypeChecker,
};

#[derive(Debug, Clone)]
pub struct FunctionCall {
    name: String,
}

impl super::Instruction for FunctionCall {
    fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        let name = parser.expect_ident()?;
        parser.expect(&TokenKind::OpenParen)?;
        parser.expect(&TokenKind::CloseParen)?;
        Ok(Self { name })
    }

    fn check(&self, type_checker: &mut TypeChecker) -> Result<Type, TypeCheckError> {
        if let Some(function) = type_checker.functions.get(&self.name) {
            Ok(function.return_type)
        } else {
            Err(TypeCheckError::UndeclaredFunctionNoToken)
        }
    }

    fn gen_ir(&self, ir_generator: &mut IrGenerator) {
        let return_type = ir_generator.env.get(&self.name).unwrap().return_type;
        ir_generator.stash = format!("call {} @{}()", return_type.to_ir(), self.name)
    }
}
