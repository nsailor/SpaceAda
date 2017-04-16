
extern crate llvm;
extern crate llvm_sys;

mod data_type;
mod expression;
mod statement;
mod prototype;
mod subprogram;
mod parser;
mod ada_grammar {
    include!(concat!(env!("OUT_DIR"), "/ada_grammar.rs"));
}
mod codegen;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use llvm::*;

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

    let ast_tree = match parser::parse(code.as_str()) {
        Ok(ast_tree) => {
            println!("Parsing complete.");
            ast_tree
        }
        Err(s) => {
            println!("Syntax error: {}", s);
            return;
        }
    };

    let ctx = Context::new();
    let module = Module::new("code.spad", &ctx);

    let mut codegen_ctx = codegen::CodegenContext {
        ctx: &ctx,
        module: &module,
        fmap: std::collections::HashMap::new(),
    };
    println!("Generating code...");
    for node in ast_tree {
        codegen_ctx.codegen(&node);
    }

    println!("----------------\nLLVM IR\n----------------");
    println!("{:?}", module);
    println!("Generating machine code...");
    module.compile(Path::new("spada-code.o"), 0).unwrap();
    println!("Done! Object code has been written to `spada-code.o`.");
}
