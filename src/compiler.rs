/*
    compiler.rs: Compiler internals. 
    TODO
*/

use crate::bytecode;
use crate::extensions;
use crate::scanner;


#[derive(Debug)]
struct Local {
    name: scanner::Token,
    depth: i64,
    is_captured: bool,
}