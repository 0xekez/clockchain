#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;
use ink_prelude::string::String;

#[ink::contract]
mod polka_evaluator {

    #[ink(storage)]
    pub struct PolkaEvaluator {
        /// Stores a single `int32` value on the storage.
        value: i32,
    }

    impl PolkaEvaluator {
        /// Constructor that initializes the `int32` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: i32) -> Self {
            Self { value: init_value }
        }

        /// Constructor that initializes the `int32` value to `0`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        /// Runs a hex string of Clockchain assembly.
        #[ink(message)]
        pub fn execute(&mut self, s: crate::String) {
            use ink_prelude::vec::Vec;
            use ink_prelude::vec;

            struct InterpreterState {
                pc: u32,
                call_stack: Vec<u32>,
                stack: Vec<i32>,
            }

            let mut state = InterpreterState {
                pc: 0,
                call_stack: vec![],
                stack: vec![],
            };
            let mut bytecode = vec![];
            for byte in (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap()) {
                bytecode.push(byte);
            }

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
                        panic!("unrecognized opcode.");
                    }
                }
                state.pc += 1;
            };
            self.value = res;
        }

        /// Simply returns the current value of our `int32`.
        #[ink(message)]
        pub fn get(&self) -> i32 {
            self.value
        }
    }

    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let polka_evaluator = PolkaEvaluator::default();
            assert_eq!(polka_evaluator.get(), 0);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut polka_evaluator = PolkaEvaluator::new(0);
            polka_evaluator.execute("010a0000001100000000".to_string());
            assert_eq!(polka_evaluator.get(), 10);
        }
    }
}
