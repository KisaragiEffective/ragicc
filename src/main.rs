//! C23のサブセットであるUTF-8で書かれたソースを解釈し、x86_64、System V ABIのアセンブリを出力するコンパイラ。
#![deny(clippy::all)]
#![warn(clippy::nursery)]

mod strtol;

use std::cell::Cell;
use std::fmt::Display;
use std::num::ParseIntError;
use std::process::ExitCode;
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

#[derive(Debug)]
struct LexerError {
    inner: LexerErrorInner,
    lexer_occurrence_position: (usize, SourcePosition),
}

#[derive(Debug)]
enum LexerErrorInner {
    InvalidInt(ParseIntError),
    InvalidByte(u8),
}

impl LexerErrorInner {
    fn message(&self) -> String {
        match self {
            Self::InvalidInt(i) => { format!("整数としてパースできません: {i}") }
            Self::InvalidByte(b) => { format!("無効なバイトです: \\x{b:2x}") }
        }
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
struct SourcePosition {
    line: usize,
    column: usize,
}

#[derive(Eq, PartialEq, Debug)]
struct Pointed<T> {
    position: SourcePosition,
    data: T,
}

struct Lexer<'a> {
    input: &'a str,
    byte_pos: usize,
    current: SourcePosition,
}

impl<'a> Lexer<'a> {
    fn new(source: &'a str) -> Self {
        Self {
            input: source,
            byte_pos: 0,
            current: SourcePosition {
                line: 1,
                column: 1,
            }
        }
    }

    fn advance_on_same_line(&mut self, n: usize) {
        self.current.column += n;
        self.byte_pos += n;
    }

    fn next(&mut self) -> Result<Pointed<Token>, LexerErrorInner> {
        let here = self.current;

        self.next_token().map(|token| Pointed {
            position: here,
            data: token,
        })
    }

    fn next_token(&mut self) -> Result<Token, LexerErrorInner> {
        let Some(mut current) = self.input.as_bytes().get(self.byte_pos).copied() else {
            return Ok(Token::EndOfFile)
        };

        while matches!(current, b' ' | b'\t') {
            self.byte_pos += 1;
            current = self.input.as_bytes()[self.byte_pos];
        }

        match current {
            b'0'..=b'9' => {
                let (parsed, rest) = strtol::str_to_fromstr::<i32>(self.rest()).map_err(LexerErrorInner::InvalidInt)?;
                self.advance_on_same_line(self.input.len() - self.byte_pos - rest.len());
                Ok(Token::LiteralInt(parsed))
            }
            b'+' => {
                self.advance_on_same_line(1);
                Ok(Token::SymPlus)
            }
            b'-' => {
                self.advance_on_same_line(1);
                Ok(Token::SymMinus)
            }
            b'\n' => {
                self.current.column = 1;
                self.current.line += 1;
                self.byte_pos += 1;
                Ok(Token::NewLine)
            }
            other => Err(LexerErrorInner::InvalidByte(other))
        }
    }

    fn rest(&self) -> &'a str {
        &self.input[self.byte_pos..]
    }
}

struct TokenStream {
    content: Vec<Pointed<Token>>,
    position: Cell<usize>,
}

impl TokenStream {
    fn peek(&self) -> Option<&Pointed<Token>> {
        self.content.get(self.position.get())
    }

    fn next(&mut self) -> Option<&Pointed<Token>> {
        let a = self.peek();
        self.position.set(self.position.get() + 1);
        a
    }
}

impl TryFrom<Lexer<'_>> for TokenStream {
    type Error = LexerError;

    fn try_from(mut value: Lexer<'_>) -> Result<Self, Self::Error> {
        let mut buffer = vec![];

        loop {
            let new = value.next().map_err(|e| LexerError {
                inner: e,
                lexer_occurrence_position: (value.byte_pos, value.current)
            })?;
            
            if new.data == Token::EndOfFile {
                break
            }

            buffer.push(new);
        }

        Ok(Self {
            content: buffer,
            position: Cell::new(0)
        })
    }
}

#[derive(Eq, PartialEq, Debug)]
enum Token {
    LiteralInt(i32),
    SymPlus,
    SymMinus,
    NewLine,
    EndOfFile,
}

fn main() -> ExitCode {
    let args = Args::parse();
    let mut tokens: TokenStream = {
        let source = args.source;
        let token_map_result = Lexer::new(&source).try_into();
        let tokens: TokenStream = match token_map_result { 
            Ok(t) => t,
            Err(e) => {
                eprintln!("{}", source);
                eprintln!("{space}^ {message}", 
                          space = " ".repeat(e.lexer_occurrence_position.0), 
                          message = e.inner.message());
                return 1.into();
            }
        };
        
        tokens
    };
    
    // TODO: convert some panics to soft error
    
    let Token::LiteralInt(i) = tokens.next().unwrap().data else {
        panic!("式の最初は数である必要があります");
    };

    let mut assembler = Assembler::new();

    assembler.emit(".intel_syntax noprefix");
    assembler.emit(".globl main");
    assembler.emit("main:");
    assembler.emit(format!("  mov rax, {i}"));

    while let Some(a) = tokens.next() {
        match &a.data {
            Token::SymPlus => {
                let Some(Pointed { data: Token::LiteralInt(next), .. }) = tokens.next() else {
                    panic!("足し算の右辺は数である必要があります")
                };
                
                assembler.emit(format!("  add rax, {next}"));
            },
            Token::SymMinus => {
                let Some(Pointed { data: Token::LiteralInt(next), .. }) = tokens.next() else {
                    panic!("引き算の右辺は数である必要があります")
                };
                
                assembler.emit(format!("  sub rax, {next}"));
            },
            other => {
                panic!("予期しないトークンです: {other:?}");
            }
        }
    }

    assembler.emit("  ret");
    
    0.into()
}
