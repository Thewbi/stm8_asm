use std::fmt;
use std::str::FromStr;

use crate::common::data_type::DataType;

#[derive(Clone, PartialEq)]
pub struct StructDecl {
    pub(crate) tag: String,
    pub(crate) members: Vec<StructDeclMember>
}

impl StructDecl {

    pub fn new() -> StructDecl {
        StructDecl {
            tag: String::new(),
            members: Vec::<StructDeclMember>::new()
        }
    }
}

impl fmt::Debug for StructDecl {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        write!(f, "struct {} {{\n", self.tag).expect("Write failed!");
        for member in &self.members {
            //write!(f, "    {} {};\n", &member.member_type, member.member_name).expect("Write failed!");
            write!(f, "    {} {};\n", &member.data_type, member.member_name).expect("Write failed!");
        }
        write!(f, "}}\n").expect("Write failed!");

        Ok(())
    }
}

#[derive(Clone, PartialEq)]
pub struct StructDeclMember {
    pub(crate) member_name: String,
    pub(crate) member_type: String, // can the member_type be of type data_type.rs??? It also has to store typedefs!
    pub(crate) data_type: DataType,
}

impl StructDeclMember {

    pub fn new() -> StructDeclMember {
        StructDeclMember {
            member_name: String::new(),
            member_type: String::new(),
            data_type: DataType::DataTypeUnknown,
        }
    }
}
