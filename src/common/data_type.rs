use std::fmt;
use std::str::FromStr;

use std::error::Error;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum DataType {
    DataTypeByte,
    DataTypeChar,
    DataTypeInt,
    DataTypeUnsignedInt,
    DataTypeShort,
    DataTypeLong,
    DataTypeUnsignedLong,
    DataTypeFloat,
    DataTypeDouble,

    DataTypeVoid, // is it beneficial to treat void as a data type? (void-pointer?)

    DataTypeUnknown,
}

impl fmt::Display for DataType {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DataType::DataTypeByte => write!(f, "byte"),
            DataType::DataTypeChar => write!(f, "char"),
            DataType::DataTypeInt => write!(f, "int"),
            DataType::DataTypeUnsignedInt => write!(f, "uint"),
            DataType::DataTypeShort => write!(f, "short"),
            DataType::DataTypeLong => write!(f, "long"),
            DataType::DataTypeUnsignedLong => write!(f, "ulong"),
            DataType::DataTypeFloat => write!(f, "float"),
            DataType::DataTypeDouble => write!(f, "double"),
            DataType::DataTypeVoid => write!(f, "void"),
            DataType::DataTypeUnknown => write!(f, "unknown"),
        }
    }
}

impl FromStr for DataType {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "int" {
            Ok(DataType::DataTypeInt)
        } else if s == "uint" {
            Ok(DataType::DataTypeUnsignedInt)
        } else if s == "char" {
            Ok(DataType::DataTypeChar)
        } else if s == "short" {
            Ok(DataType::DataTypeShort)
        } else if s == "long" {
            Ok(DataType::DataTypeLong)
        } else if s == "ulong" {
            Ok(DataType::DataTypeUnsignedLong)
        } else if s == "float" {
            Ok(DataType::DataTypeFloat)
        } else if s == "double" {
            Ok(DataType::DataTypeDouble)
        } else if s == "void" {
            Ok(DataType::DataTypeVoid)
        } else {
            Err(format!("Can not parse {}", s).into())
        }
    }
}