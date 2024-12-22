pub mod instruction;
pub mod vm;
use instruction::{Instruction, Statement};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Debug, PartialEq, Clone)]
pub struct Lexer {
    program: Vec<Instruction>,
}

impl Lexer {
    pub fn new(component: Statement) -> Result<Lexer, String> {
        let mut program = Vec::new();
        vm::compile(&component, &mut program);
        Ok(Lexer { program })
    }

    pub fn lex(&self, input: &str) {
        vm::execute(&self.program, input);
    }
}
