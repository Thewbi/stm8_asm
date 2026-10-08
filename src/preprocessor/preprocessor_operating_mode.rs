use std::fmt;

// Here is the StateMachine of the Preprocessor:
// https://dreampuf.github.io/GraphvizOnline
//
// digraph {
//     0 [label="NORMAL"]
//     1 [label="IGNORE_SINGLE_LINE_COMMENT"]
//     2 [label="IGNORE_MULTI_LINE_COMMENT"]
//     3 [label="DEFINE_WAITING_FOR_NAME"]
//     4 [label="DEFINE_IFC_OR_DEFINITION"]
//     5 [label="DEFINE_IFC"]
//     6 [label="DEFINE_DEFINITION"]
//
//     // single line comment and back to normal
//     0 -> 1 [label="//"]
//     1 -> 0 [label="\\n"]
//
//     // multi line comment and back to normal
//     0 -> 2 [label="/*"]
//     2 -> 0 [label="*/"]
//
//     // a #define PPI starts
//     0 -> 3 [label="#define"]
//
//     // #define symbol name found
//     3 -> 4 [label="String"]
//
//     // #define: make decision if there is a parameter list or not
//     4 -> 5 [label="("]
//     4 -> 6 [label="String"]
//
//     // #define: process parameter list
//     5 -> 5 [label=","]
//     5 -> 5 [label="String"]
//     5 -> 6 [label="("]
//
//     // #define: push all text into the defintion buffer
//     6 -> 6 [label="String"]
//
//     // #define: definition is over. Back to normal
//     6 -> 0 [label="\\n"]
// }

pub enum PreprocessorOperatingMode {
    Normal,
    IgnoreSingleLineComment,
    IgnoreMultiLineComment,
    DefineWaitingForName,
    DefineIfcOrDefinition,
    DefineIfc,
    DefineDefinition,
}

impl fmt::Display for PreprocessorOperatingMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            PreprocessorOperatingMode::Normal => { write!(f, "NORMAL").expect("Write failed!"); }
            PreprocessorOperatingMode::IgnoreSingleLineComment => { write!(f, "IGNORE_SINGLE_LINE_COMMENT").expect("Write failed!"); }
            PreprocessorOperatingMode::IgnoreMultiLineComment => { write!(f, "IGNORE_MULTI_LINE_COMMENT").expect("Write failed!"); }
            PreprocessorOperatingMode::DefineWaitingForName => { write!(f, "DEFINE_WAITING_FOR_NAME").expect("Write failed!"); }
            PreprocessorOperatingMode::DefineIfcOrDefinition => { write!(f, "DEFINE_IFC_OR_DEFINITION").expect("Write failed!"); }
            PreprocessorOperatingMode::DefineIfc => { write!(f, "DEFINE_IFC").expect("Write failed!"); }
            PreprocessorOperatingMode::DefineDefinition => { write!(f, "DEFINE_DEFINITION").expect("Write failed!"); }
        }

        Ok(())
    }
}