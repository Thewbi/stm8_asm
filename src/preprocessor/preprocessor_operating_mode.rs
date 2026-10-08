use std::fmt;


pub enum PreprocessorOperatingMode {
    NORMAL,
    IGNORE_SINGLE_LINE_COMMENT,
    IGNORE_MULTI_LINE_COMMENT,
    DEFINE_WAITING_FOR_NAME,
    DEFINE_IFC_OR_DEFINITION,
    DEFINE_IFC,
    DEFINE_DEFINITION,
}

impl fmt::Display for PreprocessorOperatingMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            PreprocessorOperatingMode::NORMAL => { write!(f, "NORMAL").expect("Write failed!"); }
            PreprocessorOperatingMode::IGNORE_SINGLE_LINE_COMMENT => { write!(f, "IGNORE_SINGLE_LINE_COMMENT").expect("Write failed!"); }
            PreprocessorOperatingMode::IGNORE_MULTI_LINE_COMMENT => { write!(f, "IGNORE_MULTI_LINE_COMMENT").expect("Write failed!"); }
            PreprocessorOperatingMode::DEFINE_WAITING_FOR_NAME => { write!(f, "DEFINE_WAITING_FOR_NAME").expect("Write failed!"); }
            PreprocessorOperatingMode::DEFINE_IFC_OR_DEFINITION => { write!(f, "DEFINE_IFC_OR_DEFINITION").expect("Write failed!"); }
            PreprocessorOperatingMode::DEFINE_IFC => { write!(f, "DEFINE_IFC").expect("Write failed!"); }
            PreprocessorOperatingMode::DEFINE_DEFINITION => { write!(f, "DEFINE_DEFINITION").expect("Write failed!"); }
        }

        Ok(())
    }
}