use std::collections::HashMap;

use crate::{lexer::Type, parser::Statement};

pub struct TypeChecker<'a> {
    ast: &'a Vec<Statement>,
}

impl<'a> TypeChecker<'a> {
    pub fn new(ast: &'a Vec<Statement>) -> Self {
        Self { ast }
    }
}
