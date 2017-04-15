
use ada_grammar::*;

#[derive(PartialEq, Clone, Debug)]
pub enum DataType {
    Integer,
}

#[test]
fn test_data_type_parsing() {
    assert_eq!(data_type("Integer"), Ok(DataType::Integer));
    assert!(data_type("hello").is_err());
}
