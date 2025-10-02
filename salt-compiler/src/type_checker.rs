pub struct TypeChecker {
    in_raw_function: Vec<bool>,
}

impl TypeChecker {
    pub fn new() -> Self {
        Self {
            in_raw_function: Vec::new(),
        }
    }

    pub fn new_function(&mut self, raw: bool) {
        self.in_raw_function.push(raw);
    }

    pub fn finish_function(&mut self) {
        self.in_raw_function.pop();
    }

    pub fn in_raw_function(&self) -> bool {
        *self.in_raw_function.last().unwrap()
    }
}
