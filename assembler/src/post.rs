use crate::pre;
use crate::OpCode;

#[derive(PartialEq, Debug)]
pub struct Instruction {
    pub opcode: OpCode,
    pub immediate: i32,
}

#[derive(PartialEq, Debug)]
pub struct Program {
    pub instructions: Vec<Instruction>,
}

impl pre::Program {
    pub fn unlabel(self) -> Program {
        let pre::Program {
            instructions,
            labels,
        } = self;
        Program {
            instructions: instructions
                .into_iter()
                .enumerate()
                .map(|(i, instruction)| Instruction {
                    opcode: instruction.opcode,
                    immediate: match instruction.immediate {
                        pre::Immediate::Value(v) => v as i32,
                        pre::Immediate::Label(l) => labels[&l] as i32 - i as i32,
                    },
                })
                .collect(),
        }
    }
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {}", self.opcode, self.immediate)
    }
}

impl std::fmt::Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        Ok(for inst in &self.instructions {
            writeln!(f, "{}", inst)?
        })
    }
}

impl Instruction {
    #[cfg(test)]
    fn new(opcode: OpCode, immediate: i32) -> Self {
        Self { opcode, immediate }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unlabel() {
        let pre_program = "
  pi 4
  jump fib
fib:
  copy
  pi 2
  jlt done
  copy
  pi 1
  sub
  call fib
  rot 1
  pi 2
  sub
  call fib
  add
done:
  ret
";
        let pre_program = pre::parse_string(pre_program).unwrap();
        let post_program = pre_program.unlabel();

        assert_eq!(
            post_program.instructions,
            vec![
                Instruction::new(OpCode::PI, 4),
                Instruction::new(OpCode::JUMP, 1),
                Instruction::new(OpCode::COPY, 0),
                Instruction::new(OpCode::PI, 2),
                Instruction::new(OpCode::JLT, 10),
                Instruction::new(OpCode::COPY, 0),
                Instruction::new(OpCode::PI, 1),
                Instruction::new(OpCode::SUB, 0),
                Instruction::new(OpCode::CALL, -6),
                Instruction::new(OpCode::ROT, 1),
                Instruction::new(OpCode::PI, 2),
                Instruction::new(OpCode::SUB, 0),
                Instruction::new(OpCode::CALL, -10),
                Instruction::new(OpCode::ADD, 0),
                Instruction::new(OpCode::RET, 0),
            ]
        )
    }
}
