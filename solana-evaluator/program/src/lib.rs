use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    program_error::ProgramError,
    pubkey::Pubkey,
};

/// Each calling account will create a ResultAccount that has this
/// structure and pass ownership to the program. The program will then
/// store the results of each execution in that account such that it
/// always has the result of the last invocation by the owner.
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct ResultAccount {
    /// The result of the last program execution invoked by this owner account.
    pub result: i32,
}

struct InterpreterState {
    pc: u32,
    call_stack: Vec<u32>,
    stack: Vec<i32>,
}

#[cfg(not(feature = "exclude_entrypoint"))]
entrypoint!(process_instruction);

/// The account passed in ought to contain a `ResultAccount`. This
/// program will update the `result` value in the `ResultAccount` when
/// executed.
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> entrypoint::ProgramResult {
    // Get the account that stores result information.
    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;

    // The account must be owned by the program in order for the
    // program to write to it. If that is not the case then the
    // program has been invoked incorrectly and we report as much.
    if account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Deserialize the greeting information from the account, modify
    // it, and then write it back.
    let mut result = ResultAccount::try_from_slice(&account.data.borrow())?;

    let mut state = InterpreterState {
        pc: 0,
        call_stack: vec![],
        stack: vec![],
    };

    let bytecode = instruction_data;

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
                eprintln!("unrecognized opcode ({})", opcode);
                std::process::exit(-1);
            }
        }
        state.pc += 1;
    };

    result.result = res;
    result.serialize(&mut &mut account.data.borrow_mut()[..])?;
    Ok(())
}
