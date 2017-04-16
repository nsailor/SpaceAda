
mod data_type;
mod expression;
mod statement;
mod prototype;
mod subprogram;
mod parser;
mod ada_grammar {
    include!(concat!(env!("OUT_DIR"), "/ada_grammar.rs"));
}

use std::fs::File;
use std::io::prelude::*;

fn main() {
    println!("SpaceAda Compiler v0.0.1 Pre-Alpha");
    let mut file = match File::open("code.spad") {
        Ok(f) => f,
        Err(s) => {
            println!("Failed to open `code.spad`: {}", s);
            return;
        }
    };

    let mut code = String::new();
    file.read_to_string(&mut code).unwrap();

    match parser::parse(code.as_str()) {
        Ok(ast_tree) => {
            println!("Parsing complete.");
            println!("Syntax Tree => {:?}", ast_tree)
        }
        Err(s) => println!("Syntax error: {}", s),
    }
}
