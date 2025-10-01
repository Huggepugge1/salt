use crate::{
    ir_generator::IrGenerator,
    lexer::{Token, Type},
    parser::{Parser, Statement},
};

#[derive(Debug)]
pub struct KernelFunction {
    name: String,
    body: Box<Statement>,
    return_type: Type,
}

impl super::Instruction for KernelFunction {
    fn parse(parser: &mut Parser) -> Self {
        parser.bump();
        let name = parser.expect_ident();
        parser.expect(&Token::OpenParen);
        parser.expect(&Token::CloseParen);
        let return_type = parser.expect_optional_type();

        let body = Box::new(Statement::parse(parser));

        Self {
            name,
            body,
            return_type,
        }
    }

    fn check(&self) {
        self.body.check();
    }

    fn gen_ir(&self, ir_generator: &mut IrGenerator) -> String {
        ir_generator.new_func();
        let mut ir = String::new();
        ir.push_str(&format!("define void @{}() {{\nentry:\n", self.name));

        ir.push_str(&self.body.gen_ir(ir_generator));

        if self.return_type == Type::Void {
            ir.push_str("  ret void\n");
        }
        ir.push('}');
        ir.push('\n');
        ir.push('\n');

        ir_generator.finish_func();

        ir.clone()
    }
}
