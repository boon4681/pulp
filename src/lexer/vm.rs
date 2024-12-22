use std::collections::VecDeque;

use super::instruction::{Instruction, State, Statement, Token};

#[derive(Clone, Copy, Debug)]
struct VM {
    pc: usize,
    tc: usize,
    flag: bool,
    push: bool,
    depth: usize,
}

pub fn compile(component: &Statement, program: &mut Vec<Instruction>) {
    match component {
        Statement::Reader(c) => program.push(Instruction::Match(c.clone())),
        Statement::Concat(v) => {
            for i in v {
                compile(i, program);
            }
        }
        Statement::Alternation(lhs, rhs) => {
            let split = program.len();
            program.push(Instruction::Split(0, 0));
            compile(lhs, program);
            let jmp = program.len();
            program.push(Instruction::Jmp(0));
            let after_jmp = program.len();
            compile(rhs, program);
            let end = program.len();
            if let Instruction::Split(ref mut start, ref mut end) = &mut program[split] {
                *start = split + 1;
                *end = after_jmp;
            }
            if let Instruction::Jmp(ref mut pc) = &mut program[jmp] {
                *pc = end;
            }
        }
        Statement::ZeroOrOne(inner) => {
            program.push(Instruction::Split(0, 0));
            compile(inner, program);
            let end_pos = program.len();
            if let Instruction::Split(ref mut jmp_offset, _) = &mut program[end_pos - 1] {
                *jmp_offset = end_pos;
            }
        }
        Statement::ZeroOrMore(inner) => {
            let split = program.len();
            program.push(Instruction::Split(0, 0));
            compile(inner, program);
            program.push(Instruction::Jmp(split));
            let end_pos = program.len();
            if let Instruction::Split(ref mut start, ref mut end) = &mut program[split] {
                *start = split + 1;
                *end = end_pos;
            }
        }
        Statement::OneOrMore(inner) => {
            let start_pos = program.len();
            compile(inner, program);
            let split = program.len();
            program.push(Instruction::Split(0, 0));
            let end_pos = program.len();
            if let Instruction::Split(ref mut start, ref mut end) = &mut program[split] {
                *start = start_pos;
                *end = end_pos;
            }
        }
        Statement::Condition { cond, inner } => {
            let push: usize = program.len();
            program.push(Instruction::Push(0));
            compile(cond, program);
            program.push(Instruction::Pop);
            let carry = program.len();
            program.push(Instruction::Carry);
            let jump_f = program.len();
            program.push(Instruction::JumpF(0));
            compile(inner, program);
            let end = program.len();
            if let Instruction::Push(addr) = &mut program[push] {
                *addr = carry;
            }
            if let Instruction::JumpF(addr) = &mut program[jump_f] {
                *addr = end;
            }
            program.push(Instruction::UnFlag);
        }
        Statement::Save(name, inner) => {
            program.push(Instruction::StartCapture(name.clone()));
            compile(inner, program);
            program.push(Instruction::EndCapture(name.clone()));
            program.push(Instruction::Save(name.clone()));
        }
    }
}

pub fn execute(program: &[Instruction], input: &str) -> Option<String> {
    let mut stack: VecDeque<VM> = VecDeque::new();
    let mut state: VecDeque<State> = VecDeque::new();
    let mut tokens: VecDeque<Token> = VecDeque::new();
    let input_chars: Vec<char> = input.chars().collect();
    let mut old: Option<VM> = None;
    stack.push_back(VM {
        pc: 0,
        tc: 0,
        flag: false,
        push: false,
        depth: 0,
    });
    while let Some(mut vm) = stack.pop_back() {
        let mut opc = vm.pc;
        println!("{:?} {} {}", vm, program.len(), stack.len());
        loop {
            if vm.pc >= program.len() || vm.tc > input_chars.len() {
                break;
            }
            println!("{:?}", program[vm.pc]);
            match &program[vm.pc] {
                Instruction::Match(regex) => {
                    if let Some(text) = regex.matches(&input[vm.tc..]) {
                        let len = text.len();
                        println!("{}", text);
                        if !vm.push {
                            state.push_back(State::Text(text, vm.tc, vm.tc + len));
                        }
                        vm.pc += 1;
                        vm.tc += len;
                    } else if vm.push {
                        vm.flag = true;
                        vm.pc += 1;
                    }
                }
                Instruction::Split(a, b) => {
                    stack.push_back(VM {
                        pc: *b,
                        tc: vm.tc,
                        flag: vm.flag,
                        push: false,
                        depth: vm.depth,
                    });
                    vm.pc = *a;
                }
                Instruction::Jmp(a) => {
                    vm.pc = *a;
                }
                Instruction::JumpF(a) => {
                    vm.pc += 1;
                    if vm.flag {
                        vm.pc = *a
                    }
                }
                Instruction::Save(name) => {
                    let s = state.back_mut();
                    if let Some(m) = s {
                        match m {
                            State::StartCapture(_, _) => panic!("ERROR Invalid state"),
                            State::Text(text, start, end) => {
                                if vm.depth == 0 {
                                    tokens.push_back(Token {
                                        name: name.clone(),
                                        text: text.clone(),
                                        start: start.clone(),
                                        end: end.clone(),
                                        children: vec![],
                                    });
                                    state.pop_back();
                                }
                            }
                            State::List(n, _) => {
                                if vm.depth == 0 {
                                    if let Some(mut t) = m.clone().into_token() {
                                        t.name = name.to_string();
                                        tokens.push_back(t);
                                    }
                                    state.pop_back();
                                } else {
                                    *n = name.clone();
                                }
                            }
                        }
                    }
                    vm.pc += 1;
                }
                Instruction::StartCapture(a) => {
                    vm.depth += 1;
                    state.push_back(State::StartCapture(a.clone(), vm.tc));
                    vm.pc += 1;
                }
                Instruction::EndCapture(a) => {
                    println!("{:?}", state);
                    let mut i = state.len();
                    let mut capture: Option<State> = None;
                    while i > 0 {
                        i -= 1;
                        if let Some(State::StartCapture(_, _)) = state.get(i) {
                            capture = state.get(i).cloned();
                            break;
                        }
                    }
                    if let Some(State::StartCapture(_, _)) = capture {
                        let mut list: Vec<State> = vec![];
                        while state.len() > i + 1 {
                            if let Some(t) = state.pop_back() {
                                println!("out: {:?}", t);
                                list.push(t);
                            } else {
                                panic!("error");
                                // error
                            }
                        }
                        list.reverse();
                        state.pop_back();
                        state.push_back(State::List("".to_string(), list));
                        // let mut text = String::new();
                        // while state.len() > i + 1 {
                        //     if let Some(State::Text(t, _, _)) = state.pop_back() {
                        //         text = t + text.as_str();
                        //         println!("out: {}", text);
                        //     } else {
                        //         panic!("error");
                        //         // error
                        //     }
                        // }
                        // state.pop_back();
                        // let len = 0;
                        // state.push_back(State::Text(text, start, start + len));
                    }
                    vm.depth -= 1;
                    vm.pc += 1;
                }
                Instruction::Flag => {
                    vm.pc += 1;
                    vm.flag = true
                }
                Instruction::UnFlag => {
                    vm.pc += 1;
                    vm.flag = false
                }
                Instruction::Carry => {
                    if let Some(ovm) = old {
                        vm.flag = ovm.flag
                    } else {
                        panic!("ERROR 1111")
                    }
                    vm.pc += 1;
                }
                Instruction::Push(v) => {
                    vm.pc += 1;
                    stack.push_back(VM {
                        pc: *v,
                        tc: vm.tc,
                        flag: vm.flag,
                        push: false,
                        depth: vm.depth,
                    });
                    stack.push_back(VM {
                        pc: vm.pc,
                        tc: vm.tc,
                        flag: vm.flag,
                        push: true,
                        depth: vm.depth,
                    });
                    break;
                }
                Instruction::Pop => {
                    old = Some(vm);
                    vm.pc += 1;
                    break;
                }
            }
            if opc == vm.pc {
                // error
                // panic!("ERROR")
                break;
            }
            opc = vm.pc;
        }
    }
    println!("{:?}", tokens);
    return None;
}
