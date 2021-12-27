#include <vector>
#include <string>
#include <cstdlib>
#include <iostream>
#include <fstream>
#include <sstream>

#define SAFE_AND_SLOW 0 // take out when we want to GO FAST

/*
 * data types
 */
typedef int NUM;

enum OpCode {
    INP =     0x01,
    ADD_REG = 0x02, // immediates start at 0x02
    MUL_REG = 0x03,
    DIV_REG = 0x04,
    MOD_REG = 0x05,
    EQL_REG = 0x06,
    ADD_IMM = 0x12, // aligned with register-based
    MUL_IMM = 0x13,
    DIV_IMM = 0x14,
    MOD_IMM = 0x15,
    EQL_IMM = 0x16
};

struct Insn {
    OpCode op;
    NUM reg_or_imm_a;
    NUM reg_or_imm_b; // based on value of OpCode
};

// error handlers
const int ERR_OUT_OF_RANGE = 3;
const int ERR_BAD_INSN = 4;
const int ERR_BAD_INSN_PARSE = 5;


/*
 * run a single instruction
 */
// returns true if consumes input
int run_insn(int (&state)[4], // deliberate NO-COPY here
              Insn insn, // deliberate copy here
              const char* input_stream,
              int input_stream_idx) {
#ifdef SAFE_AND_SLOW
    // do range checks in one block for readability
    if(insn.reg_or_imm_a >= 4 || insn.reg_or_imm_a < 0) {
        exit(ERR_OUT_OF_RANGE);
    }
    if((insn.op & 0x10) != 0x10 && (insn.reg_or_imm_b >= 4 || insn.reg_or_imm_b < 0)) { // if it's a reg, it should also be in range
        exit(ERR_OUT_OF_RANGE);
    }
#endif

    // INP is weird, so let's take all INPs out first.
    if(insn.op == INP) {
        state[insn.reg_or_imm_a] = input_stream[input_stream_idx] - '0';
        ++input_stream_idx;
        return true;
    }

    // all others have to do some sort of math of some kind
    // step 1, convert registers to immediates
    char store = static_cast<char>(insn.reg_or_imm_a); // has to be a reg
    insn.reg_or_imm_a = state[insn.reg_or_imm_a];
    if((insn.op & 0x10) != 0x10) { // deref if not immediate
        insn.reg_or_imm_b = state[insn.reg_or_imm_b];
    }
    NUM res;
    switch(insn.op & 0x0F) {
        case ADD_REG: res = insn.reg_or_imm_a + insn.reg_or_imm_b; break;
        case MUL_REG: res = insn.reg_or_imm_a * insn.reg_or_imm_b; break;
        case DIV_REG: res = insn.reg_or_imm_a / insn.reg_or_imm_b; break;
        case MOD_REG: res = insn.reg_or_imm_a % insn.reg_or_imm_b; break;
        case EQL_REG: res = insn.reg_or_imm_a == insn.reg_or_imm_b; break;
        default: exit(ERR_BAD_INSN);
    }
    state[store] = res;
    return false;
}

OpCode map_opcode(const std::string&& ref) {
    if(ref == "inp") {
        return INP;
    } else if(ref == "add") {
        return ADD_REG;
    } else if(ref == "mul") {
        return MUL_REG;
    } else if(ref == "div") {
        return DIV_REG;
    } else if(ref == "mod") {
        return MOD_REG;
    } else if(ref == "eql") {
        return  EQL_REG;
    } else {
        exit(ERR_BAD_INSN_PARSE);
    }
}

std::vector<Insn> parse_program(const char* filename) {
    std::vector<Insn> program;

    std::ifstream in{filename};
    std::string line;
    while(std::getline(in, line)) {
        OpCode op = map_opcode(line.substr(0, 3));
        NUM reg_a = line[4] - 'w'; // reg a

        NUM reg_b;
        if(op == INP) {
            reg_b = 0; // DO NOT CARE
        } else {
            if (line[6] >= 'w' && line[6] <= 'z') {
                reg_b = line[6] - 'w';
            } else {
                op = static_cast<OpCode>(op | 0x10);
                std::stringstream ss;
                ss << line.substr(6, line.length() - 6);
                ss >> reg_b;
            }
        }

        program.push_back(Insn { op, reg_a, reg_b });
    }
    return program;
}

// doing this in C first, then finishing up in rust.
NUM run_program(const std::vector<Insn>& program, const char* input_stream) {
    NUM state[4] = { 0, 0, 0, 0 };
    int input_stream_idx = 0;
    for(auto it : program) {
        input_stream_idx = run_insn(state, it, input_stream, input_stream_idx);
    }
//    std::cout << "w: " << state[0] << " x: " << state[1] << " y: " << state[2] << " z: " << state[3] << std::endl;
    return state[3];
}

bool decrement(char (&number)[15]) {
    for(int idx = 13; idx >= 0; --idx) {
        number[idx] -= 1;
        if(number[idx] > '0') {
            break;
        }
        number[idx] = '9';
    }
    return number[0] != '0'; // TODO VERIFY
}

int main() {
    const char* fn = "day24.txt";
    auto program = parse_program(fn);

    int count = 0;


    char start_number[15] = "99999999999999";
    start_number[14] = 0;
    bool good;
    do {
        ++count;
        if(count % 1000 == 0) {
            std::cout << "Trying " << start_number << std::endl;
        }

        NUM result = run_program(program, start_number);
        if(result == 0) {
            std::cout << "Answer: " << start_number << std::endl;
            break;
        }
        good = decrement(start_number);
    } while(good);
    return 0;
}