use crate::{
    error::{ParseError, TypeCheckError},
    ir_generator::IrGenerator,
    lexer::{Token, TokenKind, Type},
    parser::Parser,
    type_checker::TypeChecker,
};

#[derive(Debug, Clone)]
pub enum Intrinsic {
    Hlt,
}

impl super::Instruction for Intrinsic {
    fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        let symbol = parser.bump();
        let intrinsic = if let Some(Token {
            kind: TokenKind::Intrinsic(symbol),
            ..
        }) = symbol
        {
            match symbol.as_str() {
                "@hlt" => Self::Hlt,
                _ => unreachable!(),
            }
        } else {
            unreachable!();
        };
        parser.expect(&TokenKind::OpenParen)?;
        parser.expect(&TokenKind::CloseParen)?;
        Ok(intrinsic)
    }

    fn check(&self, type_checker: &mut TypeChecker) -> Result<Type, TypeCheckError> {
        match self {
            Intrinsic::Hlt => {
                if !type_checker.in_raw_function() {
                    Err(TypeCheckError::UnsafeUseNoToken)
                } else {
                    Ok(Type::Void)
                }
            }
        }
    }

    fn gen_ir(&self, _ir_generator: &mut IrGenerator) -> String {
        match self {
            Intrinsic::Hlt => String::from("call void asm \"hlt\", \"\"()"),
        }
    }
}
