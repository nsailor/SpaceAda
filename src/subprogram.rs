
use prototype;
use statement::*;
use data_type::*;

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

#[derive(PartialEq, Clone, Debug)]
pub struct Declaration {
    pub prototype: prototype::Prototype,
}
