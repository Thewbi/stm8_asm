use std::fmt;
use std::fmt::Display;
use std::fmt::Debug;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum RuleElement<T> {
    NonTerminal(T),
    Terminal(T),
    Epsilon,
    Dot,
    AcceptingStateTransition,
    Closure,
    Unused,
    Unknown
}

impl<T: Ord> Ord for RuleElement<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        
        match &self {
            RuleElement::<T>::NonTerminal(lhs_val) => {
                match &other {
                    RuleElement::<T>::NonTerminal(rhs_val) => {
                        lhs_val.cmp(rhs_val)
                    }
                    _ => {
                        panic!("test");
                    }
                }
            }
            _ => {
                panic!("test");
            }
        }
    }
}

impl<T: PartialOrd> PartialOrd for RuleElement<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        panic!("test");
    }
}

impl<T: Display> fmt::Debug for RuleElement<T> {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        // LHS
        match &self {
            RuleElement::NonTerminal(str_val) => {
                write!(f, "{}", str_val).expect("Write failed!");
            }
            RuleElement::Terminal(str_val) => {
                write!(f, "{}", str_val).expect("Write failed!");
            }
            RuleElement::Epsilon => {
                write!(f, "ϵ").expect("Write failed!");
            }
            RuleElement::Dot => {
                write!(f, ".").expect("Write failed!");
            }
            RuleElement::AcceptingStateTransition => {
                write!(f, "$").expect("Write failed!");
            }
            RuleElement::Closure => {
                write!(f, "#").expect("Write failed!");
            }
            RuleElement::Unknown => {
                write!(f, "UNKNOWN").expect("Write failed!");
            }
            RuleElement::Unused => {
                // nop, do not display unused
            }
        }

        Ok(())
    }
}

#[derive(Clone, Eq, Hash)]
pub struct Rule<T> {
    pub id: usize,
    pub original_id: usize,
    pub dot_idx: usize,
    pub lhs: RuleElement<T>,
    pub rhs: Vec::<RuleElement<T>>,
    pub lookahead: Vec::<RuleElement<T>>,
    pub channels: Vec::<usize>,
}

impl<T: Debug + std::fmt::Display> Rule<T> {

    pub fn new(id: usize) -> Self {
        Rule {
            id: id,
            original_id: id,
            dot_idx: 0,
            lhs: RuleElement::<T>::Unknown,
            rhs: Vec::<RuleElement<T>>::new(),
            lookahead: Vec::<RuleElement<T>>::new(),
            channels: Vec::<usize>::new(),
        }
    }

    pub fn print_rule_simple(&self) {

        // Rule ID
        print!("[{:?}, orig:{:?}] ", &self.id, &self.original_id);

        // LHS
        print!("{:?}", &self.lhs);

        print!(" -> ");

        // RHS
        let mut index: usize = 0;
        for symbol in &self.rhs {

            match &symbol {
                RuleElement::NonTerminal(str_val) => {
                    if index > 0 {
                        print!(" ");
                    }
                    print!("{str_val}");
                    index = index + 1;
                }
                RuleElement::Terminal(str_val) => {
                    if index > 0 {
                        print!(" ");
                    }
                    print!("{str_val}");
                    index = index + 1;
                }
                RuleElement::Epsilon => {
                    if index > 0 {
                        print!(" ");
                    }
                    print!("ϵ");
                    index = index + 1;
                }
                RuleElement::Dot => {
                    if index > 0 {
                        print!(" ");
                    }
                    print!(".");
                    index = index + 1;
                }
                RuleElement::AcceptingStateTransition => {
                    if index > 0 {
                        print!(" ");
                    }
                    print!("$");
                    index = index + 1;
                }
                RuleElement::Closure => {
                    if index > 0 {
                        print!(" ");
                    }
                    print!("#");
                    index = index + 1;
                }
                RuleElement::Unknown => {
                    if index > 0 {
                        print!(" ");
                    }
                    print!("UNKNOWN");
                    index = index + 1;
                }
                RuleElement::Unused => {
                    // nop, do not display unused
                }
            }
        }

        // println!("");
    }
}

impl<T: std::cmp::PartialEq> PartialEq<Rule<T>> for Rule<T> {

    // Rule equality is defined over 
    // - LHS 
    // - RHS, same amount, same order of elements
    // - dot marker, located at same index
    //
    // Not defined over id!
    fn eq(&self, other: &Rule<T>) -> bool {

        if self.rhs.len() != other.rhs.len() {
            return false;
        }

        // https://stackoverflow.com/questions/29504514/whats-the-best-way-to-compare-2-vectors-or-strings-element-by-element

        // first zip to compare element by element, the result is the amount of matching elements
        let matching = self.rhs.iter().zip(&other.rhs).filter(|&(a, b)| a == b).count();

        // if lhs matches and all elements in rhs match and the dot idx is at the same spot, the rules are equal
        self.lhs == other.lhs && matching == self.rhs.len() && self.dot_idx == other.dot_idx
    }
}

impl<T: Display> fmt::Debug for Rule<T> {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        // // orig id
        // write!(f, "orig_id: {:?} ", &self.original_id).expect("Write failed!");

        // LHS
        write!(f, "{:?}", &self.lhs).expect("Write failed!");

        write!(f, " -> ").expect("Write failed!");

        // RHS
        let mut index: usize = 0;
        for symbol in &self.rhs {

            if index == self.dot_idx {
                write!(f, ".");
            }

            match &symbol {
                RuleElement::NonTerminal(str_val) => {
                    if index > 0 {
                        write!(f, " ");
                    }
                    write!(f, "{}", str_val).expect("Write failed!");
                    index = index + 1;
                }
                RuleElement::Terminal(str_val) => {
                    if index > 0 {
                        write!(f, " ");
                    }
                    write!(f, "{}", str_val).expect("Write failed!");
                    index = index + 1;
                }
                RuleElement::Epsilon => {
                    if index > 0 {
                        write!(f, " ");
                    }
                    write!(f, "ϵ").expect("Write failed!");
                    index = index + 1;
                }
                RuleElement::Dot => {
                    if index > 0 {
                        write!(f, " ");
                    }
                    write!(f, ".").expect("Write failed!");
                    index = index + 1;
                }
                RuleElement::AcceptingStateTransition => {
                    if index > 0 {
                        write!(f, " ");
                    }
                    write!(f, "$").expect("Write failed!");
                    index = index + 1;
                }
                RuleElement::Closure => {
                    if index > 0 {
                        write!(f, " ");
                    }
                    write!(f, "#").expect("Write failed!");
                    index = index + 1;
                }
                RuleElement::Unknown => {
                    if index > 0 {
                        write!(f, " ");
                    }
                    write!(f, "UNKNOWN").expect("Write failed!");
                    index = index + 1;
                }
                RuleElement::Unused => {
                    // nop, do not display unused
                }
            }
        }

        if index == self.dot_idx {
            write!(f, ".");
        }

        // lookahead
        write!(f, "     ");
        for symbol in &self.lookahead {
            write!(f, " / {:?}", &symbol).expect("Write failed!");
        }

        // channels
        if self.channels.len() > 0 {
            write!(f, "    channels: {:?}", &self.channels).expect("Write failed!");
        }

        Ok(())
    }
}

impl<T: Display> fmt::Display for Rule<T> {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        // LHS
        write!(f, "{:?}", &self.lhs).expect("Write failed!");

        write!(f, " -> ").expect("Write failed!");

        let mut index: usize = 0;
        for symbol in &self.rhs {

            if index == self.dot_idx {
                write!(f, ".");
            }

            match &symbol {
                RuleElement::NonTerminal(str_val) => {
                    if index > 0 {
                        write!(f, " ");
                    }
                    write!(f, "{}", str_val).expect("Write failed!");
                    index = index + 1;
                }
                RuleElement::Terminal(str_val) => {
                    if index > 0 {
                        write!(f, " ");
                    }
                    write!(f, "{}", str_val).expect("Write failed!");
                    index = index + 1;
                }
                RuleElement::Epsilon => {
                    if index > 0 {
                        write!(f, " ");
                    }
                    write!(f, "ϵ").expect("Write failed!");
                    index = index + 1;
                }
                RuleElement::Dot => {
                    if index > 0 {
                        write!(f, " ");
                    }
                    write!(f, ".").expect("Write failed!");
                    index = index + 1;
                }
                RuleElement::AcceptingStateTransition => {
                    if index > 0 {
                        write!(f, " ");
                    }
                    write!(f, "$").expect("Write failed!");
                    index = index + 1;
                }
                RuleElement::Closure => {
                    if index > 0 {
                        write!(f, " ");
                    }
                    write!(f, "#").expect("Write failed!");
                    index = index + 1;
                }
                RuleElement::Unknown => {
                    if index > 0 {
                        write!(f, " ");
                    }
                    write!(f, "UNKNOWN").expect("Write failed!");
                    index = index + 1;
                }
                RuleElement::Unused => {
                    // nop, do not display unused
                }
            }
        }

        if index == self.dot_idx {
            write!(f, ".");
        }

        // lookahead
        write!(f, "     ");
        for symbol in &self.lookahead {
            write!(f, " / {:?}", &symbol).expect("Write failed!");
        }

        // channels
        if self.channels.len() > 0 {
            write!(f, "    channels: {:?}", &self.channels).expect("Write failed!");
        }

        Ok(())
    }
}