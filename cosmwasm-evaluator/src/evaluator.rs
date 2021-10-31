struct InterpreterState {
    pc: u32,
    call_stack: Vec<u32>,
    stack: Vec<i32>,
}

pub fn eval(bytecode: &[u8]) -> i32 {
    let mut state = InterpreterState {
        pc: 0,
        call_stack: vec![],
        stack: vec![],
    };
    let res = loop {
        let inst_start = (state.pc * 5) as usize;
        let opcode = bytecode[inst_start];
        let immediate = i32::from_le_bytes([
            bytecode[inst_start + 1],
            bytecode[inst_start + 2],
            bytecode[inst_start + 3],
            bytecode[inst_start + 4],
        ]);
        match opcode {
            1 => state.stack.push(immediate),
            2 => state.stack.push(*state.stack.last().unwrap()),
            3 => {
                let right = state.stack.pop().unwrap();
                let left = state.stack.pop().unwrap();
                state.stack.push(left + right)
            }
            4 => {
                let right = state.stack.pop().unwrap();
                let left = state.stack.pop().unwrap();
                state.stack.push(left - right)
            }
            5 => {
                let right = state.stack.pop().unwrap();
                let left = state.stack.pop().unwrap();
                state.stack.push(left * right)
            }
            6 => {
                let right = state.stack.pop().unwrap();
                let left = state.stack.pop().unwrap();
                state.stack.push(left / right)
            }
            7 => {
                let right = state.stack.pop().unwrap();
                let left = state.stack.pop().unwrap();
                state.stack.push(left % right)
            }
            8 => state.pc = (state.pc as i32 + immediate - 1) as u32,
            9 => {
                let right = state.stack.pop().unwrap();
                let left = state.stack.pop().unwrap();
                if left == right {
                    state.pc = (state.pc as i32 + immediate - 1) as u32
                }
            }
            10 => {
                let right = state.stack.pop().unwrap();
                let left = state.stack.pop().unwrap();
                if left != right {
                    state.pc = (state.pc as i32 + immediate - 1) as u32
                }
            }
            11 => {
                let right = state.stack.pop().unwrap();
                let left = state.stack.pop().unwrap();
                if left < right {
                    state.pc = (state.pc as i32 + immediate - 1) as u32
                }
            }
            12 => {
                let right = state.stack.pop().unwrap();
                let left = state.stack.pop().unwrap();
                if left > right {
                    state.pc = (state.pc as i32 + immediate - 1) as u32
                }
            }
            13 => {
                let immediate = state.stack.len() - (immediate as usize) - 1;
                let tmp = state.stack[immediate as usize];
                state.stack[immediate as usize] = state.stack[(immediate + 1) as usize];
                state.stack[(immediate + 1) as usize] = tmp;
            }
            14 => {
                state.call_stack.push(state.pc);
                state.pc = (state.pc as i32 + immediate - 1) as u32
            }
            15 => {
                state.pc = state.call_stack.pop().unwrap();
            }
            16 => {
                state.stack.pop().unwrap();
            }
            17 => {
                break state.stack.pop().unwrap();
            }
            _ => {
                panic!("unrecognized opcode")
            }
        }
        state.pc += 1;
    };
    res
}
