use std::fs::File;
use std::io::{BufWriter, Write};
use std::io::BufReader;
use std::io::BufRead;

use std::fmt;
use std::fmt::Debug;

use std::collections::{HashMap, HashSet, BTreeSet};
use std::hash::Hash;
use std::{
    sync::atomic::{AtomicUsize, Ordering}
};

use crate::regex::infix_postfix_converter::InfixPostfixConverter;
use crate::regex::regex_building_block::RegexBuildingBlock;
use crate::regex::arena::Arena;
use crate::regex::arena::NodeId;
use crate::regex::arena::Node;

// https://stackoverflow.com/questions/32935808/generate-sequential-ids-for-each-instance-of-a-struct
static STATE_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct State {
    id: usize,
    copy_id: usize,
    pub token_id: usize,
    pub token_name: String,
    start_state: bool,
    end_state: bool,
    trap_state: bool,
}

impl State {
    pub fn new(id: usize) -> Self {
        State {
            id: id,
            copy_id: 0,
            token_id: 0,
            token_name: String::from(""),
            start_state: false,
            end_state: false,
            trap_state: false,
        }
    }
}

pub trait StateTrait {
    fn get_id(&self) -> usize;
    fn set_start_state(&mut self, start_state: bool);
    fn is_start_state(&mut self) -> bool;
    fn set_end_state(&mut self, end_state: bool);
    fn is_end_state(&mut self) -> bool;
    fn is_trap_state(&mut self) -> bool;
}

impl StateTrait for State {
    fn get_id(&self) -> usize {
        self.id
    }
    fn set_start_state(&mut self, start_state: bool) {
        self.start_state = start_state;
    }
    fn is_start_state(&mut self) -> bool {
        self.start_state
    }
    fn set_end_state(&mut self, end_state: bool) {
        self.end_state = end_state;
    }
    fn is_end_state(&mut self) -> bool {
        self.end_state
    }
    fn is_trap_state(&mut self) -> bool {
        self.trap_state
    }
}

// Represents the alphabet, including Epsilon as a variant
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Input<T> {
    Symbol(T),
    Epsilon,
}

// Transition table type: (CurrentState, Input) -> Set of NextStates
pub type TransitionTable<S, T> = HashMap<(S, Input<T>), HashSet<S>>;

#[derive(Clone)]
pub struct EpsilonNfa<S, T>
where
    S: Eq + Hash + Clone,
    T: Eq + Hash + Clone,
{
    pub states: HashMap<usize, S>,
    pub alphabet: HashSet<T>,
    pub transitions: TransitionTable<usize, T>,
    pub start_state_id: usize,
    pub accept_states: HashSet<S>,
}

impl<S, T> EpsilonNfa<S, T>
where
    S: Eq + Hash + Clone + StateTrait,
    T: Eq + Hash + Clone,
{
    pub fn new(start_state: S) -> Self {

        let mut result = Self {
            states: HashMap::new(),
            alphabet: HashSet::new(),
            transitions: HashMap::new(),
            start_state_id: start_state.get_id(),
            accept_states: HashSet::new(),
        };
        
        let mut s = start_state.clone();
        s.set_start_state(true);
        result.states.insert(s.get_id(), s);

        result
    }

    pub fn clear(&mut self) {
        self.states.clear();
        self.alphabet.clear();
        self.transitions.clear();
        self.start_state_id = 0;
        self.accept_states.clear();
    }

    pub fn add_transition(&mut self, start_id: usize, input: Input<T>, end_id: usize) {
        self.transitions
             .entry((start_id, input))
             .or_insert_with(HashSet::new)
             .insert(end_id);
    }

    pub fn add_state(&mut self, state: S) -> usize {
        let id = state.get_id();
        self.states.insert(id, state);
        id
    }

    pub fn set_start_state(&mut self, state_id: usize, is_start: bool) {
        if let Some(val) = self.states.get_mut(&state_id) { val.set_start_state(is_start); };
        if is_start {
            self.start_state_id = state_id;
        }
    }

    pub fn set_end_state(&mut self, state_id: usize, is_end: bool) {
        if let Some(val) = self.states.get_mut(&state_id) { val.set_end_state(is_end); };
    }

    pub fn is_end_state(&mut self, state_id: usize) -> bool {
        if let Some(val) = self.states.get_mut(&state_id) { return val.is_end_state() };
        false
    }

    pub fn is_trap_state(&mut self, state_id: usize) -> bool {
        if let Some(val) = self.states.get_mut(&state_id) { return val.is_trap_state() };
        false
    }

    pub fn is_empty(&mut self) -> bool {
        self.states.len() == 1
    }
}

//
// Format:
// 
// The ENFA is characterized by the following objects:
// - (S) States
// - (T) Transitions
// - (ALPHA) Alphabet (ignored for now)
// - (S_ID) A single start state id
// - (ACC) Set of acceptance states
//
// Each line is constructed of cells, separated by semicolon.
// The first cell of each line contains a code that identifies the type of object that the row describes.
// The code is given in the listing of objects above (S, T, ALPHA, S_ID, ACC).
//
// The format for a state object (S) is:
// S;<state_id>;<token_id>;<token_name>;<start_state>;<end_state>;<trap_state>
// 1st cell <code> - Contains the code for states "S"
// 2nd cell <state_id> - Contains the id of this state
// 3rd cell <token_id> - Contains the token id which will be detected when following this state
// 4th cell <token_name> - Human Readable Name of token. Also used to interact with the parser. This name is used in parser rules.
// 5th cell <start_state> - 1 if this state is a start state (true), 0 if not (false)
// 6th cell <end_state> - 1 if this state is a end state (true), 0 if not (false)
// 7th cell <trap_state> - 1 if this state is a trap state (true), 0 if not (false)
//
// The format of the start state id (S_ID) is:
// S_ID;<start_state_id>
// 1st cell <code> - Contains the code for start state id "S_ID"
// 2nd cell <start_state_id> - Contains the id of the start state
//
// The format of the transitions (T) is:
// T;<start_state_id>;<symbol>;<target_state_id_0>;<target_state_id_1>;...;<target_state_id_n>
// 1st cell <code> - Contains the code for a transition "T"
// 2nd cell <start_state_id> - Contains the id of the start state
// 3rd cell <symbol> - The symbol to transition with
// 4th cell - All the ids of the target states to transition to
// 5th cell - All the ids of the target states to transition to
// ... cell - ...
// nth cell - All the ids of the target states to transition to
pub fn enfa_serialize(enfa: &mut EpsilonNfa<State, RegexBuildingBlock>, filename: &str) {

    let mut string_buffer = String::new();

    // 1. Create or overwrite the file
    let file = File::create(filename).expect("Creating file failed!");
    
    // 2. Wrap the file in a BufWriter
    let mut writer = BufWriter::new(file);

    // iterate over states
    //
    // Example Debug Format when outputting a state to the console:
    // 1200 State { id: 1200, copy_id: 0, token_id: 37, token_name: "PLUS", start_state: false, end_state: true, trap_state: false }
    for (state_id, state) in enfa.states.iter_mut() {

        println!("{:?} {:?}", state_id, state);

        // code
        string_buffer.push_str("S;");

        // state id
        string_buffer.push_str(format!("{};", state_id).as_str());

        // token id
        string_buffer.push_str(format!("{};", state.token_id).as_str());

        // token name
        string_buffer.push_str(&state.token_name.clone());
        string_buffer.push_str(";");

        // start state flag
        if state.start_state {
            string_buffer.push_str("1;");
        } else {
            string_buffer.push_str("0;");
        }

        // end state flag
        if state.end_state {
            string_buffer.push_str("1;");
        } else {
            string_buffer.push_str("0;");
        }

        // trap state flag
        if state.trap_state {
            string_buffer.push_str("1");
        } else {
            string_buffer.push_str("0");
        }

        // 3. Write data to file
        write!(writer, "{}\n", string_buffer);

        string_buffer.clear();
    }

    // alphabet -- ignored for now
    let output_alphabet = false;
    if output_alphabet {
        for regex_building_block in enfa.alphabet.iter() {
            println!("{:?}", regex_building_block);
        }
    }

    // transitions
    //
    // Example:
    // (1273, Symbol(w)) {1182}
    for (transition_id, transition) in enfa.transitions.iter() {
        // println!("{:?} {:?}", transition_id, transition);

        // code
        string_buffer.push_str("T;");

        // start state id
        string_buffer.push_str(format!("{};", transition_id.0).as_str());

        // alphabet symbol
        match transition_id.1 {
            Input::Symbol(RegexBuildingBlock::CharacterLiteral(char_value)) => {
                if char_value == ';' {
                    string_buffer.push_str("SEMICOLON");
                } else {
                    string_buffer.push_str(format!("{:?}", char_value).as_str());
                }
            }
            _ => {
                todo!("Not implemented yet!");
            }
        }

        // all target states
        for target_state_id in transition.iter() {
            // println!("{:?}", target_state_id);
            string_buffer.push_str(format!(";{}", target_state_id).as_str());
        }

        // 3. Write data to file
        write!(writer, "{}\n", string_buffer);

        string_buffer.clear();
    }

    // start state's id
    //
    // Example:
    // 1168
    println!("StartStateId: {:?}", enfa.start_state_id);

    string_buffer.push_str("S_ID;");
    string_buffer.push_str(format!("{}", enfa.start_state_id).as_str());
    write!(writer, "{}\n", string_buffer);
    string_buffer.clear();
    
    // states for acceptance -- ignored for now
    let output_acceptance_states = false;
    if output_acceptance_states {
        for accept_state in enfa.accept_states.iter() {
            println!("{:?}", accept_state);
        }
    }

    // 4. Explicitly flush the remaining data to disk
    writer.flush().expect("flush failed!");
}

// File-Format: See comment on function enfa_serialize()
pub fn enfa_deserialize(enfa: &mut EpsilonNfa<State, RegexBuildingBlock>, filename: &str) {

    let debug: bool = false;

    enfa.clear();

    let file = File::open(filename).expect("Reading file failed!");
    let reader = BufReader::new(file);

    let mut state_count = 0;
    let mut transition_count = 0;
    let mut current_line_index = 1;

    // DEBUG - output the lines read from the parse table file
    for line_result in reader.lines() {

        if let Ok(line) = line_result {

            // DEBUG
            // println!("{:?}", line);

            let row_split: Vec<_> = line.split(';').collect();

            // DEBUG
            // println!("{:?}", row_split);

            let type_code = row_split[0];

            match type_code {

                "S" => {
                    // struct state is defined in enfa.rs (top of the file)
                    let mut state: State = State::new(usize::from_str_radix(row_split[1], 10).expect("Number conversion failed!"));
                    state.token_id = usize::from_str_radix(row_split[2], 10).expect("Number conversion failed!");
                    state.token_name = String::from(row_split[3]);
                    state.start_state = row_split[4] == "1";
                    state.end_state = row_split[5] == "1";
                    state.trap_state = row_split[6] == "1";

                    enfa.add_state(state);

                    state_count = state_count + 1;
                }

                "T" => {
                    let string_val = row_split[2].trim_matches('\'');

                    let mut char_val = ' ';
                    if string_val == "SEMICOLON" {
                        char_val = ';';
                    } 
                    else if string_val.starts_with("\\\\") {
                        char_val = '\\';
                    } 
                    else if string_val.starts_with("\\r") {
                        char_val = '\r';
                    }
                    else if string_val.starts_with("\\n") {
                        char_val = '\n';
                    }
                    else {
                        char_val = string_val.chars().next().unwrap();
                    }

                    let start_state_id:usize = usize::from_str_radix(row_split[1], 10).expect("Number conversion failed!");
                    let end_state_id:usize = usize::from_str_radix(row_split[3], 10).expect("Number conversion failed!");

                    // // DEBUG
                    // if start_state_id == 1246 && end_state_id == 1171 {
                    //     println!("{} {:?} {}", current_line_index, line, char_val);
                    //     println!("test");
                    // }

                    let before = enfa.transitions.len();

                    enfa.add_transition(start_state_id, Input::Symbol(RegexBuildingBlock::CharacterLiteral(char_val)), end_state_id);

                    let after = enfa.transitions.len();

                    // error case
                    if before == after {
                        println!("{:?}", line);
                        println!("{}", string_val);
                        println!("Same");

                        panic!("DFA transition did not increase overall amount of transitions! This cannot be! Some characters are interpreted incorrectly! FIX ME!")
                    }

                    transition_count = transition_count + 1;
                }

                "S_ID" => {
                    enfa.start_state_id = usize::from_str_radix(row_split[1], 10).expect("Number conversion failed!");
                }

                _ => {
                    todo!();
                }
            }
        }

        current_line_index = current_line_index + 1;
    }

    if debug {
        println!("state_count: {}", state_count);
    }

    if enfa.states.len() != state_count {
        panic!("Load failed! State count does not match!");
    }

    if debug {
        println!("transition_count: {}", transition_count);
    }

    if enfa.transitions.len() != transition_count {
        panic!("Load failed! Transition count does not match!");
    }

    if debug {
        println!("end");
    }
}

pub fn enfa_to_dot_directed_graph(enfa: &mut EpsilonNfa<State, RegexBuildingBlock>, filename: &str) {

    // 1. Create or overwrite the file
    let file = File::create(filename).expect("Create file failed!");
    
    // 2. Wrap the file in a BufWriter
    let mut writer = BufWriter::new(file);

    write!(writer, "{}", "// https://dreampuf.github.io/GraphvizOnline\n\n");

    // digraph = directed graph
    write!(writer, "{}", "digraph {\n");

    // start pointer
    write!(writer, "\tstart_pointer [label= \"\", shape=none,height=.0,width=.0]\n");
    write!(writer, "\tstart_pointer -> {};\n", &enfa.start_state_id);

    // iterate over states
    for (state_id, state) in enfa.states.iter_mut() {

        // mark endstates
        if state.end_state {
            write!(writer, "\t{}[shape=doublecircle];\n", state_id);
        }

        // if this state is an accepting state for a token, mark it in the dot file
        if state.token_id != 0 {
            write!(writer, "\t{}[label=\"{}, Token: {}\"];\n", state_id, state_id, state.token_id);
        }
    }

    // iterate over transitions
    // ((S, Input<T>), HashSet<S>)
    //
    // a transition says: from start_state_id with symbol 'input_symbol', transition to all states in end_state_id_set
    for ((start_state_id, input_symbol), end_state_id_set) in enfa.transitions.iter_mut() {

        for end_state_id in end_state_id_set.iter() {

            write!(writer, "\t{}", start_state_id);
            write!(writer, " -> ");
            write!(writer, "{}", end_state_id);

            if *input_symbol == Input::Symbol(RegexBuildingBlock::CharacterLiteral('"')) {
                write!(writer, "[label=\"Symbol(quotes)\"]");
            } else {
                write!(writer, "[label=\"{:?}\"]", input_symbol);
            }
            // write!(writer, "[label=\"{:?}\"]", input_symbol);

            write!(writer, ";");
            write!(writer, "\n");
        }
    }

    write!(writer, "{}", "}");

    // 4. Explicitly flush the remaining data to disk
    writer.flush().expect("flush failed!");
}

#[derive(Clone)]
pub struct Fragment {
    pub enfa: EpsilonNfa::<State, RegexBuildingBlock>,
    pub symbol: RegexBuildingBlock,
    pub start_id: usize,
    pub end_id: usize,
}

impl Fragment {
    pub fn new(regex_building_block: RegexBuildingBlock) -> Self {

        let mut start_state = State::new(STATE_COUNTER.fetch_add(1, Ordering::SeqCst));
        start_state.start_state = true;

        Self {
            enfa: EpsilonNfa::<State, RegexBuildingBlock>::new(start_state.clone()),
            symbol: regex_building_block,
            start_id: start_state.id,
            end_id: start_state.id,
        }
    }
}

pub struct FragmentStack {
    pub stack: Vec<Fragment>,
}

impl FragmentStack {

    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
        }
    }

    pub fn reset(&mut self) {
        self.stack.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.stack.len() == 0
    }
}

// this is the glue code that interfaces the postfix regex tree with the eNFA construction
pub fn recurse_postfix_build_fragment_stack(arena: &Arena<RegexBuildingBlock>, 
    parent_node_id: &NodeId, 
    fragment_stack: &mut FragmentStack, 
    alphabet: &mut HashSet<RegexBuildingBlock>) {

    // postfix notation means to process both children firts, then the node last
    let parent_node: &Node<RegexBuildingBlock> = &arena.nodes[parent_node_id.index];

    match &parent_node.left {
        Some(_) => {
            recurse_postfix_build_fragment_stack(arena, parent_node.left.as_ref().unwrap(), fragment_stack, alphabet);
        }
        None => {
        }
    }

    match &parent_node.right {
        Some(_) => {
            recurse_postfix_build_fragment_stack(arena, parent_node.right.as_ref().unwrap(), fragment_stack, alphabet);
        }
        None => {
        }
    }

    // DEBUG
    // println!("'{:?}'", parent_node.data);

    match parent_node.data {

        // unescaped for processing, the special characters have to be escaped again for output
        RegexBuildingBlock::CharacterLiteral(c) => {
            add_character_literal(fragment_stack, RegexBuildingBlock::CharacterLiteral(c), alphabet);
        }

        // #
        RegexBuildingBlock::Concatenation => {
            add_concatenation(fragment_stack);
        }

        // |
        RegexBuildingBlock::Or => {
            add_or(fragment_stack);
        }

        // ?
        RegexBuildingBlock::Repeat(0, 1) => {
            add_repeat_zero_or_one(fragment_stack);
        }

        // *
        RegexBuildingBlock::Repeat(0, std::u8::MAX) => {
            add_repeat_zero_or_more(fragment_stack);
        }

        // +
        RegexBuildingBlock::Repeat(1, _) => {
            add_repeat_one_or_more(fragment_stack);
        }

        // )
        RegexBuildingBlock::ClosedBraces => {
            // nop
        }

        // ^
        RegexBuildingBlock::Not => {
            add_not_single_character_interpretation(fragment_stack, alphabet);
        }

        _ => {
            panic!("[recurse_postfix_build_fragment_stack] Unexpected Data: {:?}", parent_node.data);
        }
    }
}

pub fn add_character_literal(fragment_stack: &mut FragmentStack, regex_building_block: RegexBuildingBlock, alphabet: &mut HashSet<RegexBuildingBlock>) {

    alphabet.insert(regex_building_block);

    if fragment_stack.is_empty() {

        // fragment stack is empty

        let mut fragment = Fragment::new(regex_building_block);
    
        // create a new state and insert it into the new fragment's automaton so that this 
        // automaton can accept the character literal directly
        let mut another_state = State::new(STATE_COUNTER.fetch_add(1, Ordering::SeqCst));
        another_state.start_state = false;
        another_state.end_state = false;

        // add state into automaton
        let another_state_id = fragment.enfa.add_state(another_state);

        // build a transition between the automaton's start state and the newly created state for the character literal
        fragment.enfa.add_transition(fragment.enfa.start_state_id, Input::Symbol(regex_building_block), another_state_id);

        // the end state of the fragment is the newly created state
        fragment.end_id = another_state_id;
    
        // insert the first fragment into the stack. It now has a single transition for the character literal
        fragment_stack.stack.push(fragment);

    } else {

        // just buffer a literal without constructing an automaton from it
        // because we do not know if + or # follows
        // in case of # a constructed automaton is a waste of processing time
        // in case of + a new complex automaton is required
        fragment_stack.stack.push(Fragment::new(regex_building_block));
    }
}

// # Concatenation
fn add_concatenation(fragment_stack: &mut FragmentStack) {

    // if enfa.is_empty() {
    //     panic!("Cannot add concatenation to empty automaton!");
    // }
    // if fragment_stack.stack.len() < 2 {
    //     panic!("Cannot perform concatenation! Not enough fragments available to concatenate!");
    // }

    // pop both top elements from the fragment stack to replace them by a fragment which concatenates both into one eNFA
    let mut top_fragment = fragment_stack.stack.pop().unwrap();

    // check if the top fragment contains an atomic regex_building_block (character_literal, character_class)
    // or if the top fragment contains a complex graph
    if top_fragment.start_id == top_fragment.end_id {

        // top fragment is atomic, directly add the symbol into the bottom fragment

        let mut bottom_fragment = fragment_stack.stack.pop().unwrap();

        if bottom_fragment.start_id == bottom_fragment.end_id {

            // panic!("bottom_fragment is atomic too!");

            // --> () -bottom-symbol-> () -top-symbol-> () -->

            let mut fragment = Fragment::new(RegexBuildingBlock::Concatenation);

            let mut sta_state = State::new(STATE_COUNTER.fetch_add(1, Ordering::SeqCst));
            sta_state.start_state = true;
            sta_state.end_state = false;
            let sta_state_id = fragment.enfa.add_state(sta_state);
    
            let mut mid_state = State::new(STATE_COUNTER.fetch_add(1, Ordering::SeqCst));
            mid_state.start_state = false;
            mid_state.end_state = false;
            let mid_state_id = fragment.enfa.add_state(mid_state);

            let mut end_state = State::new(STATE_COUNTER.fetch_add(1, Ordering::SeqCst));
            end_state.start_state = false;
            end_state.end_state = true;
            let end_state_id = fragment.enfa.add_state(end_state);

            fragment.enfa.add_transition(sta_state_id, Input::Symbol(bottom_fragment.symbol), mid_state_id);
            fragment.enfa.add_transition(mid_state_id, Input::Symbol(top_fragment.symbol), end_state_id);
            
            fragment.start_id = sta_state_id;
            fragment.enfa.start_state_id = sta_state_id;
            fragment.end_id = end_state_id;

            assert!(0 != fragment.end_id);
        
            fragment_stack.stack.push(fragment);

        } else {

            // extend the automaton by a new state
            let mut another_state = State::new(STATE_COUNTER.fetch_add(1, Ordering::SeqCst));
            another_state.start_state = false;
            another_state.end_state = false;
            let another_state_id = bottom_fragment.enfa.add_state(another_state);

            // extend the automaton by a transition to the new state
            bottom_fragment.enfa.add_transition(bottom_fragment.end_id, Input::Symbol(top_fragment.symbol), another_state_id);

            // push a combined fragment on top of the stack
            bottom_fragment.end_id = another_state_id;
            bottom_fragment.symbol = RegexBuildingBlock::Concatenation;

            assert!(0 != bottom_fragment.end_id);

            fragment_stack.stack.push(bottom_fragment);

        }

    } else {

        // top fragment is complex, combine two eNFAs

        let mut bottom_fragment = fragment_stack.stack.pop().unwrap();

        // DEBUG
        // enfa_to_dot_directed_graph(&mut top_fragment.enfa, "top_automaton.dot");

        // DEBUG
        // enfa_to_dot_directed_graph(&mut bottom_fragment.enfa, "bottom_automaton.dot");
        
        let top_end_id = top_fragment.end_id;

        // copy(dest, src)
        let result_touple = enfa_copy(&mut bottom_fragment.enfa, &mut top_fragment.enfa, top_end_id);

        // // DEBUG
        // enfa_to_dot_directed_graph(&mut bottom_fragment.enfa, "merged_automaton.dot");

        let copied_start_id = result_touple.0;
        let copied_end_id = result_touple.1;

        // build epsilon transition between bottom end and top start
        bottom_fragment.enfa.add_transition(bottom_fragment.end_id, Input::Epsilon, copied_start_id);

        bottom_fragment.end_id = copied_end_id;

        // // DEBUG
        // enfa_to_dot_directed_graph(&mut bottom_fragment.enfa, "merged_automaton.dot");

        assert!(0 != bottom_fragment.end_id);

        fragment_stack.stack.push(bottom_fragment);

    }
}

// | OR
fn add_or(fragment_stack: &mut FragmentStack) {

    // if enfa.is_empty() {
    //     panic!("Cannot apply OR operation to empty automaton!");
    // }
    // if fragment_stack.stack.len() < 2 {
    //     panic!("Cannot perform OR! Not enough fragments available to perform an OR operation!");
    // }

    // pop both top elements from the fragment stack to get rid of the buffered operand to the OR operator
    let mut top_fragment = fragment_stack.stack.pop().unwrap();

    // check if the top fragment contains an atomic regex_building_block (character_literal, character_class)
    // or if the top fragment contains a complex graph
    if top_fragment.start_id == top_fragment.end_id {

        // top fragment contains an atomic regex_building_block

        // pop bottom fragment
        let mut bottom_fragment = fragment_stack.stack.pop().unwrap();
        
        if bottom_fragment.start_id == bottom_fragment.end_id {

            // panic!();

            // the bottom_fragment's eNFA already contains a start state
            // add an end state:

            // end state 
            let mut end_state = State::new(STATE_COUNTER.fetch_add(1, Ordering::SeqCst));
            end_state.start_state = false;
            end_state.end_state = false;
            let end_state_id = bottom_fragment.enfa.add_state(end_state);

            bottom_fragment.enfa.add_transition(bottom_fragment.start_id, Input::Symbol(top_fragment.symbol), end_state_id);
            bottom_fragment.enfa.add_transition(bottom_fragment.start_id, Input::Symbol(bottom_fragment.symbol), end_state_id);

            bottom_fragment.end_id = end_state_id;

            // // DEBUG
            // enfa_to_dot_directed_graph(&mut bottom_fragment.enfa, "atomic_to_atomic_automaton.dot");

            fragment_stack.stack.push(bottom_fragment);

        } else {

            // simply extend the bottom automaton

            // extend the automaton by a transition parallel to the rest of the automaton to implement an or operation and assign the new symbol
            bottom_fragment.enfa.add_transition(bottom_fragment.start_id, Input::Symbol(top_fragment.symbol), bottom_fragment.end_id);

            fragment_stack.stack.push(bottom_fragment);

        }

    } else {
        
        // top fragment is complex, combine two eNFAs
        let mut bottom_fragment = fragment_stack.stack.pop().unwrap();
        let end_id = top_fragment.end_id;

        // copy the top fragment's eNFA into the bottom eNFA.
        // This will create nodes with new ids
        // The start end end state of the copied eNFA is returned as a touple
        let result_touple = enfa_copy(&mut bottom_fragment.enfa, &mut top_fragment.enfa, end_id);

        let copied_start_id = result_touple.0;
        let copied_end_id = result_touple.1;

        // the bottom enfa remains as is but the top fragment eNFA is put in parallel to the bottom eNFA
        // the parallel path forms the OR operation
        bottom_fragment.enfa.add_transition(bottom_fragment.start_id, Input::Epsilon, copied_start_id);
        bottom_fragment.enfa.add_transition(copied_end_id, Input::Epsilon, bottom_fragment.end_id);

        fragment_stack.stack.push(bottom_fragment);

    }
}

// ? QuestionMarc, Repeat[0, 1]
fn add_repeat_zero_or_one(fragment_stack: &mut FragmentStack) {

    let mut top_fragment = fragment_stack.stack.pop().unwrap();

    // check if the top fragment contains an atomic regex_building_block (character_literal, character_class)
    // or if the top fragment contains a complex graph
    if top_fragment.start_id == top_fragment.end_id {

        // atomic graph
        
        // extend the automaton by a new end state
        let mut new_end_state = State::new(STATE_COUNTER.fetch_add(1, Ordering::SeqCst));
        new_end_state.start_state = false;
        new_end_state.end_state = false;
        let new_end_state_id = top_fragment.enfa.add_state(new_end_state);

        // add epsilon transitions from the old start state to the new end state
        top_fragment.enfa.add_transition(top_fragment.start_id, Input::Epsilon, new_end_state_id);

        // add epsilon transitions from the old start state to the new end state
        top_fragment.enfa.add_transition(top_fragment.start_id, Input::Symbol(top_fragment.symbol), new_end_state_id);

        // modify and push copy
        top_fragment.end_id = new_end_state_id;
        fragment_stack.stack.push(top_fragment);

    } else {

        // complex graph

        let old_start_id = top_fragment.start_id;
        let old_end_id = top_fragment.end_id;

        // make the old start state a regular state
        top_fragment.enfa.set_start_state(top_fragment.enfa.start_state_id, false);

        // extend the automaton by a new start state
        let mut new_start_state = State::new(STATE_COUNTER.fetch_add(1, Ordering::SeqCst));
        new_start_state.start_state = true;
        new_start_state.end_state = false;
        let new_start_state_id = top_fragment.enfa.add_state(new_start_state);

        // make the new state a start state
        top_fragment.enfa.set_start_state(new_start_state_id, true);

        // add epsilon transitions from the new start state to the old start state
        top_fragment.enfa.add_transition(new_start_state_id, Input::Epsilon, old_start_id);

        // extend the automaton by a new end state
        let mut new_end_state = State::new(STATE_COUNTER.fetch_add(1, Ordering::SeqCst));
        new_end_state.start_state = false;
        new_end_state.end_state = false;
        let new_end_state_id = top_fragment.enfa.add_state(new_end_state);

        // go to the new end state after one iteration
        top_fragment.enfa.add_transition(old_end_id, Input::Epsilon, new_end_state_id);

        // extend the automaton by a transition from new start state to the new end state
        top_fragment.enfa.add_transition(top_fragment.start_id, Input::Epsilon, new_end_state_id);

        // modify and push copy
        top_fragment.start_id = new_start_state_id;
        top_fragment.end_id = new_end_state_id;
        fragment_stack.stack.push(top_fragment);

    }
}

// * Star, Asterisk, Repeat[0, std::u8::MAX]
fn add_repeat_zero_or_more(fragment_stack: &mut FragmentStack) {

    let mut top_fragment = fragment_stack.stack.pop().unwrap();

    // check if the top fragment contains an atomic regex_building_block (character_literal, character_class)
    // or if the top fragment contains a complex graph
    if top_fragment.start_id == top_fragment.end_id {

        // atomic graph
        
        // extend the automaton by a new end state
        let mut new_end_state = State::new(STATE_COUNTER.fetch_add(1, Ordering::SeqCst));
        new_end_state.start_state = false;
        new_end_state.end_state = false;
        let new_end_state_id = top_fragment.enfa.add_state(new_end_state);

        // add epsilon transitions from the old start state to the new end state
        top_fragment.enfa.add_transition(top_fragment.start_id, Input::Epsilon, new_end_state_id);

        // add epsilon transitions from the new end state to the old start state (reverse operation)
        top_fragment.enfa.add_transition(new_end_state_id, Input::Symbol(top_fragment.symbol), top_fragment.start_id);

        // modify and push copy
        top_fragment.end_id = new_end_state_id;
        fragment_stack.stack.push(top_fragment);

    } else {

        // complex graph

        let old_start_id = top_fragment.start_id;
        let old_end_id = top_fragment.end_id;

        // make the old start state a regular state
        top_fragment.enfa.set_start_state(top_fragment.enfa.start_state_id, false);

        // extend the automaton by a new start state
        let mut new_start_state = State::new(STATE_COUNTER.fetch_add(1, Ordering::SeqCst));
        new_start_state.start_state = true;
        new_start_state.end_state = false;
        let new_start_state_id = top_fragment.enfa.add_state(new_start_state);

        // make the new state a start state
        top_fragment.enfa.set_start_state(new_start_state_id, true);

        // add epsilon transitions from the new start state to the old start state
        top_fragment.enfa.add_transition(new_start_state_id, Input::Epsilon, old_start_id);

        // add epsilon transitions from the old end state to the old start state 
        // to construct an infinte loop through the old automaton
        top_fragment.enfa.add_transition(old_end_id, Input::Epsilon, old_start_id);

        // extend the automaton by a new end state
        let mut new_end_state = State::new(STATE_COUNTER.fetch_add(1, Ordering::SeqCst));
        new_end_state.start_state = false;
        new_end_state.end_state = false;
        let new_end_state_id = top_fragment.enfa.add_state(new_end_state);

        // extend the automaton by a transition from new start state to the new end state
        top_fragment.enfa.add_transition(top_fragment.start_id, Input::Epsilon, new_end_state_id);

        // modify and push copy
        top_fragment.start_id = new_start_state_id;
        top_fragment.end_id = new_end_state_id;
        fragment_stack.stack.push(top_fragment);
    }
}

// + Plus
fn add_repeat_one_or_more(fragment_stack: &mut FragmentStack) {

    // if enfa.is_empty() {
    //     panic!("Cannot apply repeat operation to empty automaton!");
    // }

    // check if the top fragment contains a atomic regex_building_block (character_literal, character_class)
    // or if the top fragment contains a complex graph

    // pop both top elements from the fragment stack because a new state is added and the end_id value has to be changed
    let mut top_fragment = fragment_stack.stack.pop().unwrap();

    if top_fragment.start_id == top_fragment.end_id {
        
        // extend the automaton by a end new state
        let mut new_end_state = State::new(STATE_COUNTER.fetch_add(1, Ordering::SeqCst));
        new_end_state.start_state = false;
        new_end_state.end_state = true;
        let new_end_state_id = top_fragment.enfa.add_state(new_end_state);

        // extend the automaton by a transition to the new end state
        top_fragment.enfa.add_transition(top_fragment.start_id, Input::Symbol(top_fragment.symbol), new_end_state_id);

        top_fragment.end_id = new_end_state_id;
    }

    let old_start_id = top_fragment.start_id;
    let old_end_id = top_fragment.end_id;

    // extend the automaton by a transition to the new end state
    top_fragment.enfa.add_transition(old_end_id, Input::Epsilon, old_start_id);

    fragment_stack.stack.push(top_fragment);

    // // I really do not understand why a new end state is required!
    // let add_new_end_state = false;
    // if add_new_end_state {

    //     // extend the automaton by a end new state
    //     let mut new_end_state = State::new(STATE_COUNTER.fetch_add(1, Ordering::SeqCst));
    //     new_end_state.start_state = false;
    //     new_end_state.end_state = false;
    //     let new_end_state_id = top_fragment.enfa.add_state(new_end_state);

    //     // extend the automaton by a transition to the new end state
    //     top_fragment.enfa.add_transition(old_end_id, Input::Epsilon, new_end_state_id);

    //     // modify and push copy
    //     top_fragment.start_id = old_start_id;
    //     top_fragment.end_id = new_end_state_id;
    //     fragment_stack.stack.push(top_fragment);
    // } else {
    //     fragment_stack.stack.push(top_fragment);
    // }
}

// ^ Not / Inversion
// 
// Second interpretation. This second interpretation, if applied to a single character literal will accept
// all character literals in the alphabet except the negated literal.
// For example ^a means under this interpretation: b, c, d, e, f ... (anything but not a).
//
// NB: the automaton will in fact consume the negated literal and then transition to the end state!
// This means the following state machine will not ever see the negated symbol. The automaton will remain 
// in the current state and consume token until it sees the negated symbol, then transition to the end state
// while consuming the negated symbol!
fn add_not_single_character_interpretation(fragment_stack: &mut FragmentStack, alphabet: &HashSet<RegexBuildingBlock>) {

    // pop top element from the fragment stack
    let mut top_fragment = fragment_stack.stack.pop().unwrap();

    // // DEBUG
    // enfa_to_dot_directed_graph(&mut top_fragment.enfa, "top_fragment.dot");
    
    if top_fragment.enfa.transitions.len() == 0 {

        // remove character literal from top of the stack. Already done

        // build the NOT automaton

        // add new end state
        // extend the automaton by a new end state
        let mut new_end_state = State::new(STATE_COUNTER.fetch_add(1, Ordering::SeqCst));
        new_end_state.start_state = false;
        new_end_state.end_state = false;
        let new_end_state_id = top_fragment.enfa.add_state(new_end_state);

        // add transition between start and end
        top_fragment.enfa.add_transition(top_fragment.start_id, Input::Symbol(top_fragment.symbol), new_end_state_id);

        // add loop transition
        //top_fragment.enfa.add_transition(top_fragment.start_id, Input::AllOther, top_fragment.start_id);

        // add loop transition
        for symb in alphabet {
            if *symb == top_fragment.symbol {
                continue;
            }
            top_fragment.enfa.add_transition(top_fragment.start_id, Input::Symbol(*symb), top_fragment.start_id);
        }

        top_fragment.end_id = new_end_state_id;

        // push fragment back
        fragment_stack.stack.push(top_fragment.clone());
        
    } else if top_fragment.enfa.transitions.len() == 1 {

        // // Check for this structure: --> () -symbol-> () -->
        // top_fragment.enfa.add_transition(top_fragment.start_id, Input::AllOther, top_fragment.start_id);

        // add loop transition
        for symb in alphabet {
            if *symb == top_fragment.symbol {
                continue;
            }
            top_fragment.enfa.add_transition(top_fragment.start_id, Input::Symbol(*symb), top_fragment.start_id);
        }

        // push fragment back
        fragment_stack.stack.push(top_fragment.clone());
    }
    // // check if the current fragment consists of a atomic or a complex automaton
    // if top_fragment.start_id == top_fragment.end_id {
    //     top_fragment.enfa.add_transition(top_fragment.start_id, Input::Epsilon, top_fragment.end_id);
    // } 
    else {
        panic!("Not implemented for complex automata!");
    }

    // // DEBUG
    // enfa_to_dot_directed_graph(&mut top_fragment.enfa, "top_fragment.dot");
}

// ^ Not / Inversion
//
// This interpretation allows larger strings. Larger strings maybe be valid if they do exactly match the negated symbol.
// For example: ^a means under this interpretation: epsilon, aa, aaa, aaaa, .... (anything that is not 'a')
//
// A second interpretation is possible. The second interpretation, if applied to a single character literal will accept
// all character literals in the alphabet except the negated literal.
// For example ^a means under this interpretation: b, c, d, e, f ... (anything but not a)
fn add_not_extended_interpretation(fragment_stack: &mut FragmentStack, alphabet: &HashSet<RegexBuildingBlock>) {

    // pop top element from the fragment stack
    let mut top_fragment = fragment_stack.stack.pop().unwrap();

    // // DEBUG
    // enfa_to_dot_directed_graph(&mut top_fragment.enfa, "top_fragment.dot");

    // check if the current fragment consists of a atomic or a complex automaton
    if top_fragment.start_id == top_fragment.end_id {

        // create new end state id
        let mut end_state = State::new(STATE_COUNTER.fetch_add(1, Ordering::SeqCst));
        end_state.start_state = false;
        end_state.end_state = false;

        // add new state to the automaton
        let end_state_id = top_fragment.enfa.add_state(end_state);

        // extend the automaton by a transition to the new end state
        top_fragment.enfa.add_transition(top_fragment.enfa.start_state_id, Input::Symbol(top_fragment.symbol), end_state_id);

        top_fragment.end_id = end_state_id;

        // // DEBUG
        // enfa_to_dot_directed_graph(&mut top_fragment.enfa, "not_enfa_automaton.dot");

        //
        // convert eNFA to DFA
        //

        // first, make last state an accepting state, otherwise the eNFA to DFA conversion will produce incorrect results
        top_fragment.enfa.states.get_mut(&top_fragment.end_id).unwrap().end_state = true;

        // convert from eNFA to DFA
        let mut dfa = enfa_to_dfa(&mut top_fragment.enfa, alphabet);

        // // DEBUG -- DFA from initial eNFA
        // enfa_to_dot_directed_graph(&mut dfa, "dfa_automaton.dot");

        // invert end states
        for (_state_id, state) in dfa.states.iter_mut() {
            if state.end_state {
                state.end_state = false;
            } else {
                state.end_state = true;
            }
        }

        // add a new end state
        let mut new_end_state = State::new(STATE_COUNTER.fetch_add(1, Ordering::SeqCst));
        new_end_state.start_state = false;
        new_end_state.end_state = false;
        let new_end_state_id = new_end_state.id;

        dfa.states.insert(new_end_state_id, new_end_state);

        let mut end_state_memory = Vec::<usize>::new();

        // for each node that is an accepting state, remove the accepting flag and ...
        for (state_id, state) in dfa.states.iter_mut() {
            if state.end_state {
                state.end_state = false;

                end_state_memory.push(*state_id);
            }
        }

        // ... add an epsilon transition to the new end state
        for state_id in end_state_memory {
            dfa.add_transition(state_id, Input::Epsilon, new_end_state_id);
        }

        // // DEBUG - DFA with additional final state
        // enfa_to_dot_directed_graph(&mut dfa, "inverted_dfa_automaton.dot");

        top_fragment.start_id = dfa.start_state_id;
        top_fragment.end_id = new_end_state_id;
        top_fragment.enfa = dfa;

        // push new fragment
        fragment_stack.stack.push(top_fragment);

        /*
        // --> (()) -symbol-> () -->

        // build new automaton
        
        // start state of the automaton is already an accepting state
        let mut start_state = State::new(STATE_COUNTER.fetch_add(1, Ordering::SeqCst));
        start_state.start_state = true;
        start_state.end_state = true;
        let start_state_id = top_fragment.enfa.add_state(start_state);

        // end state id
        let mut trap_state = State::new(STATE_COUNTER.fetch_add(1, Ordering::SeqCst));
        trap_state.start_state = false;
        trap_state.end_state = false;
        let end_state_id = top_fragment.enfa.add_state(trap_state);

        // extend the automaton by a transition to the new end state
        top_fragment.enfa.add_transition(start_state_id, Input::Symbol(top_fragment.symbol), end_state_id);

        top_fragment.end_id = end_state_id;
        */

    } else {

        // --> () -symbol-> (()) -->
        // --> (()) -symbol-> () -->

        // convert eNFA to DFA (AI told me that a inversion of a eNFA is too complicated and it is 
        // less errorprone to invert a DFA. So convert a eNFA to DFA)

        // first, make last state an accepting state, otherwise the eNFA to DFA conversion will produce incorrect results
        top_fragment.enfa.states.get_mut(&top_fragment.end_id).unwrap().end_state = true;

        // DEBUG -- initial eNFA
        // enfa_to_dot_directed_graph(&mut top_fragment.enfa, "enfa_automaton.dot");

        // convert from eNFA to DFA
        let mut dfa = enfa_to_dfa(&mut top_fragment.enfa, alphabet);

        // // DEBUG -- DFA from initial eNFA
        // enfa_to_dot_directed_graph(&mut dfa, "dfa_automaton.dot");

        // // DEBUG
        // for (state_id, state) in dfa.states.iter_mut() {
        //     println!("StateID: {}, State: {:?}", state_id, state);
        // }

        // invert end states
        for (_state_id, state) in dfa.states.iter_mut() {
            if state.end_state {
                state.end_state = false;
            } else {
                state.end_state = true;
            }
        }

        // // DEBUG - DFA with states inverted
        // enfa_to_dot_directed_graph(&mut dfa, "dfa_automaton.dot");

        // now invert the graph

        // add a new end state
        let mut new_end_state = State::new(STATE_COUNTER.fetch_add(1, Ordering::SeqCst));
        new_end_state.start_state = false;
        new_end_state.end_state = false;
        let new_end_state_id = new_end_state.id;

        dfa.states.insert(new_end_state_id, new_end_state);

        let mut end_state_memory = Vec::<usize>::new();

        // for each node that is an accepting state, remove the accepting flag and ...
        for (state_id, state) in dfa.states.iter_mut() {
            if state.end_state {
                state.end_state = false;

                end_state_memory.push(*state_id);
            }
        }

        // ... add an epsilon transition to the new end state
        for state_id in end_state_memory {
            dfa.add_transition(state_id, Input::Epsilon, new_end_state_id);
        }

        // // DEBUG - mark the new state as end state for better visibility in the dot graphviz graph
        // dfa.set_end_state(new_end_state_id, true);

        // // DEBUG
        // for (state_id, state) in dfa.states.iter_mut() {
        //     println!("StateID: {}, State: {:?}", state_id, state);
        // }

        // // DEBUG - DFA with additional final state
        // enfa_to_dot_directed_graph(&mut dfa, "dfa_automaton.dot");

        // build a new fragment
        let mut fragment = Fragment::new(RegexBuildingBlock::Not);
        fragment.start_id = dfa.start_state_id;
        fragment.end_id = new_end_state_id;
        fragment.enfa = dfa;

        // push new fragment
        fragment_stack.stack.push(fragment);
    }
}

// build start state by building the epsilon-reach of the eNFA start state
// The epsilon-reach is the set of states that can be reached by applying
// following arbitrary many epsilon transitions from all the states in a powerstate.
// A powerstate is a state that is made up of one or more states of the eNFA.
// Powerstates will become the states of the DFA.
fn epsilon_reach(enfa: &mut EpsilonNfa::<State, RegexBuildingBlock>, power_state_in: & BTreeSet::<usize>) -> BTreeSet::<usize> {

    // build power state for start state
    let mut power_state = power_state_in.clone();
    let mut new_states = BTreeSet::<usize>::new();
    let mut temp_states = BTreeSet::<usize>::new();

    // insert start state (has id 0) TODO: use real start state of NFA
    //power_state.insert(0);
    new_states.extend(power_state_in.clone());

    // loop as long as the epsilon-reach is still extending into the automaton
    let mut set_has_changed: bool = true;
    while set_has_changed {

        // abort the loop unless the epsilon-reach is still extends accross two consecutive iterations
        set_has_changed = false;

        // iterate over all states at the edge of the current epsilon-reach
        for current_start_state_id in &new_states {

            // iterate over all transitions of the eNFA
            for ((start_state_id, input_symbol), end_state_id_set) in enfa.transitions.iter_mut() {

                // find epsilon-transitions extending from the current edge state
                if *start_state_id == *current_start_state_id && *input_symbol == Input::Epsilon {

                    // over all end states that the epsilon-transitions reach
                    for end_state_id in end_state_id_set.iter() {

                        // if the end state is not handled yet, add it to the set
                        if !power_state.contains(end_state_id) {

                            power_state.insert(*end_state_id);
                            temp_states.insert(*end_state_id);

                            // the reach algorithm keeps iterating
                            set_has_changed = true;
                        }
                    }
                }
            }
        }

        new_states.clear();
        new_states.extend(&temp_states);
    }

    // println!("{:?}", power_state);

    power_state
}

pub fn enfa_copy(dest: &mut EpsilonNfa::<State, RegexBuildingBlock>, src: &mut EpsilonNfa::<State, RegexBuildingBlock>, end_id: usize) -> (usize, usize) {

    // // DEBUG
    // println!("enfa_copy start >>>   end_id: {}", end_id);

    // // DEBUG
    // enfa_to_dot_directed_graph(dest, "dest_automaton.dot");
    // enfa_to_dot_directed_graph(src, "src_automaton.dot");

    let mut copied_start_id = 0;
    let mut copied_end_id = 0;

    // over all transitions in the source graph
    for ((start_state_id, input_symbol), end_state_id_set) in src.transitions.iter_mut() {

        // copy state over from src to dest
        // flag the state as copied by writing the id of the newly created node in the dest graph into it

        // copy_id is zero unless the state has been copied already
        let mut another_state_id = src.states[start_state_id].copy_id;

        // only copy the state if it has not been copied already
        if another_state_id == 0 {

            // insert new state into dest
            let another_state = State::new(STATE_COUNTER.fetch_add(1, Ordering::SeqCst));
            another_state_id = dest.add_state(another_state);

            // write dest id into src state's copy_id so it will not be copied again
            src.states.get_mut(start_state_id).unwrap().copy_id = another_state_id;

            // retrieve the start state of the transition from the src graph
            let src_state = src.states.get_mut(start_state_id).unwrap();

            // copy token_id, token_name over
            let dest_state = dest.states.get_mut(&another_state_id).unwrap();
            dest_state.token_id = src_state.token_id;
            dest_state.token_name = src_state.token_name.clone();

            // if the start state of the transition is the end state of the src enfa, 
            // remember the id of the corresponding copied node for later use
            if *start_state_id == end_id {

                // // DEBUG
                // println!("EDGE-CASE: End Node First: Target Node is end state!");

                copied_end_id = another_state_id;

                dest.states.get_mut(&another_state_id).unwrap().end_state = true;
            }
        }

        // if this is the start state of the src enfa, remember the id of the corresponding copied node for later use
        if *start_state_id == src.start_state_id {
            copied_start_id = another_state_id;
        }

        // // if this is the end state of the top enfa, remember the id of the corresponding copied node for later use
        // if *start_state_id == end_id {
        //     copied_end_id = another_state_id;
        // }

        // copy all transitions emanating from the state
        for end_state_id in end_state_id_set.iter() {

            // find the id of the end state in the dest graph by looking at the id stored in copy id in the state in the src graph
            let mut copy_state_id = src.states[end_state_id].copy_id;

            // // DEBUG
            // println!("Transition {} -{:?}-> {}    copied-id: {}", start_state_id, input_symbol, end_state_id, copy_state_id);

            // only if the target state has not been copied already, create it in the dest graph
            if copy_state_id == 0 {

                // // DEBUG
                // println!("Creating new node in dest for target-node of transition");
            
                // create a new state in the dest graph
                let another_state = State::new(STATE_COUNTER.fetch_add(1, Ordering::SeqCst));
                copy_state_id = dest.add_state(another_state);

                //println!("Comparing {} -> {}", *end_state_id, end_id);

                // if this is the end state of the src enfa, remember the id of the corresponding copied node for later use
                if *end_state_id == end_id {

                    // println!("Target Node is end state!");

                    copied_end_id = copy_state_id;

                    dest.states.get_mut(&copy_state_id).unwrap().end_state = true;
                    dest.states.get_mut(&copy_state_id).unwrap().token_id = src.states.get_mut(end_state_id).unwrap().token_id;
                    dest.states.get_mut(&copy_state_id).unwrap().token_name = src.states.get_mut(end_state_id).unwrap().token_name.clone();
                }

                // write the destination id into the copy_id field of the source node
                src.states.get_mut(end_state_id).unwrap().copy_id = copy_state_id;

                // // DEBUG
                // println!("B End State Id {} -> {}", end_state_id, copy_state_id);
            }

            // // a transition forms a loop! Why should this be an issue?
            // if another_state_id == copy_state_id {
            //     panic!("loop!");
            // }
            
            // add a transition between the start and the end end node in the dest graph
            dest.add_transition(another_state_id, *input_symbol, copy_state_id);
        }
    }

    // println!("enfa_copy end <<<");

    // if copied_end_id == 0 {
    //     panic!("test");
    // }

    assert!(copied_start_id != 0, "This function only works if the source NFA does have a state flagged as start state and the start state's id is contained in the start_state_id property of the NFA!");
    assert!(copied_end_id != 0, "This function only works if the source NFA does have a state flagged as end state!");

    (copied_start_id, copied_end_id)
}

pub fn enfa_to_dfa(enfa: &mut EpsilonNfa::<State, RegexBuildingBlock>, 
    alphabet: &HashSet<RegexBuildingBlock>) -> EpsilonNfa::<State, RegexBuildingBlock> {

    let mut power_state_id_map = HashMap::<BTreeSet::<usize>, usize>::new();

    //
    // STEP 1 - build start state from the start state of the eNFA and the epsilon_reach of that start state
    //

    let mut start_power_state = BTreeSet::<usize>::new();
    start_power_state.insert(enfa.start_state_id);
    let epsilon_reach_start_state = epsilon_reach(enfa, &start_power_state);

    // check if there is at least one end state in the epsilon reach
    let mut end_state = false;
    for state_id in &epsilon_reach_start_state {
        if enfa.states[state_id].end_state {
            // println!("{:?} is end state", &epsilon_reach_start_state);
            end_state = true;
            break;
        }
    }

    // map power state to it's new id which will be use in the resulting DFA graph
    // insert the start state (which is a power state itself) into the power state map which assigns an id to the powerstate
    power_state_id_map.insert(epsilon_reach_start_state.clone(), STATE_COUNTER.fetch_add(1, Ordering::SeqCst));

    // read id back
    let temp_id = power_state_id_map[&epsilon_reach_start_state];
    // println!("New State: {:?}. EndState: {}. DFA-ID: {}", epsilon_reach_start_state, end_state, temp_id);

    //
    // Prepare DFA data structure / variable
    //

    // extend the DFA by a new state for the power state that has just been constructed 
    // BUILD DFA start state using the start power state from above
    let mut dfa_start_state = State::new(temp_id);
    dfa_start_state.start_state = true;
    dfa_start_state.end_state = end_state;

    // create DFA instance
    let mut dfa = EpsilonNfa::<State, RegexBuildingBlock>::new(dfa_start_state.clone());

    //
    // STEP 2 - for each power state in D (= states need processing) for each input symbol, find powerstate the eNFA transition into
    // 

    // set D
    let mut d = Vec::<BTreeSet::<usize>>::new();
    d.push(epsilon_reach_start_state.clone());

    let mut processed = Vec::<BTreeSet::<usize>>::new();

    // as long as d has states to process
    while d.len() > 0 {

        let current_power_state = d.pop().unwrap();

        // DEBUG 
        //println!("Processing state: {:?}", &current_power_state);

        for symbol in alphabet {

            let mut next_power_state = BTreeSet::<usize>::new();
                
            // iterate over all states at the edge of the current epsilon-reach
            for current_state_id in &current_power_state {

                // iterate over all transitions of the eNFA
                for ((start_state_id, transition_input_symbol), end_state_id_set) in enfa.transitions.iter_mut() {

                    // find symbol-transitions extending from the current state
                    if *start_state_id == *current_state_id && *transition_input_symbol == Input::Symbol(*symbol) {
                        
                        // insert all end states into the newly created powerstate
                        next_power_state.extend(end_state_id_set.clone());
                    }
                }                  
            }

            let epsilon_reach_next_power_state = epsilon_reach(enfa, &next_power_state);

            processed.push(current_power_state.clone());

            // the trap-state is not inserted into D
            if epsilon_reach_next_power_state.len() != 0 {

                // only insert into D if not contained and not processed already
                if !d.contains(&epsilon_reach_next_power_state) && !processed.contains(&epsilon_reach_next_power_state) {
                    
                    let mut end_state = false;
                    let mut token_id = 0;
                    let mut token_name = String::from("");

                    // iterate over all states in the power state
                    for state_id in &epsilon_reach_next_power_state {

                        // check if the power state contains at least one end state
                        if enfa.states[state_id].end_state {
                            
                            // println!("{:?} is end state", &epsilon_reach_next_power_state);
                            end_state = true;
                            // break;

                            // check if the end state is an accepting state and copy it's token id
                            if enfa.states[state_id].token_id != 0 {
                                // panic!("found");
                                token_id = enfa.states[state_id].token_id;
                                token_name = enfa.states[state_id].token_name.clone();
                            }
                        }
                    }

                    power_state_id_map.insert(epsilon_reach_next_power_state.clone(), STATE_COUNTER.fetch_add(1, Ordering::SeqCst));
                    // power_state_id_map.insert(epsilon_reach_next_power_state.clone(), next_id);
                    // next_id = next_id + 1;

                    let temp_id = power_state_id_map[&epsilon_reach_next_power_state];
                    // println!("New State: {:?}. EndState: {}. DFA-ID: {}", epsilon_reach_next_power_state, end_state, temp_id);

                    // push new state into the D set which is the set of states that need to be processed
                    d.push(epsilon_reach_next_power_state.clone());

                    // extend the DFA by a new state for the power state that has just been constructed 
                    let mut dfa_state = State::new(temp_id);
                    dfa_state.start_state = false;
                    dfa_state.end_state = end_state;
                    dfa_state.token_id = token_id;
                    dfa_state.token_name = token_name.clone();

                    dfa.add_state(dfa_state);
                }
            }

            // trap state
            if epsilon_reach_next_power_state.len() == 0 && !power_state_id_map.contains_key(&epsilon_reach_next_power_state) {
                // println!("New Trap State {{}}");

                power_state_id_map.insert(epsilon_reach_next_power_state.clone(), STATE_COUNTER.fetch_add(1, Ordering::SeqCst));
                // power_state_id_map.insert(epsilon_reach_next_power_state.clone(), next_id);
                // next_id = next_id + 1;

                let temp_id = power_state_id_map[&epsilon_reach_next_power_state];
                // println!("New Trap State: {:?}. EndState: {}. DFA-ID: {}", epsilon_reach_next_power_state, end_state, temp_id);

                // BUILD DFA
                let mut trap_state = State::new(temp_id);
                trap_state.start_state = false;
                trap_state.end_state = false;
                trap_state.trap_state = true;

                dfa.add_state(trap_state);

                // TODO loop for all symbols from trap_state to trap_state

                for temp_symbol in alphabet {
                    dfa.add_transition(temp_id, Input::Symbol(*temp_symbol), temp_id);
                }
            }

            // // DEBUG
            // println!("{:?} => using symbol {:?} => {:?}", current_power_state, *symbol, epsilon_reach_next_power_state);

            let s_id = power_state_id_map[&current_power_state];
            let e_id = power_state_id_map[&epsilon_reach_next_power_state];
            
            dfa.add_transition(s_id, Input::Symbol(*symbol), e_id);
        }
    }

    dfa
}

// try to transition the large lexer DFA to produce a token for the input
pub fn transition_dfa(dfa: &mut EpsilonNfa::<State, RegexBuildingBlock>, start_id: usize, input: &RegexBuildingBlock) -> usize {
    
    let target_state_ids = dfa.transitions.entry((start_id, Input::Symbol(*input)));
    let val = target_state_ids.or_default();
    
    // DEBUG
    // println!("{:?}", val);

    val.clone().into_iter().next().unwrap()
}