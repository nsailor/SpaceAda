
extern crate llvm;
extern crate llvm_sys;
extern crate docopt;
extern crate rustc_serialize;

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
use docopt::Docopt;

const USAGE: &'static str = "
Usage: spadac [options] INPUT 
       spadac (--help | --version)

Options:
    -h, --help         Show this message.
    -v, --version      Show the version of spadac.
    --emit TYPE        Configure the output that spadac will produce.
                       Valid values: ir, obj.
    -o, --output PATH  The path of the output file.
";

#[allow(non_snake_case)]
#[derive(RustcDecodable, Debug)]
struct Args {
    arg_INPUT: String,
    flag_output: Option<String>,
    flag_emit: Option<Emit>,
    flag_help: bool,
    flag_version: bool,
}

#[derive(RustcDecodable, Debug)]
enum Emit {
    Ir,
    Obj,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    if args.flag_version {
        println!("SpaceAda Compiler v0.0.1 Pre-Alpha.");
        println!("(C) 2017 Jason Barmparesos. All rights reserved.");
        return;
    }

    let output_format = args.flag_emit.unwrap_or(Emit::Obj);
    let output_path = args.flag_output.unwrap_or("spada.o".to_string());
    let output_path = Path::new(output_path.as_str());

    let mut file = match File::open(args.arg_INPUT.as_str()) {
        Ok(f) => f,
        Err(s) => {
            println!("Failed to open `{}`: {}", args.arg_INPUT, s);
            return;
        }
    };

    let mut code = String::new();
    file.read_to_string(&mut code).unwrap();

    let ast_tree = match parser::parse(code.as_str()) {
        Ok(ast_tree) => ast_tree,
        Err(s) => {
            println!("Syntax error: {}", s);
            return;
        }
    };

    let ctx = Context::new();
    let module = Module::new(args.arg_INPUT.as_str(), &ctx);

    let mut codegen_ctx = codegen::CodegenContext {
        ctx: &ctx,
        module: &module,
        fmap: std::collections::HashMap::new(),
    };

    for node in ast_tree {
        codegen_ctx.codegen(&node);
    }

    match output_format {
        Emit::Ir => {
            let ir = format!("{:?}", module);
            let mut out_file = std::fs::File::create(output_path).unwrap();
            out_file.write_all(ir.as_bytes()).unwrap()
        }
        Emit::Obj => module.compile(output_path, 1).unwrap(),
    }
}
