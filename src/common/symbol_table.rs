use std::collections::HashMap;

use crate::common::data_type::DataType;

#[derive(Clone, Debug, PartialEq)]
pub enum SymbolTableEntryType {
    Function,
    Variable,
    Unknown,
}

#[derive(Clone, Debug)]
pub struct SymbolTableEntry {
    pub symbol_table_entry_type: SymbolTableEntryType,
    pub data_type: DataType,
    pub parameter_count: usize,
    pub has_body: bool,

    // TODO: add custom typedeffed types here somehow!
}

impl SymbolTableEntry {

    pub fn new() -> SymbolTableEntry {
        let instance = SymbolTableEntry {
            symbol_table_entry_type: SymbolTableEntryType::Unknown,
            data_type: DataType::DataTypeUnknown, // data type of variable and return data type for functions
            parameter_count: 0,
            has_body: false,
        };

        instance
    }
}

impl PartialEq<SymbolTableEntry> for SymbolTableEntry {
    fn eq(&self, other: &SymbolTableEntry) -> bool {
        self.symbol_table_entry_type == other.symbol_table_entry_type && self.data_type == other.data_type && self.parameter_count == other.parameter_count
    }
}

pub struct SymbolTable {
    identifier_type_map: HashMap::<String, SymbolTableEntry>,
}

impl SymbolTable {

    pub fn new() -> SymbolTable {

        let instance = SymbolTable {
            identifier_type_map: HashMap::<String, SymbolTableEntry>::new(),
        };

        instance
    }
    
    pub fn insert(&mut self, varname: String, symbol_table_entry: SymbolTableEntry) {
        // // DEBUG
        // println!("Inserting '{}' with type {:?}", varname, symbol_table_entry);
        self.identifier_type_map.insert(varname, symbol_table_entry);
    }

    pub fn contains(&mut self, varname: &String) -> bool {
        self.identifier_type_map.contains_key(varname)
    }

    pub fn retrieve(&mut self, varname: &String) -> SymbolTableEntry {
        self.identifier_type_map.get(varname).unwrap().clone()
    }

}