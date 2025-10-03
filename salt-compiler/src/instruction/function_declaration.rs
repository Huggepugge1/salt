use crate::{
    error::{ParseError, TypeCheckError},
    ir_generator::IrGenerator,
    lexer::{Keyword, Token, TokenKind, Type},
    parser::{Parser, Statement, StatementKind},
    type_checker::TypeChecker,
};

#[derive(Debug, Clone)]
pub struct FunctionDeclaration {
    pub name: String,
    pub body: Box<Statement>,
    pub return_type: Type,

    pub raw: bool,
}

impl super::Instruction for FunctionDeclaration {
    fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
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
        let return_type = parser.expect_optional_type()?;

        let body = Box::new(Statement::parse(parser)?);

        Ok(Self {
            name,
            body,
            return_type,

            raw,
        })
    }

    fn check(&self, type_checker: &mut TypeChecker) -> Result<Type, TypeCheckError> {
        type_checker.new_function(self);
        let mut t = self.body.check(type_checker)?;
        t = match self.body.returns {
            true => t,
            false => Type::Void,
        };
        type_checker.finish_function();
        if self.return_type == t {
            Ok(t)
        } else {
            Err(TypeCheckError::MismatchedType {
                location: self.body.last().location.clone(),
                expected: self.return_type,
                actual: t,
            })
        }
    }

    fn gen_ir(&self, ir_generator: &mut IrGenerator) {
        ir_generator.new_function();
        let mut ir = String::new();
        ir.push_str(&format!(
            "define {} @{}() {{\nentry:\n",
            self.return_type.to_ir(),
            self.name
        ));

        self.body.gen_ir(ir_generator);
        match self.body.kind {
            StatementKind::FunctionDeclaration(_) => (),
            _ => {
                ir.push_str(&ir_generator.pop_stash());
            }
        }

        if self.body.last().returns {
            ir.push_str(&format!(
                "  ret {} %{}\n",
                self.return_type.to_ir(),
                ir_generator.values.last().unwrap() - 1
            ))
        } else if self.return_type == Type::Void {
            ir.push_str("  ret void\n");
        }
        ir.push('}');
        ir.push('\n');

        ir_generator.finish_function();
        ir_generator.function_declarations.push(ir);
    }
}
