use std::fmt;

use std::collections::{BTreeMap, HashMap};

use crate::preprocessor::expression_parser::ExpressionParser;

pub struct DefineSymbol {
    pub name: String,
    pub formal_parameter_map_index_to_param: BTreeMap::<usize, String>,
    pub formal_parameter_map_param_to_index: BTreeMap::<String, usize>,
    pub definition: String,
    pub has_defined_value: bool,
    pub expression_parser: ExpressionParser, // stores the AST parsed form the definition in it's internal arena
}

impl DefineSymbol {
    pub fn new() -> DefineSymbol {
        DefineSymbol {
            name: String::from(""),
            formal_parameter_map_index_to_param: BTreeMap::<usize, String>::new(),
            formal_parameter_map_param_to_index: BTreeMap::<String, usize>::new(),
            definition: String::from(""),
            has_defined_value: true,
            expression_parser: ExpressionParser::new(),
        }
    }
}

impl fmt::Debug for DefineSymbol {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        write!(f, "{}", self.name);

        let mut count = 0;

        if self.formal_parameter_map_index_to_param.len() > 0 {
            write!(f, " (");
            for (key, value) in &self.formal_parameter_map_index_to_param {
                if count > 0 {
                    write!(f, ", ");
                }
                write!(f, "{}", value);
                count = count + 1;
            }
            write!(f, ")");
        }

        write!(f, " -> {}", self.definition);

        Ok(())
    }
}

impl fmt::Display for DefineSymbol {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        write!(f, "{}", self.name);

        let mut count = 0;

        if self.formal_parameter_map_index_to_param.len() > 0 {
            write!(f, " (");
            for (key, value) in &self.formal_parameter_map_index_to_param {
                if count > 0 {
                    write!(f, ", ");
                }
                write!(f, "{}", value);
                count = count + 1;
            }
            write!(f, ")");
        }

        write!(f, " -> {}", self.definition);

        Ok(())
    }
}