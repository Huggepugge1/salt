use anyhow::Result;

use crate::{
    ir_generator::IrGenerator,
    lexer::{Token, TokenKind},
    parser::Parser,
    type_checker::TypeChecker,
};

#[derive(Debug)]
pub enum Intrinsic {
    Hlt,
}

impl super::Instruction for Intrinsic {
    fn parse(parser: &mut Parser) -> Result<Self> {
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

    fn check(&self, type_checker: &mut TypeChecker) {
        match self {
            Intrinsic::Hlt => {
                if !type_checker.in_raw_function() {
                    panic!("@hlt() used in a safe function");
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
