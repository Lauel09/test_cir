use scanner::Scanner;

mod bytecode;
mod bytecode_interpreter;
mod scanner;
mod compiler;
mod extensions;

fn main() {
    let mut scanner = Scanner::default();
    let test_code = "123.32";
    scanner.scan_tokens(test_code.to_string());
    println!("{:#?}", scanner);
}
