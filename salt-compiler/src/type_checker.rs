use std::collections::HashMap;

use crate::{
    instruction::{block::Block, function_declaration::FunctionDeclaration},
    parser::{Statement, StatementKind},
};

pub struct TypeChecker {
    pub functions: HashMap<String, FunctionDeclaration>,
    in_raw_function: Vec<bool>,
}

impl TypeChecker {
    pub fn new(ast: &Statement) -> Self {
        let mut type_checker = Self {
            functions: HashMap::new(),
            in_raw_function: Vec::new(),
        };

        type_checker.build_symbol_table(ast);
        type_checker
    }

    pub fn build_symbol_table(&mut self, ast: &Statement) {
        match &ast.kind {
            StatementKind::Module { ast, .. } => {
                for statement in ast {
                    if let StatementKind::FunctionDeclaration(function_declaration) =
                        &statement.kind
                    {
                        self.functions.insert(
                            function_declaration.name.clone(),
                            function_declaration.clone(),
                        );
                    }
                }
            }
            StatementKind::Block(Block { body, .. }) => {
                for statement in body {
                    if let StatementKind::FunctionDeclaration(function_declaration) =
                        &statement.kind
                    {
                        self.functions.insert(
                            function_declaration.name.clone(),
                            function_declaration.clone(),
                        );
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    pub fn new_function(&mut self, function: &FunctionDeclaration) {
        self.in_raw_function.push(function.raw);
    }

    pub fn finish_function(&mut self) {
        self.in_raw_function.pop();
    }

    pub fn get_function(&self, name: &str) -> Option<&FunctionDeclaration> {
        if let Some(function) = self.functions.get(name) {
            return Some(function);
        }
        None
    }

    pub fn in_raw_function(&self) -> bool {
        *self.in_raw_function.last().unwrap()
    }
}
