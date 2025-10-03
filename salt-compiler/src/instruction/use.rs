use crate::{
    error::{ExpectedToken, ParseError, TypeCheckError},
    ir_generator::IrGenerator,
    lexer::{TokenKind, Type},
    parser::Parser,
    type_checker::TypeChecker,
};

#[derive(Debug, Clone)]
pub struct Use {
    function: String,
}

impl super::Instruction for Use {
    fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        parser.bump();
        let mut path = String::new();
        while let Some(token) = parser.peek() {
            match token.kind {
                TokenKind::Identifier(identifier) => path.push_str(&identifier),
                TokenKind::PathSeparator => path.push('_'),
                TokenKind::Semicolon => break,
                _ => {
                    return Err(ParseError::UnexpectedToken {
                        actual: Box::new(token),
                        expected: ExpectedToken::ImportSymbol,
                    });
                }
            }
            parser.bump();
        }
        Ok(Self { function: path })
    }

    fn check(&self, _type_checker: &mut TypeChecker) -> Result<Type, TypeCheckError> {
        Ok(Type::Void)
    }

    fn gen_ir(&self, ir_generator: &mut IrGenerator) {
        ir_generator.stash = format!("declare void @_salt_stdlib_io_vga_print(i8*)");
    }
}
