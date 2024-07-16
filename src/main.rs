//! C23のサブセットを解釈し、x86_64、System V ABIのアセンブリを出力するコンパイラ。

mod strtol;

use std::fmt::Display;
use clap::Parser;

#[derive(Parser)]
struct Args {
    #[clap(long)]
    source: String,
}

struct Assembler;

impl Assembler {
    fn new() -> Self {
        Self
    }

    fn emit(&mut self, assembly_line: impl Display) {
        println!("{assembly_line}");
    }
}

fn main() {
    let args = Args::parse();
    let source = args.source;
    let i = source.parse::<i32>().expect("与えられたソースは数字ではありません");
    let mut assembler = Assembler::new();
    assembler.emit(".intel_syntax noprefix");
    assembler.emit(".globl main");
    assembler.emit("main:");
    assembler.emit(format!("  mov rax, {i}"));
    assembler.emit("  ret");
}
