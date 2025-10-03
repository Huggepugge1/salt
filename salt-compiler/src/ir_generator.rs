use std::collections::HashMap;

use crate::instruction::{
    function_declaration::FunctionDeclaration, string_literal::StringLiteral,
};

pub struct IrGenerator {
    pub current_loop: Vec<usize>,
    pub values: Vec<usize>,

    pub strings: Vec<StringLiteral>,
    pub function_declarations: Vec<String>,
    pub ir: String,
    pub stash: String,

    pub env: HashMap<String, FunctionDeclaration>,
}

impl IrGenerator {
    pub fn new(env: HashMap<String, FunctionDeclaration>) -> Self {
        Self {
            current_loop: Vec::new(),
            values: Vec::new(),

            strings: Vec::new(),
            function_declarations: Vec::new(),
            ir: String::new(),
            stash: String::new(),

            env,
        }
    }

    pub fn new_function(&mut self) {
        self.current_loop.push(0);
        self.values.push(0);
    }

    pub fn finish_function(&mut self) {
        self.current_loop.pop();
        self.values.pop();
    }

    pub fn new_loop(&mut self) -> usize {
        *self.current_loop.last_mut().unwrap() += 1;
        self.current_loop.last().unwrap() - 1
    }

    pub fn new_value(&mut self) -> usize {
        *self.values.last_mut().unwrap() += 1;
        self.values.last().unwrap() - 1
    }

    pub fn pop_stash(&mut self) -> String {
        std::mem::take(&mut self.stash)
    }

    pub fn get_ir(&mut self) -> String {
        self.strings
            .iter()
            .enumerate()
            .map(|(i, e)| e.to_ir(i) + "\n")
            .collect::<String>()
            + "\n"
            + &self.function_declarations.join("\n")
            + &self.ir
            + &self.stash
    }
}
