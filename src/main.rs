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
    let (i, mut rest) = strtol::str_to_fromstr::<i32>(&source).expect("与えられたソースは数字ではありません");
    let mut assembler = Assembler::new();

    assembler.emit(".intel_syntax noprefix");
    assembler.emit(".globl main");
    assembler.emit("main:");
    assembler.emit(format!("  mov rax, {i}"));

    if !rest.is_empty() {
        while !rest.is_empty() {
            eprintln!("{rest}");
            let h = rest.as_bytes()[0];
            match h {
                b'+' => {
                    let next;
                    (next, rest) = strtol::str_to_fromstr::<i32>(&rest[1..]).expect("予期しない文字が出現しました");
                    assembler.emit(format!("  add rax, {next}"));
                },
                b'-' => {
                    let next;
                    (next, rest) = strtol::str_to_fromstr::<i32>(&rest[1..]).expect("予期しない文字が出現しました");
                    assembler.emit(format!("  sub rax, {next}"));
                },
                other => {
                    panic!("予期しない文字です: \\x{other:2x}");
                }
            }
        }
    }

    assembler.emit("  ret");
}
