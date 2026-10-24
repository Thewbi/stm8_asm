use std::fmt;
use std::str::FromStr;

use std::error::Error;

use crate::common::struct_decl::StructDecl;

#[derive(Clone, Debug, PartialEq)]
pub enum DataType {
    DataTypeChar,
    DataTypeUnsignedChar,
    DataTypeInt,
    DataTypeUnsignedInt,
    DataTypeShort,
    DataTypeLong,
    DataTypeUnsignedLong,
    DataTypeFloat,
    DataTypeDouble,
    DataTypeVoid, // is it beneficial to treat void as a data type? (void-pointer?)
    DataTypePointer(Box<DataType>),
    DataTypeStruct(StructDecl),
    DataTypeArray(Box<DataType>, Option<usize>),
    DataTypeTypedeffed(String),
    DataTypeUnknown,
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DataType::DataTypeChar => write!(f, "char"),
            DataType::DataTypeUnsignedChar => write!(f, "unsigned char"),
            DataType::DataTypeInt => write!(f, "int"),
            DataType::DataTypeUnsignedInt => write!(f, "unsigned int"),
            DataType::DataTypeShort => write!(f, "short"),
            DataType::DataTypeLong => write!(f, "long"),
            DataType::DataTypeUnsignedLong => write!(f, "unsigned long"),
            DataType::DataTypeFloat => write!(f, "float"),
            DataType::DataTypeDouble => write!(f, "double"),
            DataType::DataTypeVoid => write!(f, "void"),
            DataType::DataTypePointer(data_type) => write!(f, "Pointer-To-[{}]", data_type),
            DataType::DataTypeStruct(data_type) => write!(f, "StructDecl: [{:?}]", data_type),
            DataType::DataTypeArray(data_type, size) => write!(f, "Array-Of-[{}]-Size-[{:?}]", data_type, size),
            DataType::DataTypeTypedeffed(data_type) => write!(f, "Typedeffed-[{}]", data_type),
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
        } else if s == "unsigned int" {
            Ok(DataType::DataTypeUnsignedInt)
        } else if s == "char" {
            Ok(DataType::DataTypeChar)
        } else if s == "unsigned char" {
            Ok(DataType::DataTypeUnsignedChar)
        } else if s == "short" {
            Ok(DataType::DataTypeShort)
        } else if s == "long" {
            Ok(DataType::DataTypeLong)
        } else if s == "ulong" {
            Ok(DataType::DataTypeUnsignedLong)
        } else if s == "unsigned long" {
            Ok(DataType::DataTypeUnsignedLong)
        }  else if s == "float" {
            Ok(DataType::DataTypeFloat)
        } else if s == "double" {
            Ok(DataType::DataTypeDouble)
        } else if s == "void" {
            Ok(DataType::DataTypeVoid)
        } else if s == "struct" {
            Ok(DataType::DataTypeStruct(StructDecl::new()))
        } else {
            Err(format!("[DataType::FromStr] Can not parse '{}' into type", s).into())
        }
    }
}

impl DataType {
    pub fn get_size(&self) -> usize {
        match self {
            DataType::DataTypeChar => 1,
            DataType::DataTypeShort => 2,
            DataType::DataTypeInt => 4,
            DataType::DataTypeUnsignedInt => 4,
            DataType::DataTypeLong => 8,
            DataType::DataTypeUnsignedLong => 8,
            DataType::DataTypeFloat => 4,
            DataType::DataTypeDouble => 8,
            DataType::DataTypePointer(inner_data_type) => {
                match **inner_data_type {
                    DataType::DataTypeLong => 8,
                    _ => {
                        todo!();
                    }
                }
            }
            _ => {
                todo!();
            }
        }
    }
}
