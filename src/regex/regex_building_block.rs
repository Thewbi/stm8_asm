use std::fmt;
use std::fmt::Debug;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum RegexBuildingBlock {
    CharacterLiteral(char),
    Concatenation,
    CharacterClass(char, char),
    Repeat(u8, u8),
    Or,
    Not,
    OpeningBraces,
    ClosingBraces,
    ClosedBraces,
}

impl fmt::Debug for RegexBuildingBlock {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        match self {
            Self::Not => {
                write!(f, "^").expect("Write failed!");
                Ok(())
            }
            Self::CharacterLiteral(c) => {
                //f.debug_tuple("CharacterLiteral").field(c).finish(),
                match c {
                    '\n' => { write!(f, "\\n").expect("Write failed!"); }
                    '\r' => { write!(f, "\\r").expect("Write failed!"); }
                    '\t' => { write!(f, "\\t").expect("Write failed!"); }
                    //r"\*" => { write!(f, "\\*").expect("Write failed!"); }
                    // '\s' => { write!(f, "\\s").expect("Write failed!"); }
                    _ => { write!(f, "{}", c).expect("Write failed!"); }
                }
                Ok(())
            }
            Self::Concatenation => write!(f, "#"),
            Self::CharacterClass(start, end) => {
                //f.debug_tuple("CharacterClass").field(start).field(end).finish(),
                write!(f, "[{}-{}]", start, end).expect("Write failed!");
                Ok(())
            }
            Self::Repeat(min, max) => {
                //f.debug_tuple("Repeat").field(min).field(max).finish(),
                if *min == 0 && *max == 1 {
                    write!(f, "?").expect("Write failed!");
                } else if *min == 0 && *max == std::u8::MAX {
                    write!(f, "*").expect("Write failed!");
                } else if *min == 1 && *max == std::u8::MAX {
                    write!(f, "+").expect("Write failed!");
                } else if *min == *max {
                    write!(f, "{{{}}}", *min).expect("Write failed!");
                } else {
                    write!(f, "{{{},{}}}", *min, *max).expect("Write failed!");
                }
                Ok(())
            }
            Self::Or => write!(f, "|"),

            Self::OpeningBraces => write!(f, "("),
            Self::ClosingBraces => write!(f, ")"),

            //Self::ClosedBraces => write!(f, "()"),
            Self::ClosedBraces => write!(f, ""),
        }
    }
}