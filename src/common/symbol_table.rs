use std::{collections::HashMap, fmt::{self, Display}};

use crate::common::data_type::DataType;

#[derive(Clone, Debug, PartialEq)]
pub enum SymbolTableEntryType {
    Function,
    Variable,
    Unknown,
}

#[derive(Clone)]
pub struct SymbolTableEntry {
    pub name: String,
    pub symbol_table_entry_type: SymbolTableEntryType,
    pub data_type: DataType,
    pub parameter_count: usize,
    pub has_body: bool,
    pub is_array: bool,
    pub array_element_count: i32,
    pub is_pointer: bool,

    // TODO: add custom typedeffed types here somehow!
}

impl SymbolTableEntry {

    pub fn new() -> SymbolTableEntry {
        let instance = SymbolTableEntry {
            name: String::new(),
            symbol_table_entry_type: SymbolTableEntryType::Unknown,
            data_type: DataType::DataTypeUnknown, // data type of variable and return data type for functions
            parameter_count: 0usize,
            has_body: false,
            is_array: false,
            array_element_count: 0i32,
            is_pointer: false,
        };
        instance
    }
}

impl fmt::Debug for SymbolTableEntry {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        write!(f, "SymbolTableEntry {{\n").expect("Write failed!");
        write!(f, "  name: {}\n", &self.name).expect("Write failed!");
        write!(f, "  symbol_table_entry_type: {:?}\n", &self.symbol_table_entry_type).expect("Write failed!");
        write!(f, "  parameter_count: {}\n", &self.parameter_count).expect("Write failed!");
        write!(f, "  has_body: {}\n", &self.has_body).expect("Write failed!");
        write!(f, "  is_array: {}\n", &self.is_array).expect("Write failed!");
        write!(f, "  array_element_count: {}\n", &self.array_element_count).expect("Write failed!");
        write!(f, "  is_pointer: {}\n", &self.is_pointer).expect("Write failed!");
        write!(f, "}}\n").expect("Write failed!");

        Ok(())
    }
}

impl PartialEq<SymbolTableEntry> for SymbolTableEntry {
    fn eq(&self, other: &SymbolTableEntry) -> bool {
        self.symbol_table_entry_type == other.symbol_table_entry_type && self.data_type == other.data_type && self.parameter_count == other.parameter_count
    }
}

pub struct SymbolTable {
    identifier_type_map: HashMap::<String, SymbolTableEntry>,
    debug: bool,
}

impl SymbolTable {

    pub fn new() -> SymbolTable {
        let instance = SymbolTable {
            identifier_type_map: HashMap::<String, SymbolTableEntry>::new(),
            debug: false,
        };
        instance
    }

    pub fn insert(&mut self, varname: String, symbol_table_entry: SymbolTableEntry) {
        // DEBUG
        if self.debug {
            println!("Inserting '{}' with type {:?}", varname, symbol_table_entry);
        }
        self.identifier_type_map.insert(varname, symbol_table_entry);
    }

    pub fn contains(&mut self, varname: &String) -> bool {
        self.identifier_type_map.contains_key(varname)
    }

    pub fn contains_key(&mut self, varname: &String) -> bool {
        self.identifier_type_map.contains_key(varname)
    }

    pub fn retrieve(&mut self, varname: &String) -> SymbolTableEntry {
        self.identifier_type_map.get(varname).unwrap().clone()
    }

    pub fn get(&mut self, varname: &String) -> SymbolTableEntry {
        self.identifier_type_map.get(varname).unwrap().clone()
    }

    pub fn print_symbol_table(&self) {
        println!("print_symbol_table() ------------------------------------------------------------");
        let mut index = 0;
        for (key, value) in self.identifier_type_map.clone().into_iter() {
            println!("{}) {} / {:?}", index, key, value);
            // println!("{} / {:?}", key, value.data_type);
            println!("");
            index = index + 1;
        }
        println!("---------------------------------------------------------------------------------");
    }
}