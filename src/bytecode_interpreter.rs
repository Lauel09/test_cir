use std::fmt::format;

use crate::bytecode::{
    Chunk,
    Op,
};


/// Takes in a `Chunk` and for each `Op` prints the disassembly 
/// information of those opcodes with `lineno` and `idx`
pub fn dis_code(chunk: &Chunk) -> Vec<String> {

    let mut lines: Vec<String> = Vec::new();
   
    for (idx, (op, lineno)) in chunk.code.iter().enumerate() {
        println!("Hello, World!");
        let formatted_op = match *op {
            Op::Return  => "OP_RETURN".to_string(),
            Op::Constant(constant_idx) => {
                let get_idx_value = match chunk.constants.get(constant_idx) {
                    Some(value) => value,
                    None  => panic!("Invalid index value!"),
                };

                format!(
                    "OP_CONSTANT {} (idx={})",
                    get_idx_value,
                    constant_idx
                )
            },
                Op::Nil     =>  "OP_NIL".to_string(),
                Op::True    =>  "OP_TRUE".to_string(),
                Op::False   =>  "OP_FALSE".to_string(),
                Op::Negate  =>  "OP_NEGATE".to_string(),
                Op::Add     =>  "OP_ADD".to_string(),
                Op::Subtract    => "OP_SUBTRACT".to_string(),
                Op::Multiply    => "OP_MULTIPLY".to_string(),
                Op::Equal   =>  "OP_EQUAL".to_string(),
                Op::Greater =>  "OP_GREATER".to_string(),
                Op::Less    =>  "OP_LESS".to_string(),
                Op::Divide  => "OP_DIVIDE".to_string(),
                Op::Pop     => "OP_POP".to_string(),
                Op::Print   =>  "OP_PRINT".to_string(),
                Op::Not     =>  "OP_NOT".to_string(),
        };

        lines.push(
            format!(
                "{0: <04}   {1: <50} line {2: <50}",
                idx, formatted_op, lineno.value
            )
        )
    }
    lines
}   

/// Takes in a `Chunk` and `name` and then proceeds to disassemble the whole
/// chunk.
pub fn dis_chunk( chunk: &Chunk, name: &str ) -> String {
    let mut lines: Vec<String> = Vec::with_capacity(100);

    if !name.is_empty() {
        lines.push(format!("============ {} ============", name));
    }

    lines.push("------------ constants ------------".to_string());

    for(idx, constant) in chunk.constants.iter().enumerate() {
        lines.push(format!("{:<4} {}", idx, constant));
    }

    lines.push("\n------------ code -----------------".to_string());
    
    for line in dis_code(&chunk) {
        lines.push(line);
    } 

    lines.join("\n")
}