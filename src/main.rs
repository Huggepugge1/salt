use logos::Logos;

use std::{
    fs::File,
    io::{Read, Write},
};

mod compile;
mod instruction;
mod lexer;
mod parser;

fn main() {
    let mut source = Vec::new();
    let _read = File::open("./salt_code/main.salt")
        .unwrap()
        .read_to_end(&mut source)
        .unwrap();

    let tokens = lexer::Token::lexer(&String::from_utf8_lossy(&source))
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    let ast = parser::Parser::new(tokens).parse();
    for statement in &ast {
        statement.check();
    }
    let mut ir = String::new();
    for statement in &ast {
        ir.push_str(&statement.gen_ir());
    }
    File::create("kernel.ll")
        .unwrap()
        .write_all(ir.as_bytes())
        .unwrap();

    // build_minimal_llvm_ir_kernel();
    compile::compile();
}

#[allow(dead_code)]
fn build_minimal_llvm_ir_kernel() {
    let ir = r#"
define void @main() {
entry:
    %vga = inttoptr i64 u0xB8000 to i8*
    store i8 u0x48, i8* %vga       ; 'H'
    %attr = getelementptr i8, i8* %vga, i64 1
    store i8 u0x0F, i8* %attr       ; attribute

    %vga1 = getelementptr i8, i8* %vga, i64 2
    store i8 u0x69, i8* %vga1       ; 'i'
    %attr1 = getelementptr i8, i8* %vga, i64 3
    store i8 u0x0F, i8* %attr1

    %vga2 = getelementptr i8, i8* %vga, i64 4
    store i8 u0x21, i8* %vga2       ; '!'
    %attr2 = getelementptr i8, i8* %vga, i64 5
    store i8 u0x0F, i8* %attr2

    br label %halt_loop

halt_loop:
  call void asm sideeffect "hlt", ""()
  br label %halt_loop
  
  ret void
}
"#;

    File::create("kernel.ll")
        .unwrap()
        .write_all(ir.as_bytes())
        .unwrap();
}
