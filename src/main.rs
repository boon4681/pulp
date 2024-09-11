use pulp::regex::{self as Regex, instruction::Instruction};

fn main() {
    let regex = "a*(bc|e)+";
    let expr_result = Regex::parser::parse(regex);
    if let Ok(expr) = expr_result {
        println!("{}", expr);
        let mut program = Vec::new();
        Regex::vm::compile(&expr, &mut program);
        program.push(Instruction::Match);

        let input = "aaabcbceebc";
        println!("Executing");
        if let Some(matches) = Regex::vm::execute(&program, input) {
            println!("Input matches pattern: {}", matches);
        }
    } else {
        println!("{}", expr_result.err().unwrap());
    }
}
