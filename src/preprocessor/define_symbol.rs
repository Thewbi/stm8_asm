use std::fmt;

use std::collections::{BTreeMap, HashMap};

use crate::preprocessor::expression_parser::ExpressionParser;

// DefineSymbol {
// 	    name: "SQUARE"
// 	    formal_parameter_map: { key: 0, value: x }
// 	    definition: "((x) * (x))"
// }
pub struct DefineSymbol {
    pub name: String,
    pub formal_parameter_map: BTreeMap::<usize, String>,
    pub definition: String,
    pub expression_parser: ExpressionParser, // stores the AST parsed form the definition in it's internal arena
}

impl DefineSymbol {
    pub fn new() -> DefineSymbol {
        DefineSymbol {
            name: String::from(""),
            formal_parameter_map: BTreeMap::<usize, String>::new(),
            definition: String::from(""),
            expression_parser: ExpressionParser::new(),
        }
    }
}

impl fmt::Display for DefineSymbol {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        write!(f, "{}", self.name);

        let mut count = 0;

        if self.formal_parameter_map.len() > 0 {
            write!(f, " (");
            for (key, value) in &self.formal_parameter_map {
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