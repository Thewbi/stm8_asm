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

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PreprocessorOperatingMode {

    // normal operation, no preprocessor instruction
    Normal,

    // comment
    IgnoreSingleLineComment,
    IgnoreMultiLineComment,

    // if-defined, if-not-defined
    Ifdef,
    Ifndef,

    // #define Preprocessor Instruction (PPI)
    DefineWaitingForName,
    DefineIfcOrDefinition,
    DefineIfc,
    DefineDefinition,

    // #undef Preprocessor Instruction (PPI)
    Undefine,

    // #if Preprocessor Instruction (PPI)
    If,
    Elif,
    Else,
    Endif,

    // #pragma,
    Pragma
}

impl fmt::Display for PreprocessorOperatingMode {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {

            PreprocessorOperatingMode::Normal => { write!(f, "NORMAL").expect("Write failed!"); }

            PreprocessorOperatingMode::IgnoreSingleLineComment => { write!(f, "IGNORE_SINGLE_LINE_COMMENT").expect("Write failed!"); }
            PreprocessorOperatingMode::IgnoreMultiLineComment => { write!(f, "IGNORE_MULTI_LINE_COMMENT").expect("Write failed!"); }

            PreprocessorOperatingMode::Ifdef => { write!(f, "IF DEFINED").expect("Write failed!"); }
            PreprocessorOperatingMode::Ifndef => { write!(f, "IF NOT DEFINED").expect("Write failed!"); }

            PreprocessorOperatingMode::DefineWaitingForName => { write!(f, "DEFINE_WAITING_FOR_NAME").expect("Write failed!"); }
            PreprocessorOperatingMode::DefineIfcOrDefinition => { write!(f, "DEFINE_IFC_OR_DEFINITION").expect("Write failed!"); }
            PreprocessorOperatingMode::DefineIfc => { write!(f, "DEFINE_IFC").expect("Write failed!"); }
            PreprocessorOperatingMode::DefineDefinition => { write!(f, "DEFINE_DEFINITION").expect("Write failed!"); }

            PreprocessorOperatingMode::Undefine => { write!(f, "Undefine").expect("Write failed!"); }

            PreprocessorOperatingMode::If => { write!(f, "IF").expect("Write failed!"); }
            PreprocessorOperatingMode::Elif => { write!(f, "ELIF").expect("Write failed!"); }
            PreprocessorOperatingMode::Else => { write!(f, "ELSE").expect("Write failed!"); }
            PreprocessorOperatingMode::Endif => { write!(f, "ENDIF").expect("Write failed!"); }

            PreprocessorOperatingMode::Pragma => { write!(f, "PRAGMA").expect("Write failed!"); }
        }

        Ok(())
    }
}