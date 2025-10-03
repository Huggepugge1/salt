use std::fs::remove_file;
use std::process::Command;

pub fn compile() {
    compile_llvm_ir();
    assemble_nasm();
    link_elf();
    make_iso();
    clean_up();
    run_qemu();
}

pub fn compile_llvm_ir() {
    Command::new("llc")
        .args(["--filetype=obj", "kernel.ll", "-o", "kernel.o"])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}

fn assemble_nasm() {
    Command::new("nasm")
        .args(["-f", "elf64", "-o", "multiboot.o", "multiboot.asm"])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}

fn link_elf() {
    Command::new("ld")
        .args([
            "-T",
            "linker.ld",
            "../salt-stdlib/target/release",
            "-o",
            "iso/boot/kernel.elf",
        ])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}
fn make_iso() {
    Command::new("grub-mkrescue")
        .args(["-o", "saltos.iso", "iso"])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}

fn clean_up() {
    remove_file("./kernel.ll").unwrap();
    remove_file("./kernel.o").unwrap();
    remove_file("./multiboot.o").unwrap();
}

fn run_qemu() {
    Command::new("qemu-system-x86_64")
        .args(["-cdrom", "saltos.iso"])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}
