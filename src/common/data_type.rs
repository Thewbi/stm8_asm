use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

use std::error::Error;

use crate::common::struct_decl::StructDecl;

#[derive(Clone, Debug, PartialEq)]
pub enum DataType {
    DataTypeChar,
    DataTypeUnsignedChar,
    DataTypeInt,
    DataTypeSignedInt,
    DataTypeUnsignedInt,
    DataTypeShort,
    DataTypeSignedShort,
    DataTypeUnsignedShort,
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
            DataType::DataTypeSignedInt => write!(f, "int"),
            DataType::DataTypeUnsignedInt => write!(f, "unsigned int"),
            DataType::DataTypeShort => write!(f, "short"),
            DataType::DataTypeSignedShort => write!(f, "short"),
            DataType::DataTypeUnsignedShort => write!(f, "unsigned short"),
            DataType::DataTypeLong => write!(f, "long"),
            DataType::DataTypeUnsignedLong => write!(f, "unsigned long"),
            DataType::DataTypeFloat => write!(f, "float"),
            DataType::DataTypeDouble => write!(f, "double"),
            DataType::DataTypeVoid => write!(f, "void"),
            DataType::DataTypePointer(data_type) => write!(f, "Pointer-To-[{}]", data_type),
            //DataType::DataTypeStruct(data_type) => write!(f, "StructDecl: [{:?}]", data_type.tag),
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

    pub fn get_size(&self,
            type_system: &HashMap::<String, DataType>) -> usize {
        match self {
            DataType::DataTypeChar => 1,
            DataType::DataTypeUnsignedChar => 1,
            DataType::DataTypeShort => 2,
            DataType::DataTypeSignedShort => 2,
            DataType::DataTypeUnsignedShort => 2,
            DataType::DataTypeInt => 4,
            DataType::DataTypeUnsignedInt => 4,
            DataType::DataTypeLong => 8,
            DataType::DataTypeUnsignedLong => 8,
            DataType::DataTypeFloat => 4,
            DataType::DataTypeDouble => 8,
            DataType::DataTypePointer(inner_data_type) => {
                return 8; // ptr on 64 bit are 8 byte!
                // match **inner_data_type {
                //     DataType::DataTypeLong => 8,
                //     _ => {
                //         todo!();
                //     }
                // }
            }
            DataType::DataTypeTypedeffed(type_identifier) => {
                // println!("type_identifier: '{}'", type_identifier);
                if let Some(data_type) = type_system.get(type_identifier) {
                    // println!("{:?}", data_type);
                    return data_type.get_size(type_system);
                }
                todo!("{}", self);
            }
            DataType::DataTypeArray(data_type, size_option) => {
                if let Some(size) = size_option {
                    println!("size: {}, data_type: {}", size, data_type);
                    let data_type_size = data_type.get_size(type_system);
                    return data_type_size * size;
                } else {
                    panic!("size: unspecified, data_type: {}", data_type);
                }
            }
            _ => {
                todo!("{}", self);
            }
        }
    }

    // for alignment use size()
    // Each data type is align to it's size
    pub fn get_align(&self, type_system: &HashMap::<String, DataType>) -> usize {

        // let align = self.get_size(type_system);

        // // alignment needs to be power of 2 (why: https://www.open-std.org/jtc1/sc22/wg14/www/docs/n1447.htm
        // // > 6.2.8 Alignment of objects > "Every alignment value shall be an integral power of two.")

        // // assert power of 2
        // // https://www.includehelp.com/rust/check-a-given-number-is-the-power-of-2-using-bitwise-operator.aspx
        // assert_eq!(align & (align - 1), 0);

        let mut align = 0;

        // distinguish between elementary data type and arrays
        // the alignment of an array is the alignment of the element type
        match &self {
            DataType::DataTypeArray(element_data_type, size_option) => {

                if let Some(size) = size_option {
                    // println!("size: {}, data_type: {}", size, data_type);
                    // let data_type_size = data_type.get_size(type_system);
                    // return data_type_size * size;

                    let data_type_size = element_data_type.get_size(type_system);
                    //let temp_align = data_type_size * size;
                    align = data_type_size;
                    // if align < temp_align {
                    //     align = temp_align;
                    // }
                } else {
                    panic!("size: unspecified, data_type: {}", element_data_type);
                }
            }
            _ => {
                //let temp_align = self.get_align(type_system);
                align = self.get_size(type_system);
                // if align < temp_align {
                //     align = temp_align;
                // }
            }
        }

        align
    }
}
