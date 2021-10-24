//SPDX-License-Identifier: UNLICENSED

pragma solidity ^ 0.8 .7;


contract Interpreter {
    constructor() {}

    struct InterpreterState {
        uint32 pc;
        uint32[1024] callstack;
        uint256 cs_length;
        int32[1024] stack;
        uint256 stack_length;
    }

    event log(uint256 time, int32 b);

    function execute(uint8[] memory code) public payable {
        emit log(block.timestamp, 1);
        InterpreterState memory state;
        state.pc = 0;
        state.cs_length = 0;
        state.stack_length = 0;
        while (true) {
            uint32 inst_start = (state.pc * 5);
            uint8 opcode = code[inst_start];
            int32 immediate = int32(uint32(code[inst_start + 1]) << 0 |
                 uint32(code[inst_start + 2]) << 8 |
                 uint32(code[inst_start + 3]) << 16 |
                 uint32(code[inst_start + 4]) << 24);
            uint256 last = 0;
            if (state.stack_length > 0) last = state.stack_length - 1;

            if (opcode == 1) {
                state.stack[state.stack_length] = immediate;
                state.stack_length++;
            } else if (opcode == 2) {
                state.stack[state.stack_length] = state.stack[last];
                state.stack_length++;
            } else if (opcode == 3) {
                state.stack[last - 1] = state.stack[last - 1] + state.stack[last];
                state.stack_length -= 1;
            } else if (opcode == 4) {
                state.stack[last - 1] = state.stack[last - 1] - state.stack[last];
                state.stack_length -= 1;
            } else if (opcode == 5) {
                state.stack[last - 1] = state.stack[last - 1] * state.stack[last];
                state.stack_length -= 1;
            } else if (opcode == 6) {
                state.stack[last - 1] = state.stack[last - 1] / state.stack[last];
                state.stack_length -= 1;
            } else if (opcode == 7) {
                state.stack[last - 1] = state.stack[last - 1] % state.stack[last];
                state.stack_length -= 1;
            } else if (opcode == 8) {
                state.pc = uint32(int32(state.pc) + immediate - 1);
            } else if (opcode == 9) {
                bool jmp = state.stack[last - 1] == state.stack[last];
                state.stack_length -= 2;
                if (jmp) state.pc = uint32(int32(state.pc) + immediate - 1);
            } else if (opcode == 10) {
                bool jmp = state.stack[last - 1] != state.stack[last];
                state.stack_length -= 2;
                if (jmp) state.pc = uint32(int32(state.pc) + immediate - 1);
            } else if (opcode == 11) {
                bool jmp = state.stack[last - 1] < state.stack[last];
                state.stack_length -= 2;
                if (jmp) state.pc = uint32(int32(state.pc) + immediate - 1);
            } else if (opcode == 12) {
                bool jmp = state.stack[last - 1] > state.stack[last];
                state.stack_length -= 2;
                if (jmp) state.pc = uint32(int32(state.pc) + immediate - 1);
            } else if (opcode == 13) {
                uint256 imm = state.stack_length - uint256(int256(immediate)) - 1;
                int32 tmp = state.stack[imm];
                state.stack[imm] = state.stack[imm + 1];
                state.stack[imm + 1] = tmp;
            } else if (opcode == 14) {
                state.callstack[state.cs_length] = state.pc;
                state.cs_length++;
                state.pc = uint32(int32(state.pc) + immediate - 1);
            } else if (opcode == 15) {
                state.pc = state.callstack[state.cs_length - 1];
                state.cs_length--;
            } else if (opcode == 16) {
                state.stack_length--;
            } else if (opcode == 17) {
                emit log(block.timestamp, state.stack[last]);
                break;
            } else {
                emit log(block.timestamp, 0);
                break;
            }
            state.pc++;
        }
    }
}
