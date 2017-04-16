
extern crate llvm;

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

    match parser::parse(code.as_str()) {
        Ok(ast_tree) => {
            println!("Parsing complete.");
            println!("Syntax Tree => {:?}", ast_tree)
        }
        Err(s) => println!("Syntax error: {}", s),
    }

    println!("----------------\nLLVM Output\n----------------");
    let ctx = Context::new();
    let module = Module::new("spadac1", &ctx);

    let ftype = FunctionType::new(Type::get::<i32>(&ctx), &[Type::get::<i32>(&ctx)]);
    let func = module.add_function("Square", ftype);
    let entry = func.append("entry");
    let builder = Builder::new(&ctx);
    builder.position_at_end(entry);
    let arg_x = &func[0];
    let sum = builder.build_mul(arg_x, arg_x);
    builder.build_ret(sum);

    println!("Module = {:?}", module);
    println!("Compiling LLVM-IR...");
    module.compile(Path::new("spad-code.o"), 1).unwrap();
    println!("Done!");
}
