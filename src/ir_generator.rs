pub struct IrGenerator {
    pub current_loop: Vec<usize>,
}

impl IrGenerator {
    pub fn new() -> Self {
        Self {
            current_loop: Vec::new(),
        }
    }

    pub fn new_func(&mut self) {
        self.current_loop.push(0);
    }

    pub fn finish_func(&mut self) {
        self.current_loop.push(0);
    }

    pub fn new_loop(&mut self) -> usize {
        let len = self.current_loop.len();
        self.current_loop[len - 1] += 1;
        self.current_loop[len - 1]
    }
}
