use std::fmt;

use std::collections::{BTreeMap, HashMap};

// DefineSymbol {
// 	    name: "SQUARE"
// 	    formal_parameter_map: { key: 0, value: x }
// 	    definition: "((x) * (x))"
// }
pub struct DefineSymbol {
    pub name: String,
    pub formal_parameter_map: BTreeMap::<usize, String>,
    pub definition: String,
}

impl DefineSymbol {
    pub fn new() -> DefineSymbol {
        DefineSymbol {
            name: String::from(""),
            formal_parameter_map: BTreeMap::<usize, String>::new(),
            definition: String::from(""),
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