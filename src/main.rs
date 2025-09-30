use std::fs::File;
use std::io::{Read, Write};
use std::process::Command;

fn main() {
    create_bootloader();
    build_minimal_llvm_ir_kernel();
    assemble_bootloader();
    compile_llvm_ir();
    let bootloader = read_bootloader();
    let kernel = read_kernel();
    create_image(bootloader, kernel)
}

fn create_bootloader() {
    let asm = r#"
[BITS 16]
ORG 0x7c00

start:
    cli
    xor ax, ax
    mov ds, ax
    mov es, ax
    mov ss, ax
    mov sp, 0x700

    mov ah, 0x00,
    mov al, 0x03,
    int 0x10,

    jmp 0x0000:0x1000

times 510 - (& - &&) db 0

db 0x55
db 0xAA
    "#;

    File::create("bootloader.asm")
        .unwrap()
        .write_all(asm.as_bytes())
        .unwrap();
}

fn build_minimal_llvm_ir_kernel() {
    let ir = r#"
; -------------------------
; Globals
; -------------------------
@vga = global i16* inttoptr (i32 753664 to i16*)
    

; -------------------------
; Kernel entry
; -------------------------
define void @main() {
entry:
    ; -------------------------
    ; clear_screen() → fill VGA with spaces (0x0F20)
    ; -------------------------
    %vga_ptr = getelementptr [2000 x i16], [2000 x i16]* @vga, i32 0, i32 0
    %i = alloca i32
    store i32 0, i32* %i
    ; -------------------------
    ; typed VGA writes
    ; -------------------------
    ; vga[0].value = (0x0F << 8) | 'H'
    %vga0_ptr = getelementptr [2000 x i16], [2000 x i16]* @vga, i32 0, i32 0
    store volatile i16 3928, i16* %vga0_ptr  ; 0x0F << 8 | 0x48 ('H')

    ; vga[1].value = (0x0F << 8) | 'i'
    %vga1_ptr = getelementptr [2000 x i16], [2000 x i16]* @vga, i32 0, i32 1
    store volatile i16 3945, i16* %vga1_ptr  ; 0x0F << 8 | 0x69 ('i')

    ; (~vga[10]).write16((0x0F << 8) | '!')
    %vga10_ptr = getelementptr [2000 x i16], [2000 x i16]* @vga, i32 0, i32 10
    store volatile i16 3873, i16* %vga10_ptr  ; 0x0F << 8 | 0x21 ('!')

    call void asm sideeffect "hlt", ""()

    ret void
}
"#;

    File::create("kernel.ll")
        .unwrap()
        .write_all(ir.as_bytes())
        .unwrap();
}

fn assemble_bootloader() {
    Command::new("nasm")
        .args(["-f", "bin", "bootloader.asm", "-o", "bootloader.bin"])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}

fn compile_llvm_ir() {
    Command::new("llc")
        .args(["--filetype=obj", "kernel.ll", "-o", "kernel.o"])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
    Command::new("ld")
        .args([
            "--Ttext=0x1000",
            "--oformat=binary",
            "kernel.o",
            "-o",
            "kernel.bin",
            "--entry=main",
        ])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}

fn read_bootloader() -> Vec<u8> {
    let mut buf = Vec::new();
    File::open("bootloader.bin")
        .unwrap()
        .read_to_end(&mut buf)
        .unwrap();
    buf
}

fn read_kernel() -> Vec<u8> {
    let mut buf = Vec::new();
    File::open("kernel.bin")
        .unwrap()
        .read_to_end(&mut buf)
        .unwrap();
    buf
}

fn create_image(mut boot_loader: Vec<u8>, kernel: Vec<u8>) {
    boot_loader.extend(&kernel);
    File::create("saltos.bin")
        .unwrap()
        .write_all(&boot_loader)
        .unwrap();
}
