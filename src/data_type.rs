
#[derive(PartialEq, Clone, Debug)]
pub enum DataType {
    Integer,
    Float,
}

#[test]
fn test_data_type_parsing() {
    use ada_grammar::*;
    assert_eq!(data_type("Integer"), Ok(DataType::Integer));
    assert!(data_type("hello").is_err());
}
