
mod data_type;
mod expression;
mod statement;
mod prototype;
mod subprogram;
mod ada_grammar {
    include!(concat!(env!("OUT_DIR"), "/ada_grammar.rs"));
}

fn main() {
    println!("SpaceAda Compiler v0.0.1 Pre-Alpha");
    println!("Not much to do, bye!");
    println!("{:?}",
             ada_grammar::subprogram("procedure Fire_Missile(Target_X : in Integer; Target_Y : \
                                      in Integer) is Err : Integer; begin Hit(Target_X, \
                                    Target_Y, Err); Print_Int(Err; end Fire_Missile; "));
}
