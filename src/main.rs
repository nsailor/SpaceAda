
mod data_type;
mod expression;
mod ada_grammar {
    include!(concat!(env!("OUT_DIR"), "/ada_grammar.rs"));
}

fn main() {
    println!("SpaceAda Compiler v0.0.1 Pre-Alpha");
    println!("Not much to do, bye!");
    println!("{:?}",
             ada_grammar::expression("Pitch_Down(PID_Signal(6, 5 - 2), 3)"));
}
