
mod data_type;
mod expression;
mod statement;
mod prototype;
mod ada_grammar {
    include!(concat!(env!("OUT_DIR"), "/ada_grammar.rs"));
}

fn main() {
    println!("SpaceAda Compiler v0.0.1 Pre-Alpha");
    println!("Not much to do, bye!");
    println!("{:?}",
             ada_grammar::prototype("procedure Fire_Missile(Target_X : in Integer; Target_Y : \
                                     in Integer)"));
}
