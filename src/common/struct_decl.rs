use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

use crate::common::data_type::DataType::{self, DataTypeArray};

#[derive(Clone, PartialEq)]
pub struct StructDecl {
    pub(crate) tag: String,
    pub(crate) members: Vec<StructDeclMember>,
    pub(crate) size: usize
}

impl StructDecl {

    pub fn new() -> StructDecl {
        StructDecl {
            tag: String::new(),
            members: Vec::<StructDeclMember>::new(),
            size: 0usize,
        }
    }

    /// - alignment and padding rules. (Nora Sandler, page 493)
	/// - the entire struct is memory aligned with the alignment of it's most strictly
    ///   aligned member (This is the member with the largest alignment)
	///   e.g. long has an alignment of 8. If a struct contains a long,
    ///   the struct is aligned at boundaries of 8.
	/// - padding: the size of a struct must be the multiple of it's alignment.
    ///   If the struct is smaller, it is padded.
	///   e.g. long has an alignment of 8. If a struct contains a long, the struct
    ///   is aligned at boundaries of 8. If the struct has an alignment of 8,
	///   it's length must be a multiple of 8. Use padding if necessary.
    ///
    /// Debugging: https://www.embedcalc.com/structalign/#structalign
    pub fn compute_memory_layout(&mut self, type_system: &HashMap::<String, DataType>) {

        println!("compute_memory_layout()");

        let mut align = 0;

        let len = self.members.len();
        for i in 0..len {

            let memb = &self.members[i];

            // distingish between elementary data type and arrays
            // the alignment of an array is the alignment of the element type
            match &memb.data_type {
                DataType::DataTypeArray(element_data_type, size_option) => {

                    if let Some(size) = size_option {
                        // println!("size: {}, data_type: {}", size, data_type);
                        // let data_type_size = data_type.get_size(type_system);
                        // return data_type_size * size;

                        let data_type_size = element_data_type.get_size(type_system);
                        //let temp_align = data_type_size * size;
                        let temp_align = data_type_size;
                        if align < temp_align {
                            align = temp_align;
                        }
                    } else {
                        panic!("size: unspecified, data_type: {}", element_data_type);
                    }
                }
                _ => {
                    let temp_align = memb.data_type.get_align(type_system);
                    if align < temp_align {
                        align = temp_align;
                    }
                }
            }
        }

        // find largest size / alignment of each member
        //let align = self.members.iter().map(|m| m.data_type.get_align(type_system)).max().unwrap();
        // println!("compute_memory_layout() align: {}", align);

        let mut offset: usize = 0;

        let mut wasted_bytes = 0;

        let len = self.members.len();
        for i in 0..len {

            let dt = &self.members[i].data_type;
            let dt_size = DataType::get_size(&dt, type_system);
            let dt_align = DataType::get_align(&dt, type_system);

            println!("data_type: {}, dt_size: {}, dt_align: {}", dt, dt_size, dt_align);

            // fields are aligned to a boundary!
            // This boundary is not the align of the entire struct.
            // Instead each field is aligned to a multiple of it's
            // own size!
            let old_offset = offset;
            //while offset % dt_size != 0 {
            while offset % dt_align != 0 {
                offset = offset + 1;
            }
            wasted_bytes = wasted_bytes + (offset - old_offset);

            self.members[i].offset = offset;
            self.members[i].size = dt_size;
            offset = offset + dt_size;
        }

        let old_offset = offset;
        while offset % align != 0 {
            offset = offset + 1;
        }
        wasted_bytes = wasted_bytes + (offset - old_offset);

        self.size = offset;

        let mut waste_percentage: f32 = wasted_bytes as f32 / self.size as f32;
        waste_percentage = waste_percentage * 100.0;

        println!("Total Size: {} bytes", self.size);
        println!("Alignment: {}-byte", align);
        println!("Padding Waste: {} bytes ({}%)", wasted_bytes, waste_percentage);

        let len = self.members.len();
        for i in 0..len {
            let offset = &self.members[i].offset;
            let size = &self.members[i].size;
            let name = &self.members[i].member_name;
            println!("Member {}) Offset:{} Size:{} Name:{}", i, offset, size, name);
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
    pub(crate) offset: usize,
    pub(crate) size: usize,
}

impl StructDeclMember {

    pub fn new() -> StructDeclMember {
        StructDeclMember {
            member_name: String::new(),
            member_type: String::new(),
            data_type: DataType::DataTypeUnknown,
            offset: 0usize,
            size: 0usize,
        }
    }
}
