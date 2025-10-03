use crate::{
    error::{ParseError, TypeCheckError},
    ir_generator::IrGenerator,
    lexer::Type,
    parser::{Parser, Statement},
    type_checker::TypeChecker,
};

#[derive(Debug, Clone)]
pub struct Loop {
    pub body: Box<Statement>,
}

impl super::Instruction for Loop {
    fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        parser.bump();
        let body = Box::new(Statement::parse(parser)?);
        Ok(Self { body })
    }

    fn check(&self, type_checker: &mut TypeChecker) -> Result<Type, TypeCheckError> {
        self.body.check(type_checker)
    }

    fn gen_ir(&self, ir_generator: &mut IrGenerator) {
        let mut ir = String::new();
        let loop_nr = ir_generator.new_loop();
        ir.push_str(&format!("br label %loop{}\n", loop_nr));
        ir.push_str(&format!("loop{}:\n", loop_nr));
        self.body.gen_ir(ir_generator);
        ir.push_str(&ir_generator.pop_stash());
        ir.push_str(&format!("br label %loop{}\n", loop_nr));
        ir.push_str(&format!("loop_exit{}:", loop_nr));

        ir_generator.stash = ir;
    }
}
