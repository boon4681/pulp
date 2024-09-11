pub mod instruction;
pub mod vm;
pub mod parser;
use instruction::Instruction;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Debug, PartialEq, Clone)]
pub struct Regex {
    program: Vec<Instruction>,
}

#[wasm_bindgen]
impl Regex {
    pub fn new(regex: &str) -> Result<Regex, String> {
        let expr = parser::parse(regex)?;
        let mut program = Vec::new();
        vm::compile(&expr, &mut program);
        program.push(Instruction::Match);
        Ok(Regex { program })
    }

    pub fn matches(&self, input: &str) -> Option<String> {
        vm::execute(&self.program, input)
    }
}
