use std::error::Error;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
pub enum DataType {
    DataTypeByte,
    DataTypeChar,
    DataTypeInt,
    DataTypeShort,
    DataTypeLong,
    DataTypeFloat,
    DataTypeDouble,

    DataTypeUnknown,
}

impl FromStr for DataType {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "int" {
            Ok(DataType::DataTypeInt)
        } else if s == "char" {
            Ok(DataType::DataTypeChar)
        } else if s == "short" {
            Ok(DataType::DataTypeShort)
        } else if s == "long" {
            Ok(DataType::DataTypeLong)
        } else if s == "float" {
            Ok(DataType::DataTypeFloat)
        } else if s == "double" {
            Ok(DataType::DataTypeDouble)
        } else {
            Err(format!("Can not parse {}", s).into())
        }
    }
}