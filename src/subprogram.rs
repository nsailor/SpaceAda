
use prototype;
use statement::*;
use prototype::*;
use data_type::*;
use expression::*;

#[derive(PartialEq, Clone, Debug)]
pub struct VarDeclaration {
    pub data_type: DataType,
    pub name: String,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Subprogram {
    pub prototype: prototype::Prototype,
    pub variables: Vec<VarDeclaration>,
    pub body: Vec<Statement>,
}
