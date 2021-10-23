pub mod assemble;
pub mod post;
pub mod pre;

use thiserror::Error;

#[derive(PartialEq, Debug)]
#[repr(u8)]
pub enum OpCode {
    PI = 1,
    COPY,
    ADD,
    SUB,
    MUL,
    DIV,
    MOD,
    JUMP,
    JEQ,
    JNEQ,
    JLT,
    JGT,
    ROT,
    CALL,
    RET,
    POP,
    EXIT,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("i/o error: ({0})")]
    IoError(#[from] std::io::Error),
    #[error("parse error: ({0})")]
    ParseError(String),
    #[error("verification error: ({0})")]
    VerificationError(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl std::fmt::Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                OpCode::PI => "pi",
                OpCode::COPY => "copy",
                OpCode::ADD => "add",
                OpCode::SUB => "sub",
                OpCode::MUL => "mul",
                OpCode::DIV => "div",
                OpCode::MOD => "mod",
                OpCode::JUMP => "jump",
                OpCode::JEQ => "jeq",
                OpCode::JNEQ => "jneq",
                OpCode::JLT => "jlt",
                OpCode::JGT => "jgt",
                OpCode::ROT => "rot",
                OpCode::CALL => "call",
                OpCode::RET => "ret",
                OpCode::POP => "pop",
                OpCode::EXIT => "exit",
            }
        )
    }
}
