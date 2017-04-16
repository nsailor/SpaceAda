
use data_type;

#[derive(PartialEq, Clone, Debug)]
pub struct Prototype {
    pub name: String,
    pub returns: Option<data_type::DataType>,
    pub arguments: Vec<ParameterDeclaration>,
}

#[derive(PartialEq, Clone, Debug)]
pub enum ParameterDirection {
    In,
    Out,
    InOut,
}

#[derive(PartialEq, Clone, Debug)]
pub struct ParameterDeclaration {
    pub data_type: data_type::DataType,
    pub name: String,
    pub direction: ParameterDirection,
}
