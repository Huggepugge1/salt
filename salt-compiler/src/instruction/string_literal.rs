use crate::{
    error::{ParseError, TypeCheckError},
    ir_generator::IrGenerator,
    lexer::{TokenKind, Type},
    parser::Parser,
    type_checker::TypeChecker,
};

#[derive(Debug, Clone)]
pub struct StringLiteral {
    value: String,
}

impl super::Instruction for StringLiteral {
    fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        if let TokenKind::StringLiteral(value) = parser.bump().unwrap().kind {
            Ok(Self { value })
        } else {
            unreachable!()
        }
    }

    fn check(&self, _type_checker: &mut TypeChecker) -> Result<Type, TypeCheckError> {
        Ok(Type::Str)
    }

    fn gen_ir(&self, ir_generator: &mut IrGenerator) {
        let str_num = ir_generator.strings.len();
        ir_generator.strings.push(self.clone());
        ir_generator.stash = format!(
            "getelementptr [{} x i8], [{} x i8]* @.str.{}, i64 0, i64 0",
            self.value.len() + 1,
            self.value.len() + 1,
            str_num
        );
    }
}

impl StringLiteral {
    pub fn to_ir(&self, str_num: usize) -> String {
        format!(
            "@.str.{} = internal constant [{} x i8] c\"{}\\00\"",
            str_num,
            self.value.len() + 1,
            self.value
        )
    }
}
