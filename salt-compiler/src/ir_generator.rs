use crate::instruction::string_literal::StringLiteral;

pub struct IrGenerator {
    pub current_loop: Vec<usize>,
    pub strings: Vec<StringLiteral>,
    pub function_declarations: Vec<String>,
    pub ir: String,
    pub stash: String,
}

impl IrGenerator {
    pub fn new() -> Self {
        Self {
            current_loop: Vec::new(),
            strings: Vec::new(),
            function_declarations: Vec::new(),
            ir: String::new(),
            stash: String::new(),
        }
    }

    pub fn new_function(&mut self) {
        self.current_loop.push(0);
    }

    pub fn finish_function(&mut self) {
        self.current_loop.pop();
    }

    pub fn new_loop(&mut self) -> usize {
        let len = self.current_loop.len();
        self.current_loop[len - 1] += 1;
        self.current_loop[len - 1]
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
