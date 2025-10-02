use anyhow::Result;

use crate::{
    ir_generator::IrGenerator,
    lexer::{Keyword, Token, TokenKind, Type},
    parser::{Parser, Statement},
    type_checker::TypeChecker,
};

#[derive(Debug)]
pub struct FunctionDeclaration {
    name: String,
    body: Box<Statement>,
    return_type: Type,

    raw: bool,
}

impl super::Instruction for FunctionDeclaration {
    fn parse(parser: &mut Parser) -> Result<Self> {
        let raw = match parser.bump() {
            Some(Token {
                kind: TokenKind::Keyword(Keyword::Raw),
                ..
            }) => {
                parser.expect(&TokenKind::Keyword(Keyword::Fn))?;
                true
            }
            _ => false,
        };
        let name = parser.expect_ident()?;
        parser.expect(&TokenKind::OpenParen)?;
        parser.expect(&TokenKind::CloseParen)?;
        let return_type = parser.expect_optional_type();

        let body = Box::new(Statement::parse(parser)?);

        Ok(Self {
            name,
            body,
            return_type,

            raw,
        })
    }

    fn check(&self, type_checker: &mut TypeChecker) {
        type_checker.new_function(self.raw);
        self.body.check(type_checker);
        type_checker.finish_function();
    }

    fn gen_ir(&self, ir_generator: &mut IrGenerator) -> String {
        ir_generator.new_function();
        let mut ir = String::new();
        ir.push_str(&format!("define void @{}() {{\nentry:\n", self.name));

        ir.push_str(&self.body.gen_ir(ir_generator));

        if self.return_type == Type::Void {
            ir.push_str("  ret void\n");
        }
        ir.push('}');
        ir.push('\n');
        ir.push('\n');

        ir_generator.finish_function();

        ir
    }
}
