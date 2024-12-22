use std::collections::VecDeque;

use super::instruction::{Expr, Instruction};

#[derive(Clone, Copy)]
struct VM {
    pc: usize,
    tc: usize,
}

pub fn compile(expr: &Expr, program: &mut Vec<Instruction>) {
    match expr {
        Expr::Char(c) => program.push(Instruction::Char(*c)),
        Expr::Concat(lhs, rhs) => {
            compile(lhs, program);
            compile(rhs, program);
        }
        Expr::Alternate(lhs, rhs) => {
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
        Expr::ZeroOrMore(inner) => {
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
        Expr::OneOrMore(inner) => {
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
        Expr::ZeroOrOne(inner) => {
            program.push(Instruction::Split(0, 0)); // Placeholder
            compile(inner, program);
            let end_pos = program.len();
            if let Instruction::Split(ref mut jmp_offset, _) = &mut program[end_pos - 1] {
                *jmp_offset = end_pos;
            }
        }
        Expr::Text(text) => program.push(Instruction::Text(text.chars().collect())),
        Expr::Any => program.push(Instruction::Any),
        Expr::AnyWhitespace => program.push(Instruction::AnyWhitespace),
        Expr::AnyNonWhitespace => program.push(Instruction::AnyNonWhitespace),
        Expr::AnyDigit => program.push(Instruction::AnyDigit),
        Expr::AnyNonDigit => program.push(Instruction::AnyNonDigit),
        Expr::AnyWord => program.push(Instruction::AnyWord),
        Expr::AnyNonWord => program.push(Instruction::AnyNonWord),
    }
}

pub fn execute(program: &[Instruction], input: &str) -> Option<String> {
    let mut stack: VecDeque<VM> = VecDeque::new();
    let input_chars: Vec<char> = input.chars().collect();
    stack.push_back(VM { pc: 0, tc: 0 });
    while let Some(mut vm) = stack.pop_back() {
        loop {
            if vm.pc >= program.len() || vm.tc > input_chars.len() {
                return None;
            }
            match &program[vm.pc] {
                Instruction::Any => {
                    if vm.tc >= input.len() {
                        break;
                    }
                    vm.pc += 1;
                    vm.tc += 1;
                }
                Instruction::AnyWhitespace => {
                    if vm.tc >= input.len() || !input_chars[vm.tc].is_whitespace() {
                        break;
                    }
                    vm.pc += 1;
                    vm.tc += 1;
                },
                Instruction::AnyNonWhitespace => {
                    if vm.tc >= input.len() || input_chars[vm.tc].is_whitespace() {
                        break;
                    }
                    vm.pc += 1;
                    vm.tc += 1;
                },
                Instruction::AnyDigit => {
                    if vm.tc >= input.len() || !input_chars[vm.tc].is_numeric() {
                        break;
                    }
                    vm.pc += 1;
                    vm.tc += 1;
                }
                Instruction::AnyNonDigit => {
                    if vm.tc >= input.len() || input_chars[vm.tc].is_numeric() {
                        break;
                    }
                    vm.pc += 1;
                    vm.tc += 1;
                },
                Instruction::AnyWord =>  {
                    if vm.tc >= input.len() || !input_chars[vm.tc].is_alphabetic() {
                        break;
                    }
                    vm.pc += 1;
                    vm.tc += 1;
                },
                Instruction::AnyNonWord => {
                    if vm.tc >= input.len() || input_chars[vm.tc].is_alphabetic() {
                        break;
                    }
                    vm.pc += 1;
                    vm.tc += 1;
                },
                Instruction::Char(c) => {
                    if vm.tc >= input.len() || input_chars[vm.tc] != *c {
                        break;
                    }
                    vm.pc += 1;
                    vm.tc += 1;
                }
                Instruction::Text(text) => {
                    if vm.tc >= input.len() {
                        break;
                    }
                    let mut matched = true;
                    for n in 0..text.len() {
                        if vm.tc + n >= input_chars.len() || text[n] != input_chars[vm.tc + n] {
                            matched = false;
                            break;
                        }
                    }
                    if !matched {
                        break;
                    }
                    vm.pc += 1;
                    vm.tc += text.len();
                }
                Instruction::Match => {
                    return Some(input[0..vm.tc].to_string());
                }
                Instruction::Jmp(a) => {
                    vm.pc = *a;
                }
                Instruction::Split(a, b) => {
                    stack.push_back(VM { pc: *b, tc: vm.tc });
                    vm.pc = *a;
                }
            }
        }
    }
    return None;
}
