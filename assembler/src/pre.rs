//! Handles parsing and validation of programs before labels are
//! removed and replaced with relative jumps.

use crate::{Error, OpCode, Result};

use std::collections::HashMap;

#[derive(PartialEq, Debug)]
pub enum Immediate {
    Label(String),
    Value(i32),
}

#[derive(PartialEq, Debug)]
pub struct Instruction {
    pub opcode: OpCode,
    pub immediate: Immediate,
}

struct ProgramBuilder {
    labels: HashMap<String, usize>,
    instructions: Vec<Instruction>,
}

#[derive(PartialEq, Debug)]
pub struct Program {
    pub labels: HashMap<String, usize>,
    pub instructions: Vec<Instruction>,
}

pub fn parse_file(path: &str) -> Result<Program> {
    let program = std::fs::read_to_string(path)?;
    parse_string(&program)
}

pub fn parse_string(program: &str) -> Result<Program> {
    let lines = program.lines();

    let mut builder = ProgramBuilder::new();

    for (i, line) in lines.filter(|l| !l.trim().is_empty()).enumerate() {
        if line.ends_with(":") {
            if line.contains(char::is_whitespace) {
                return Err(Error::ParseError(format!(
                    "invalid label containing whitesapce: ({})",
                    line
                )));
            } else {
                builder.add_label(&line[0..line.len() - 1], i - builder.label_count());
            }
        } else {
            builder.add_instruction(parse_instruction(line)?);
        }
    }

    builder.construct()
}

fn parse_instruction(line: &str) -> Result<Instruction> {
    let mut iter = line.split_whitespace();
    let op = iter
        .next()
        .ok_or(Error::ParseError("unexpected empty line".to_string()))?;

    let op = match op {
        "pi" => OpCode::PI,
        "copy" => OpCode::COPY,
        "add" => OpCode::ADD,
        "sub" => OpCode::SUB,
        "mul" => OpCode::MUL,
        "div" => OpCode::DIV,
        "mod" => OpCode::MOD,
        "jump" => OpCode::JUMP,
        "jeq" => OpCode::JEQ,
        "jneq" => OpCode::JNEQ,
        "jlt" => OpCode::JLT,
        "jgt" => OpCode::JGT,
        "rot" => OpCode::ROT,
        "call" => OpCode::CALL,
        "ret" => OpCode::RET,
        "pop" => OpCode::POP,
        "exit" => OpCode::EXIT,
        _ => {
            return Err(Error::ParseError(format!(
                "unrecognized operation: ({})",
                op
            )))
        }
    };

    let immediate = match op {
        OpCode::PI | OpCode::ROT => {
            let imm = iter.next().ok_or(Error::ParseError(format!(
                "missing immediate for ({}) instruction",
                op
            )))?;
            Immediate::Value(imm.parse().map_err(|e| {
                Error::ParseError(format!(
                    "error parsing immediate for ({}) instruction: ({})",
                    op, e
                ))
            })?)
        }
        OpCode::JEQ | OpCode::JNEQ | OpCode::JLT | OpCode::JGT | OpCode::JUMP | OpCode::CALL => {
            let imm = iter.next().ok_or(Error::ParseError(format!(
                "missing immediate for ({}) instruction",
                op
            )))?;
            Immediate::Label(imm.to_string())
        }
        _ => Immediate::Value(0),
    };

    Ok(Instruction {
        opcode: op,
        immediate,
    })
}

impl ProgramBuilder {
    pub fn new() -> Self {
        Self {
            labels: HashMap::new(),
            instructions: Vec::new(),
        }
    }

    pub fn add_label(&mut self, name: &str, index: usize) {
        self.labels.insert(name.to_string(), index);
    }

    pub fn label_count(&self) -> usize {
        self.labels.len()
    }

    pub fn add_instruction(&mut self, i: Instruction) {
        self.instructions.push(i)
    }

    pub fn construct(self) -> Result<Program> {
        for (label, index) in &self.labels {
            if *index >= self.instructions.len() {
                return Err(Error::VerificationError(format!(
                    "label ({}) is not followed by an instruction",
                    label
                )));
            }
        }
        for instruction in &self.instructions {
            if let Immediate::Label(l) = &instruction.immediate {
                if !self.labels.contains_key(l) {
                    return Err(Error::VerificationError(format!(
                        "label ({}) is featured in an instruction but does not exist",
                        l
                    )));
                }
            }

            if instruction.opcode == OpCode::ROT {
                if let Immediate::Value(immediate) = instruction.immediate {
                    if immediate < 1 {
                        return Err(Error::VerificationError(format!(
                            "illegal instruction (rot {}): immediate is < 1 ",
                            immediate
                        )));
                    }
                }
            }
        }
        Ok(Program {
            instructions: self.instructions,
            labels: self.labels,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_label_parse() {
        let program = "
pi 3
pi 5
add
copy
pop
ret
exit
";
        let Program {
            instructions,
            labels,
        } = parse_string(program).unwrap();

        assert_eq!(labels, HashMap::new());

        assert_eq!(
            instructions,
            vec![
                Instruction {
                    opcode: OpCode::PI,
                    immediate: Immediate::Value(3)
                },
                Instruction {
                    opcode: OpCode::PI,
                    immediate: Immediate::Value(5)
                },
                Instruction {
                    opcode: OpCode::ADD,
                    immediate: Immediate::Value(0)
                },
                Instruction {
                    opcode: OpCode::COPY,
                    immediate: Immediate::Value(0)
                },
                Instruction {
                    opcode: OpCode::POP,
                    immediate: Immediate::Value(0)
                },
                Instruction {
                    opcode: OpCode::RET,
                    immediate: Immediate::Value(0)
                },
                Instruction {
                    opcode: OpCode::EXIT,
                    immediate: Immediate::Value(0)
                }
            ]
        )
    }

    #[test]
    fn test_parse() {
        let program = "
fib:
  call fib
  rot 4
  mod
  jgt bif
flib:
bif:
  div
";
        let Program {
            instructions,
            labels,
        } = parse_string(program).unwrap();

        assert_eq!(
            labels,
            vec![
                ("fib".to_string(), 0),
                ("flib".to_string(), 4),
                ("bif".to_string(), 4)
            ]
            .into_iter()
            .collect()
        );

        assert_eq!(
            instructions,
            vec![
                Instruction {
                    opcode: OpCode::CALL,
                    immediate: Immediate::Label("fib".to_string())
                },
                Instruction {
                    opcode: OpCode::ROT,
                    immediate: Immediate::Value(4)
                },
                Instruction {
                    opcode: OpCode::MOD,
                    immediate: Immediate::Value(0)
                },
                Instruction {
                    opcode: OpCode::JGT,
                    immediate: Immediate::Label("bif".to_string())
                },
                Instruction {
                    opcode: OpCode::DIV,
                    immediate: Immediate::Value(0)
                }
            ]
        )
    }
}
