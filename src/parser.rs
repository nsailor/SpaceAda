
extern crate regex;

use subprogram;
use ada_grammar;

#[derive(PartialEq, Clone, Debug)]
pub enum ASTNode {
    Subprogram(subprogram::Subprogram),
    Declaration(subprogram::Declaration),
}

pub fn parse(input: &str) -> Result<Vec<ASTNode>, ada_grammar::ParseError> {
    // Remove the comments and replace them with newlines so that line numbers
    // from the PEG parser still apply.
    let comment_regex = regex::Regex::new(r"(?m)--.*\n").unwrap();
    let preprocessed = comment_regex.replace_all(input, "\n");

    ada_grammar::compilation_unit(preprocessed.into_owned().as_str())
}
