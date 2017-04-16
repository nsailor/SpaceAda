
mod data_type;
mod expression;
mod statement;
mod ada_grammar {
    include!(concat!(env!("OUT_DIR"), "/ada_grammar.rs"));
}

fn main() {
    println!("SpaceAda Compiler v0.0.1 Pre-Alpha");
    println!("Not much to do, bye!");
    println!("{:?}", ada_grammar::statement("Fire(64, 55);"));
}
