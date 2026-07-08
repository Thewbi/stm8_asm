// filename: main_regex_enfa_dfa.rs

#![allow(
dead_code,
unused_imports,
unused_must_use,
unused_variables,
unused_assignments
)]

use std::fs::File;
use std::io::{BufWriter, Write};

use std::fmt;
use std::fmt::Debug;

use std::collections::{HashMap, HashSet, BTreeSet};
use std::hash::Hash;
use std::{
    sync::atomic::{AtomicUsize, Ordering}
};

mod regex;
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
    token_id: usize,
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

        // if this state is an accepting state for a tooken, mark it in the dot file
        if state.token_id != 0 {
            write!(writer, "\t{}[label=\"{}, Token: {}\"];\n", state_id, state_id, state.token_id);
        }
    }

    // iterate over transitions
    // ((S, Input<T>), HashSet<S>) 
    for ((start_state_id, input_symbol), end_state_id_set) in enfa.transitions.iter_mut() {

        for end_state_id in end_state_id_set.iter() {

            write!(writer, "\t{}", start_state_id);
            write!(writer, " -> ");
            write!(writer, "{}", end_state_id);
            write!(writer, "[label=\"{:?}\"]", input_symbol);
            write!(writer, ";");
            write!(writer, "\n");
        }
    }

    write!(writer, "{}", "}");

    // 4. Explicitly flush the remaining data to disk
    writer.flush().expect("flush failed!");
}

#[derive(Clone)]
struct Fragment {
    enfa: EpsilonNfa::<State, RegexBuildingBlock>,
    symbol: RegexBuildingBlock,
    start_id: usize,
    end_id: usize,
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

struct FragmentStack {
    stack: Vec<Fragment>,
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

fn add_character_literal(fragment_stack: &mut FragmentStack, regex_building_block: RegexBuildingBlock, alphabet: &mut HashSet<RegexBuildingBlock>) {

    alphabet.insert(regex_building_block);

    if fragment_stack.is_empty() {

        let mut fragment = Fragment::new(regex_building_block);
    
        let mut another_state = State::new(STATE_COUNTER.fetch_add(1, Ordering::SeqCst));
        another_state.start_state = false;
        another_state.end_state = false;
        let another_state_id = fragment.enfa.add_state(another_state);

        fragment.enfa.add_transition(fragment.enfa.start_state_id, Input::Symbol(regex_building_block), another_state_id);
        fragment.end_id = another_state_id;
    
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
        enfa_to_dot_directed_graph(&mut top_fragment.enfa, "top_automaton.dot");

        // DEBUG
        enfa_to_dot_directed_graph(&mut bottom_fragment.enfa, "bottom_automaton.dot");
        
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

// * Star, Asterisk, Repeat[0, std::u8::MAX]
fn add_repeat_zero_or_more(fragment_stack: &mut FragmentStack) {

    let mut top_fragment = fragment_stack.stack.pop().unwrap();

    // // check if the top fragment contains an atomic regex_building_block (character_literal, character_class)
    // // or if the top fragment contains a complex graph
    if top_fragment.start_id == top_fragment.end_id {
        
        // extend the automaton by a new end state
        let mut new_end_state = State::new(STATE_COUNTER.fetch_add(1, Ordering::SeqCst));
        new_end_state.start_state = false;
        new_end_state.end_state = false;
        let new_end_state_id = top_fragment.enfa.add_state(new_end_state);

        // add epsilon transitions from the old start state to the new end state
        top_fragment.enfa.add_transition(top_fragment.start_id, Input::Epsilon, new_end_state_id);

        top_fragment.enfa.add_transition(new_end_state_id, Input::Symbol(top_fragment.symbol), top_fragment.start_id);


        // modify and push copy
        // top_fragment.start_id = new_start_state_id;
        top_fragment.end_id = new_end_state_id;
        fragment_stack.stack.push(top_fragment);

    } else {
        let old_start_id = top_fragment.start_id;
        let old_end_id = top_fragment.end_id;

        // make the old start state a regular state
        top_fragment.enfa.set_start_state(top_fragment.enfa.start_state_id, false);

        // extend the automaton by a new start state
        let mut new_start_state = State::new(STATE_COUNTER.fetch_add(1, Ordering::SeqCst));
        new_start_state.start_state = true;
        new_start_state.end_state = false;
        let new_start_state_id = top_fragment.enfa.add_state(new_start_state);
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

        // extend the automaton by a transition to the new end state
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
fn add_not(fragment_stack: &mut FragmentStack, alphabet: &HashSet<RegexBuildingBlock>) {

    // pop both top elements from the fragment stack because a new state is added and the end_id value has to be changed
    let mut top_fragment = fragment_stack.stack.pop().unwrap();

    // check if the current fragment consists of a atomic or a complex automaton
    if top_fragment.start_id == top_fragment.end_id {

        // panic!("not implemented yet!");

        // end state id
        let mut end_state = State::new(STATE_COUNTER.fetch_add(1, Ordering::SeqCst));
        end_state.start_state = false;
        end_state.end_state = false;
        let end_state_id = top_fragment.enfa.add_state(end_state);

        // extend the automaton by a transition to the new end state
        top_fragment.enfa.add_transition(top_fragment.enfa.start_state_id, Input::Symbol(top_fragment.symbol), end_state_id);

        top_fragment.end_id = end_state_id;

        // // DEBUG
        // enfa_to_dot_directed_graph(&mut top_fragment.enfa, "enfa_automaton.dot");

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
        // enfa_to_dot_directed_graph(&mut dfa, "dfa_automaton.dot");

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
        //enfa_to_dot_directed_graph(&mut top_fragment.enfa, "enfa_automaton.dot");

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

fn enfa_copy(dest: &mut EpsilonNfa::<State, RegexBuildingBlock>, src: &mut EpsilonNfa::<State, RegexBuildingBlock>, end_id: usize) -> (usize, usize) {

    // // DEBUG
    // println!("enfa_copy start >>>   end_id: {}", end_id);

    // DEBUG
    enfa_to_dot_directed_graph(dest, "dest_automaton.dot");
    enfa_to_dot_directed_graph(src, "src_automaton.dot");

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

            // copy token_id over
            //another_state.token_id = src.states.get_mut(start_state_id).unwrap().token_id;

            let src_state = src.states.get_mut(start_state_id).unwrap();
            // src_state.copy_id = another_state_id;
            // another_state.token_id = src_state.token_id;

            let dest_state = dest.states.get_mut(&another_state_id).unwrap();
            dest_state.token_id = src_state.token_id;

            // if the start state of the transition is the end state of the src enfa, remember the id of the corresponding copied node for later use
            if *start_state_id == end_id {

                // // DEBUG
                // println!("EDGE-CASE: End Node First: Target Node is end state!");

                copied_end_id = another_state_id;

                dest.states.get_mut(&another_state_id).unwrap().end_state = true;
                // src.states.get_mut(&start_state_id).unwrap().token_id = src.states.get_mut(end_state_id).unwrap().token_id;
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
                }

                // write the destination id into the copy_id field of the source node
                src.states.get_mut(end_state_id).unwrap().copy_id = copy_state_id;

                // // DEBUG
                // println!("B End State Id {} -> {}", end_state_id, copy_state_id);
            }

            if another_state_id == copy_state_id {
                panic!("loop!");
            }
            
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

fn enfa_to_dfa(enfa: &mut EpsilonNfa::<State, RegexBuildingBlock>, alphabet: &HashSet<RegexBuildingBlock>) -> EpsilonNfa::<State, RegexBuildingBlock> {

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

fn transition_dfa(dfa: &mut EpsilonNfa::<State, RegexBuildingBlock>, start_id: usize, input: &RegexBuildingBlock) -> usize {
    let target_state_ids = dfa.transitions.entry((start_id, Input::Symbol(*input)));
    let val = target_state_ids.or_default();

    // println!("{:?}", val);

    val.clone().into_iter().next().unwrap()
}

// https://courses.grainger.illinois.edu/cs374/fa2020/lec_prerec/05/5_1_2.pdf
// 1|0
fn build_nfa(alphabet: &mut HashSet<RegexBuildingBlock>) -> EpsilonNfa::<State, RegexBuildingBlock> {

    let mut q0 = State::new(STATE_COUNTER.fetch_add(1, Ordering::SeqCst));
    q0.start_state = true;

    let mut q1 = State::new(STATE_COUNTER.fetch_add(1, Ordering::SeqCst));
    q1.start_state = false;

    let mut q2 = State::new(STATE_COUNTER.fetch_add(1, Ordering::SeqCst));
    q2.start_state = false;

    let mut q3 = State::new(STATE_COUNTER.fetch_add(1, Ordering::SeqCst));
    q3.start_state = false;
    q3.end_state = true;

    let mut enfa = EpsilonNfa::<State, RegexBuildingBlock>::new(q0.clone());

    let q0_id = enfa.start_state_id;
    let q1_id = enfa.add_state(q1);
    let q2_id = enfa.add_state(q2);
    let q3_id = enfa.add_state(q3);

    // 1|0

    enfa.add_transition(q0_id, Input::Epsilon, q1_id);

    enfa.add_transition(q0_id, Input::Symbol(RegexBuildingBlock::CharacterLiteral('1')), q2_id);
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('1'));

    enfa.add_transition(q1_id, Input::Symbol(RegexBuildingBlock::CharacterLiteral('0')), q3_id);
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('0'));

    enfa.add_transition(q2_id, Input::Epsilon, q3_id);

    // make last state an end state
    enfa.states.get_mut(&q3_id).unwrap().end_state = true;

    //enfa_to_dot_directed_graph(&mut top_fragment.enfa, "enfa_automaton.dot");

    enfa
}

// https://www.geeksforgeeks.org/theory-of-computation/conversion-from-nfa-to-dfa/
// (a|b)*ab
fn build_nfa_2(alphabet: &mut HashSet<RegexBuildingBlock>) -> EpsilonNfa::<State, RegexBuildingBlock> {

    let mut q0 = State::new(STATE_COUNTER.fetch_add(1, Ordering::SeqCst));
    q0.start_state = true;

    let mut q1 = State::new(STATE_COUNTER.fetch_add(1, Ordering::SeqCst));
    q1.start_state = false;

    let mut q2 = State::new(STATE_COUNTER.fetch_add(1, Ordering::SeqCst));
    q2.start_state = false;
    q2.end_state = true;

    let mut enfa = EpsilonNfa::<State, RegexBuildingBlock>::new(q0.clone());

    let q0_id = enfa.start_state_id;
    let q1_id = enfa.add_state(q1);
    let q2_id = enfa.add_state(q2);

    alphabet.insert(RegexBuildingBlock::CharacterLiteral('a'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('b'));

    // (a|b)*ab

    enfa.add_transition(q0_id, Input::Symbol(RegexBuildingBlock::CharacterLiteral('a')), q0_id);
    enfa.add_transition(q0_id, Input::Symbol(RegexBuildingBlock::CharacterLiteral('b')), q0_id);

    enfa.add_transition(q0_id, Input::Symbol(RegexBuildingBlock::CharacterLiteral('a')), q1_id);

    enfa.add_transition(q1_id, Input::Symbol(RegexBuildingBlock::CharacterLiteral('b')), q2_id);

    enfa
}

// d+c*
fn build_nfa_3(alphabet: &mut HashSet<RegexBuildingBlock>) -> EpsilonNfa::<State, RegexBuildingBlock> {

    let mut fragment_stack = FragmentStack::new();

    // d+c*

    add_character_literal(&mut fragment_stack, RegexBuildingBlock::CharacterLiteral('d'), alphabet);
    add_repeat_one_or_more(&mut fragment_stack);

    add_character_literal(&mut fragment_stack, RegexBuildingBlock::CharacterLiteral('c'), alphabet);
    add_repeat_zero_or_more(&mut fragment_stack);

    add_concatenation(&mut fragment_stack);

    let mut top_fragment = fragment_stack.stack.pop().unwrap();

    // make last state an end state
    top_fragment.enfa.states.get_mut(&top_fragment.end_id).unwrap().end_state = true;

    //enfa_to_dot_directed_graph(&mut top_fragment.enfa, "enfa_automaton.dot");

    top_fragment.enfa
}

// hello | world
fn build_nfa_4(alphabet: &mut HashSet<RegexBuildingBlock>) -> EpsilonNfa::<State, RegexBuildingBlock> {

    let mut fragment_stack = FragmentStack::new();

    // hello

    add_character_literal(&mut fragment_stack, RegexBuildingBlock::CharacterLiteral('h'), alphabet);
    add_character_literal(&mut fragment_stack, RegexBuildingBlock::CharacterLiteral('e'), alphabet);
    add_concatenation(&mut fragment_stack);
    add_character_literal(&mut fragment_stack, RegexBuildingBlock::CharacterLiteral('l'), alphabet);
    add_concatenation(&mut fragment_stack);
    add_character_literal(&mut fragment_stack, RegexBuildingBlock::CharacterLiteral('l'), alphabet);
    add_concatenation(&mut fragment_stack);
    add_character_literal(&mut fragment_stack, RegexBuildingBlock::CharacterLiteral('o'), alphabet);
    add_concatenation(&mut fragment_stack);

    // world

    add_character_literal(&mut fragment_stack, RegexBuildingBlock::CharacterLiteral('w'), alphabet);
    add_character_literal(&mut fragment_stack, RegexBuildingBlock::CharacterLiteral('o'), alphabet);
    add_concatenation(&mut fragment_stack);
    add_character_literal(&mut fragment_stack, RegexBuildingBlock::CharacterLiteral('r'), alphabet);
    add_concatenation(&mut fragment_stack);
    add_character_literal(&mut fragment_stack, RegexBuildingBlock::CharacterLiteral('l'), alphabet);
    add_concatenation(&mut fragment_stack);
    add_character_literal(&mut fragment_stack, RegexBuildingBlock::CharacterLiteral('d'), alphabet);
    add_concatenation(&mut fragment_stack);

    add_or(&mut fragment_stack);

    let mut top_fragment = fragment_stack.stack.pop().unwrap();

    // make last state an end state
    top_fragment.enfa.states.get_mut(&top_fragment.end_id).unwrap().end_state = true;

    //enfa_to_dot_directed_graph(&mut top_fragment.enfa, "enfa_automaton.dot");

    top_fragment.enfa
}

// fn main() {
fn main_nfa_to_dfa() {

    //let mut next_id: usize = 0;
    // let mut power_state_id_map = HashMap::<BTreeSet::<usize>, usize>::new();

    let mut alphabet = HashSet::new();

    let mut q0 = State::new(STATE_COUNTER.fetch_add(1, Ordering::SeqCst));
    q0.start_state = true;

    // let mut enfa = build_nfa(&mut alphabet); // 1|0
    //let mut enfa = build_nfa_2(&mut alphabet); // (a|b)*ab
    //let mut enfa = build_nfa_3(&mut alphabet); // d+c*
    let mut enfa = build_nfa_4(&mut alphabet); // hello | world
   
    enfa_to_dot_directed_graph(&mut enfa, "enfa_automaton.dot");

    let mut dfa = enfa_to_dfa(&mut enfa, &mut alphabet);

    enfa_to_dot_directed_graph(&mut dfa, "dfa_automaton.dot");

    //
    // Testing
    //

    let mut new_state_id = dfa.start_state_id;

    let str = "hello";
    // let str = "world";
    // let str = "hello world";
    for character in str.chars() { 
        println!("Input: {}", character);
        new_state_id = transition_dfa(&mut dfa, new_state_id, &RegexBuildingBlock::CharacterLiteral(character));

        if dfa.is_end_state(new_state_id) {
            println!("STATE '{}' END STATE!", new_state_id);
        } else {
            println!("STATE '{}' NOT END STATE!", new_state_id);
        }
    }
    if dfa.is_end_state(new_state_id) {
        println!("ACCEPTING '{}'! END STATE!", str);
    } else {
        println!("REJECTING '{}'! Not an end state!", str);
    }

    println!("done");
}

fn main() {
// fn combined_token_hex() {

    //
    // Pre-Build alphabet
    //

    let mut alphabet = HashSet::<RegexBuildingBlock>::new();

    alphabet.insert(RegexBuildingBlock::CharacterLiteral('a'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('b'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('c'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('d'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('e'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('f'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('g'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('h'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('i'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('j'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('k'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('l'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('m'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('n'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('o'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('p'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('q'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('r'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('s'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('t'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('u'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('v'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('w'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('x'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('y'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('z'));

    alphabet.insert(RegexBuildingBlock::CharacterLiteral('1'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('2'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('3'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('4'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('5'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('6'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('7'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('8'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('9'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('0'));

    alphabet.insert(RegexBuildingBlock::CharacterLiteral(' '));

    alphabet.insert(RegexBuildingBlock::CharacterLiteral('_'));

    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('\n'));

    //
    // Construct two separate keywords (hello and world)
    //

    // hello (toke-id: 277)

    let mut fragment_stack_1 = FragmentStack::new();
    add_character_literal(&mut fragment_stack_1, RegexBuildingBlock::CharacterLiteral('h'), &mut alphabet);
    add_character_literal(&mut fragment_stack_1, RegexBuildingBlock::CharacterLiteral('e'), &mut alphabet);
    add_concatenation(&mut fragment_stack_1);
    add_character_literal(&mut fragment_stack_1, RegexBuildingBlock::CharacterLiteral('l'), &mut alphabet);
    add_concatenation(&mut fragment_stack_1);
    add_character_literal(&mut fragment_stack_1, RegexBuildingBlock::CharacterLiteral('l'), &mut alphabet);
    add_concatenation(&mut fragment_stack_1);
    add_character_literal(&mut fragment_stack_1, RegexBuildingBlock::CharacterLiteral('o'), &mut alphabet);
    add_concatenation(&mut fragment_stack_1);
    let mut fragment_1 = fragment_stack_1.stack.pop().unwrap();
    fragment_1.enfa.states.get_mut(&fragment_1.end_id).unwrap().token_id = 277;

    enfa_to_dot_directed_graph(&mut fragment_1.enfa, "fragment_1_automaton.dot");

    // world (toke-id: 315)

    let mut fragment_stack_2 = FragmentStack::new();
    add_character_literal(&mut fragment_stack_2, RegexBuildingBlock::CharacterLiteral('w'), &mut alphabet);
    add_character_literal(&mut fragment_stack_2, RegexBuildingBlock::CharacterLiteral('o'), &mut alphabet);
    add_concatenation(&mut fragment_stack_2);
    add_character_literal(&mut fragment_stack_2, RegexBuildingBlock::CharacterLiteral('r'), &mut alphabet);
    add_concatenation(&mut fragment_stack_2);
    add_character_literal(&mut fragment_stack_2, RegexBuildingBlock::CharacterLiteral('l'), &mut alphabet);
    add_concatenation(&mut fragment_stack_2);
    add_character_literal(&mut fragment_stack_2, RegexBuildingBlock::CharacterLiteral('d'), &mut alphabet);
    add_concatenation(&mut fragment_stack_2);
    let mut fragment_2 = fragment_stack_2.stack.pop().unwrap();
    fragment_2.enfa.states.get_mut(&fragment_2.end_id).unwrap().token_id = 315;

    enfa_to_dot_directed_graph(&mut fragment_2.enfa, "fragment_2_automaton.dot");

    // int (toke-id: 777)

    let mut fragment_stack_3 = FragmentStack::new();
    add_character_literal(&mut fragment_stack_3, RegexBuildingBlock::CharacterLiteral('i'), &mut alphabet);
    add_character_literal(&mut fragment_stack_3, RegexBuildingBlock::CharacterLiteral('n'), &mut alphabet);
    add_concatenation(&mut fragment_stack_3);
    add_character_literal(&mut fragment_stack_3, RegexBuildingBlock::CharacterLiteral('t'), &mut alphabet);
    add_concatenation(&mut fragment_stack_3);
    let mut fragment_3 = fragment_stack_3.stack.pop().unwrap();
    fragment_3.enfa.states.get_mut(&fragment_3.end_id).unwrap().token_id = 777;

    enfa_to_dot_directed_graph(&mut fragment_3.enfa, "fragment_3_automaton.dot");

    // interop (toke-id: 888)

    let mut fragment_stack_4 = FragmentStack::new();
    add_character_literal(&mut fragment_stack_4, RegexBuildingBlock::CharacterLiteral('i'), &mut alphabet);
    add_character_literal(&mut fragment_stack_4, RegexBuildingBlock::CharacterLiteral('n'), &mut alphabet);
    add_concatenation(&mut fragment_stack_4);
    add_character_literal(&mut fragment_stack_4, RegexBuildingBlock::CharacterLiteral('t'), &mut alphabet);
    add_concatenation(&mut fragment_stack_4);
    add_character_literal(&mut fragment_stack_4, RegexBuildingBlock::CharacterLiteral('e'), &mut alphabet);
    add_concatenation(&mut fragment_stack_4);
    add_character_literal(&mut fragment_stack_4, RegexBuildingBlock::CharacterLiteral('r'), &mut alphabet);
    add_concatenation(&mut fragment_stack_4);
    add_character_literal(&mut fragment_stack_4, RegexBuildingBlock::CharacterLiteral('o'), &mut alphabet);
    add_concatenation(&mut fragment_stack_4);
    add_character_literal(&mut fragment_stack_4, RegexBuildingBlock::CharacterLiteral('p'), &mut alphabet);
    add_concatenation(&mut fragment_stack_4);
    
    // the top fragment on the fragment stack contains the root of the eNFA
    let mut fragment_4 = fragment_stack_4.stack.pop().unwrap();
    fragment_4.enfa.states.get_mut(&fragment_4.end_id).unwrap().token_id = 888;

    enfa_to_dot_directed_graph(&mut fragment_4.enfa, "fragment_4_automaton.dot");




    





/*
    let mut converter = InfixPostfixConverter::new();
    // assert_eq!("ab#", converter.infix_to_postfix("a(b)"));

    converter.infix_to_postfix("a(b)");
    //let root_node_id = converter.root_stack[converter.root_index].clone();

    let mut string_buffer = String::from("");
    let mut fragment_stack_5 = FragmentStack::new();
    recurse_postfix_build_fragment_stack(&converter.arena, &converter.root_node_id, /*&mut string_buffer,*/ &mut fragment_stack_5, &mut alphabet);
    println!("");

    // the top fragment on the fragment stack contains the root of the eNFA
    let mut fragment_5 = fragment_stack_5.stack.pop().unwrap();
    fragment_5.enfa.states.get_mut(&fragment_5.end_id).unwrap().token_id = 123;

    enfa_to_dot_directed_graph(&mut fragment_5.enfa, "fragment_5_automaton.dot");

    converter.reset();
*/



    let mut converter = InfixPostfixConverter::new();
    converter.infix_to_postfix("(_|a|b|c|d|e|f|g|h|i|j|k|l|m|n|o|p|q|r|s|t|u|v|w|x|y|z)(_|a|b|c|d|e|f|g|h|i|j|k|l|m|n|o|p|q|r|s|t|u|v|w|x|y|z)+");
    // converter.infix_to_postfix("(a|b|c)(a|b|c|1|2|3)+");

    //let root_node_id = converter.root_stack[converter.root_index].clone();

    // DEBUG - output the generated regex postfix notation
    // let mut string_buffer = String::from("");
    // recurse_postfix(&converter.arena, &converter.root_node_id, &mut string_buffer);
    // println!("{}", string_buffer);

    let mut fragment_stack_6 = FragmentStack::new();
    recurse_postfix_build_fragment_stack(&converter.arena, &converter.root_node_id, /*&mut string_buffer,*/ &mut fragment_stack_6, &mut alphabet);


    
    

    // the top fragment on the fragment stack contains the root of the eNFA
    let mut fragment_6 = fragment_stack_6.stack.pop().unwrap();
    // if fragment_6.end_id == 0 {
    //     // DEBUG - print to dot file format for debugging with https://dreampuf.github.io/GraphvizOnline
    //     enfa_to_dot_directed_graph(&mut fragment_6.enfa, "enfa_automaton.dot");
    //     panic!("test");
    // }
    fragment_6.enfa.states.get_mut(&fragment_6.end_id).unwrap().token_id = 500;

    
    // let mut done: bool = false;
    // while !done {
    //     match fragment_6.enfa.states.get_mut(&fragment_6.end_id) {
    //         Some(_) => {
    //             done = true;
    //         }
    //         None => {
    //             done = false;
    //         }
    //     }
    // }


    // the top fragment on the fragment stack contains the root of the eNFA
    // let mut fragment_6 = fragment_stack_6.stack.pop().unwrap();
    fragment_6.enfa.states.get_mut(&fragment_6.end_id).unwrap().token_id = 500;

    enfa_to_dot_directed_graph(&mut fragment_6.enfa, "fragment_6_automaton.dot");

    converter.reset();






    // ' ' (toke-id: 43)

    let mut fragment_stack_7 = FragmentStack::new();
    add_character_literal(&mut fragment_stack_7, RegexBuildingBlock::CharacterLiteral(' '), &mut alphabet);
    
    // the top fragment on the fragment stack contains the root of the eNFA
    let mut fragment_7 = fragment_stack_7.stack.pop().unwrap();
    fragment_7.enfa.states.get_mut(&fragment_7.end_id).unwrap().token_id = 15;

    enfa_to_dot_directed_graph(&mut fragment_7.enfa, "fragment_7_automaton.dot");





    // next steps:

    // using the recurse function, loop a fragment stack and an alphabet through the recursion
    // and let the recursion call add_concatenation(), add_character_literal(), ....

    // the result is a eNFA for the initial regex




    let mut combined_fragment = Fragment::new(RegexBuildingBlock::Or);

    // copy first keyword over (hello)
    let (start_id_1, end_id_1) = enfa_copy(&mut combined_fragment.enfa, &mut fragment_1.enfa, fragment_1.end_id);
    // copy second keyword over (world)
    let (start_id_2, end_id_2) = enfa_copy(&mut combined_fragment.enfa, &mut fragment_2.enfa, fragment_2.end_id);
    // copy third keyword over (int)
    let (start_id_3, end_id_3) = enfa_copy(&mut combined_fragment.enfa, &mut fragment_3.enfa, fragment_3.end_id);
    // copy fourth keyword over (interop)
    let (start_id_4, end_id_4) = enfa_copy(&mut combined_fragment.enfa, &mut fragment_4.enfa, fragment_4.end_id);
    // // copy 5th keyword over (ab)
    // let (start_id_5, end_id_5) = enfa_copy(&mut combined_fragment.enfa, &mut fragment_5.enfa, fragment_5.end_id);
    // copy 6th keyword over (identifier)
    let (start_id_6, end_id_6) = enfa_copy(&mut combined_fragment.enfa, &mut fragment_6.enfa, fragment_6.end_id);

    let (start_id_7, end_id_7) = enfa_copy(&mut combined_fragment.enfa, &mut fragment_7.enfa, fragment_7.end_id);

    // add epsilon transitions to all the keywords
    combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_1);
    combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_2);
    combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_3);
    combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_4);
    // combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_5);
    combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_6);
    combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_7);

    // DEBUG - print to dot file format for debugging with https://dreampuf.github.io/GraphvizOnline
    enfa_to_dot_directed_graph(&mut combined_fragment.enfa, "enfa_automaton.dot");

    //
    // Convert eNFA to DFA
    //
    let mut dfa = enfa_to_dfa(&mut combined_fragment.enfa, &mut alphabet);

    // DEBUG - print to dot file format for debugging with https://dreampuf.github.io/GraphvizOnline
    enfa_to_dot_directed_graph(&mut dfa, "dfa_automaton.dot");


    //
    // Testing
    //

    let mut current_state_id = dfa.start_state_id;
    let mut last_state_id = dfa.start_state_id;
    // println!("STATE '{}' START STATE!", current_state_id);

    // let str = "hello"; // hello (toke-id: 277)
    // let str = "world"; // world (toke-id: 315)
    //let str = "int"; // int (toke-id: 777)
    // let str = "interop"; // interop (toke-id: 888)
    // let str = "ab"; // interop (toke-id: 123)

    // let str = "_abc123abc"; // identifier (toke-id: 500)
    // let str = "abc123abc"; // identifier (toke-id: 500)

    // let str = "hello world";
    // let str = "inter";
    // let str = "int interop hello world _abc123abc";
    let str = "halt dein maul  du   furz";

    let mut token_string_buffer = String::from("");
    for character in str.chars() {

        let mut char_consumed = false;
        while !char_consumed {

            last_state_id = current_state_id;

            // println!("Input: {}", character);
            //token_string_buffer.push(character);

            // try to transition
            current_state_id = transition_dfa(&mut dfa, current_state_id, &RegexBuildingBlock::CharacterLiteral(character));

            if dfa.is_end_state(current_state_id) {

                // println!("STATE '{}' END STATE!", current_state_id);
                // println!("ACCEPTING '{}'! END STATE! Token-Id: {}", token_string_buffer, dfa.states[&current_state_id].token_id);

                token_string_buffer.push(character);

                char_consumed = true;

            } else if dfa.is_trap_state(current_state_id) {

                // reset the dfa to the start state and try to accept the symbol again
                char_consumed = false;
                current_state_id = dfa.start_state_id;

                println!("EMITTING '{}'. Token-Id: {}", token_string_buffer, dfa.states[&last_state_id].token_id);

                token_string_buffer.clear();

            } else {
                // println!("STATE '{}' NOT END STATE!", current_state_id);

                token_string_buffer.push(character);

                char_consumed = true;
            }
        }
    }

    println!("EMITTING '{}'. Token-Id: {}", token_string_buffer, dfa.states[&last_state_id].token_id);

    // if dfa.is_end_state(current_state_id) {
    //     println!("ACCEPTING '{}'! END STATE! Token-Id: {}", str, dfa.states[&current_state_id].token_id);
    // } else {
    //     println!("REJECTING '{}'! Not an end state!", str);
    // }

    println!("done");
}

// this is the glue code that interfaces the postfix regex tree with the eNFA construction
fn recurse_postfix_build_fragment_stack(arena: &Arena<RegexBuildingBlock>, 
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

    // // DEBUG
    // println!("{:?}", parent_node.data);

    match parent_node.data {

        // unescaped for processing, the special characters have to be escaped again for output
        RegexBuildingBlock::CharacterLiteral(c) => {

            match c {
                '|' => { panic!("test"); }
                '+' => { panic!("test"); }
                '-' => { panic!("test"); }
                '*' => { panic!("test"); }
                '^' => { panic!("test"); }

                _ => { 
                    //string_buffer.push_str(format!("{:?}", parent_node.data).as_str()); 
                    add_character_literal(fragment_stack, RegexBuildingBlock::CharacterLiteral(c), alphabet);
                }
            }

        }

        RegexBuildingBlock::Concatenation => {
            add_concatenation(fragment_stack);
        }

        RegexBuildingBlock::Or => {
            add_or(fragment_stack);
        }

        RegexBuildingBlock::Repeat(1, _) => {
            add_repeat_one_or_more(fragment_stack);
        }

        RegexBuildingBlock::ClosedBraces => {
            // nop
        }

        _ => {
            panic!("test {:?}", parent_node.data);
        }
    }
}

pub fn recurse_postfix(arena: &Arena<RegexBuildingBlock>, parent_node_id: &NodeId, string_buffer: &mut String) {

    let parent_node: &Node<RegexBuildingBlock> = &arena.nodes[parent_node_id.index];
    match &parent_node.left {
        Some(_) => {
            recurse_postfix(arena, parent_node.left.as_ref().unwrap(), string_buffer);
        }
        None => {
        }
    }
    match &parent_node.right {
        Some(_) => {
            recurse_postfix(arena, parent_node.right.as_ref().unwrap(), string_buffer);
        }
        None => {
        }
    }

    // // DEBUG
    // print!("{:?}", parent_node.data);

    match parent_node.data {

        // unescaped for processing, the special characters have to be escaped again for output
        RegexBuildingBlock::CharacterLiteral(c) => {
            match c {
                '|' => { string_buffer.push_str("\\|"); }
                '+' => { string_buffer.push_str("\\+"); }
                '-' => { string_buffer.push_str("\\-"); }
                '*' => { string_buffer.push_str("\\*"); }
                '^' => { string_buffer.push_str("\\^"); }
                _ => { string_buffer.push_str(format!("{:?}", parent_node.data).as_str()); }
            }
        }
        _ => {
            // output to string buffer
            string_buffer.push_str(format!("{:?}", parent_node.data).as_str());
        }
    }
    
    // output to string buffer
    //string_buffer.push_str(format!("{:?}", parent_node.data).as_str());
}

// 0x([0-9]|[A-F])+
// fn main() {
fn main_hex() {

    // complete alphabet has to be known in advance
    let mut alphabet = HashSet::<RegexBuildingBlock>::new();
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('/'));

    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('a'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('b'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('c'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('d'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('e'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('f'));

    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('g'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('h'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('i'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('j'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('k'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('l'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('m'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('n'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('o'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('p'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('q'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('r'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('s'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('t'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('u'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('v'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('w'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('x'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('y'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('z'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('1'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('2'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('3'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('4'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('5'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('6'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('7'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('8'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('9'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('0'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral(' '));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('\n'));

    let mut fragment_stack = FragmentStack::new();

    //
    // 0x #
    add_character_literal(&mut fragment_stack, RegexBuildingBlock::CharacterLiteral('0'), &mut alphabet);
    add_character_literal(&mut fragment_stack, RegexBuildingBlock::CharacterLiteral('x'), &mut alphabet);
    add_concatenation(&mut fragment_stack);
    
    //
    // ([0-9]|[A-F])+
    add_character_literal(&mut fragment_stack, RegexBuildingBlock::CharacterLiteral('0'), &mut alphabet);
    add_character_literal(&mut fragment_stack, RegexBuildingBlock::CharacterLiteral('1'), &mut alphabet);
    add_or(&mut fragment_stack);
    add_character_literal(&mut fragment_stack, RegexBuildingBlock::CharacterLiteral('2'), &mut alphabet);
    add_or(&mut fragment_stack);
    add_character_literal(&mut fragment_stack, RegexBuildingBlock::CharacterLiteral('3'), &mut alphabet);
    add_or(&mut fragment_stack);
    add_character_literal(&mut fragment_stack, RegexBuildingBlock::CharacterLiteral('4'), &mut alphabet);
    add_or(&mut fragment_stack);
    add_character_literal(&mut fragment_stack, RegexBuildingBlock::CharacterLiteral('5'), &mut alphabet);
    add_or(&mut fragment_stack);
    add_character_literal(&mut fragment_stack, RegexBuildingBlock::CharacterLiteral('6'), &mut alphabet);
    add_or(&mut fragment_stack);
    add_character_literal(&mut fragment_stack, RegexBuildingBlock::CharacterLiteral('7'), &mut alphabet);
    add_or(&mut fragment_stack);
    add_character_literal(&mut fragment_stack, RegexBuildingBlock::CharacterLiteral('8'), &mut alphabet);
    add_or(&mut fragment_stack);
    add_character_literal(&mut fragment_stack, RegexBuildingBlock::CharacterLiteral('9'), &mut alphabet);
    add_or(&mut fragment_stack);
    add_character_literal(&mut fragment_stack, RegexBuildingBlock::CharacterLiteral('A'), &mut alphabet);
    add_or(&mut fragment_stack);
    add_character_literal(&mut fragment_stack, RegexBuildingBlock::CharacterLiteral('B'), &mut alphabet);
    add_or(&mut fragment_stack);
    add_character_literal(&mut fragment_stack, RegexBuildingBlock::CharacterLiteral('C'), &mut alphabet);
    add_or(&mut fragment_stack);
    add_character_literal(&mut fragment_stack, RegexBuildingBlock::CharacterLiteral('D'), &mut alphabet);
    add_or(&mut fragment_stack);
    add_character_literal(&mut fragment_stack, RegexBuildingBlock::CharacterLiteral('E'), &mut alphabet);
    add_or(&mut fragment_stack);
    add_character_literal(&mut fragment_stack, RegexBuildingBlock::CharacterLiteral('F'), &mut alphabet);
    add_or(&mut fragment_stack);
    add_repeat_one_or_more(&mut fragment_stack);

    // // DEBUG
    // let mut top_fragment = fragment_stack.stack.pop().unwrap();
    // enfa_to_dot_directed_graph(&mut top_fragment.enfa, "top_automaton.dot");

    //
    // concatenation two parts
    add_concatenation(&mut fragment_stack);




    let mut top_fragment = fragment_stack.stack.pop().unwrap();

    // println!("start-id: {}, end-id: {}", top_fragment.start_id, top_fragment.end_id);

    // // DEBUG
    // enfa_to_dot_directed_graph(&mut top_fragment.enfa, "enfa_automaton.dot");

    // make last state an end state
    //top_fragment.enfa.states.get_mut(&top_fragment.end_id).unwrap().end_state = true;

    // mark end state
    top_fragment.enfa.set_end_state(top_fragment.end_id, true);

    // DEBUG
    enfa_to_dot_directed_graph(&mut top_fragment.enfa, "enfa_automaton.dot");

    let mut dfa = enfa_to_dfa(&mut top_fragment.enfa, &mut alphabet);

    // DEBUG
    enfa_to_dot_directed_graph(&mut dfa, "dfa_automaton.dot");

    //
    // Testing
    //

    let mut new_state_id = dfa.start_state_id;

    // let str = "0x001";
    let str = "0x8001";
    for character in str.chars() { 
        println!("Input: {}", character);
        new_state_id = transition_dfa(&mut dfa, new_state_id, &RegexBuildingBlock::CharacterLiteral(character));

        if dfa.is_end_state(new_state_id) {
            println!("STATE '{}' END STATE!", new_state_id);
        } else {
            println!("STATE '{}' NOT END STATE!", new_state_id);
        }
    }
    if dfa.is_end_state(new_state_id) {
        println!("ACCEPTING '{}'! END STATE!", str);
    } else {
        println!("REJECTING '{}'! Not an end state!", str);
    }

    println!("done");
}

fn main_accept_comment() {
// fn main() {

    // complete alphabet has to be known in advance
    let mut alphabet = HashSet::<RegexBuildingBlock>::new();
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('/'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('a'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('b'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('c'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('d'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('e'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('f'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('g'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('h'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('i'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('j'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('k'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('l'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('m'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('n'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('o'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('p'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('q'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('r'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('s'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('t'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('u'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('v'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('w'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('x'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('y'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('z'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('1'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('2'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('3'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('4'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('5'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('6'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('7'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('8'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('9'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('0'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral(' '));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('\n'));

    let mut fragment_stack = FragmentStack::new();

    // //
    add_character_literal(&mut fragment_stack, RegexBuildingBlock::CharacterLiteral('/'), &mut alphabet);
    add_character_literal(&mut fragment_stack, RegexBuildingBlock::CharacterLiteral('/'), &mut alphabet);
    add_concatenation(&mut fragment_stack);

    // ^a (not an a, but empty string or any other string possible using the alphabet!)
    //add_character_literal(&mut fragment_stack, RegexBuildingBlock::CharacterLiteral('a'), &mut alphabet);
    add_character_literal(&mut fragment_stack, RegexBuildingBlock::CharacterLiteral('\n'), &mut alphabet);
    add_not(&mut fragment_stack, &mut alphabet);

    add_concatenation(&mut fragment_stack);

    // a
    // add_character_literal(&mut fragment_stack, RegexBuildingBlock::CharacterLiteral('a'), &mut alphabet);
    add_character_literal(&mut fragment_stack, RegexBuildingBlock::CharacterLiteral('\n'), &mut alphabet);

    // // ( a | b | c ) *
    // add_character_literal(&mut fragment_stack, RegexBuildingBlock::CharacterLiteral('a'), &mut alphabet);
    // add_character_literal(&mut fragment_stack, RegexBuildingBlock::CharacterLiteral('b'), &mut alphabet);
    // add_or(&mut fragment_stack);
    // add_character_literal(&mut fragment_stack, RegexBuildingBlock::CharacterLiteral('c'), &mut alphabet);
    // add_or(&mut fragment_stack);
    // add_repeat_zero_or_more(&mut fragment_stack);

    // concatenation two parts
    add_concatenation(&mut fragment_stack);

    let mut top_fragment = fragment_stack.stack.pop().unwrap();

    // println!("start-id: {}, end-id: {}", top_fragment.start_id, top_fragment.end_id);

    enfa_to_dot_directed_graph(&mut top_fragment.enfa, "enfa_automaton.dot");

    // make last state an end state
    //top_fragment.enfa.states.get_mut(&top_fragment.end_id).unwrap().end_state = true;

    // mark end state
    top_fragment.enfa.set_end_state(top_fragment.end_id, true);

    enfa_to_dot_directed_graph(&mut top_fragment.enfa, "enfa_automaton.dot");

    let mut dfa = enfa_to_dfa(&mut top_fragment.enfa, &mut alphabet);

    enfa_to_dot_directed_graph(&mut dfa, "dfa_automaton.dot");



    //
    // Testing
    //

    let mut new_state_id = dfa.start_state_id;
    for character in "// supra mayro kratt 64 is da best gem\n".chars() { 
        // println!("{} test", character);
        new_state_id = transition_dfa(&mut dfa, new_state_id, &RegexBuildingBlock::CharacterLiteral(character));
    }
    if dfa.is_end_state(new_state_id) {
        println!("ACCEPT! END STATE!");
    } else {
        println!("REJECTED! Not an end state!");
    }

    // let mut new_state_id = dfa.start_state_id;

    // new_state_id = transition_dfa(&mut dfa, new_state_id, &RegexBuildingBlock::CharacterLiteral('/'));
    // new_state_id = transition_dfa(&mut dfa, new_state_id, &RegexBuildingBlock::CharacterLiteral('/'));

    // //new_state_id = transition_dfa(&mut dfa, new_state_id, &RegexBuildingBlock::CharacterLiteral('a'));
    // // new_state_id = transition_dfa(&mut dfa, new_state_id, &RegexBuildingBlock::CharacterLiteral('b'));
    // // new_state_id = transition_dfa(&mut dfa, new_state_id, &RegexBuildingBlock::CharacterLiteral('c'));

    // new_state_id = transition_dfa(&mut dfa, new_state_id, &RegexBuildingBlock::CharacterLiteral('/'));

    // //new_state_id = transition_dfa(&mut dfa, new_state_id, &RegexBuildingBlock::CharacterLiteral('a'));
    // new_state_id = transition_dfa(&mut dfa, new_state_id, &RegexBuildingBlock::CharacterLiteral('\n'));
    
    // // new_state_id = transition_dfa(&mut dfa, new_state_id, &RegexBuildingBlock::CharacterLiteral('/'));
    // // new_state_id = transition_dfa(&mut dfa, new_state_id, &RegexBuildingBlock::CharacterLiteral('/'));
    // // new_state_id = transition_dfa(&mut dfa, new_state_id, &RegexBuildingBlock::CharacterLiteral('a'));
    // // new_state_id = transition_dfa(&mut dfa, new_state_id, &RegexBuildingBlock::CharacterLiteral('f'));
    // // new_state_id = transition_dfa(&mut dfa, new_state_id, &RegexBuildingBlock::CharacterLiteral('f'));

    // if dfa.is_end_state(new_state_id) {
    //     println!("ACCEPT! END STATE!");
    // } else {
    //     println!("REJECTED! Not an end state!");
    // }

    println!("done");
}

//fn main() {
fn main_not_test_1() {

    // complete alphabet has to be known in advance
    let mut alphabet = HashSet::<RegexBuildingBlock>::new();
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('a'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('f'));

    let mut fragment_stack = FragmentStack::new();

    // a
    add_character_literal(&mut fragment_stack, RegexBuildingBlock::CharacterLiteral('a'), &mut alphabet);

    // ^
    add_not(&mut fragment_stack, &mut alphabet);

    // f
    add_character_literal(&mut fragment_stack, RegexBuildingBlock::CharacterLiteral('f'), &mut alphabet);
    add_concatenation(&mut fragment_stack);

    let mut top_fragment = fragment_stack.stack.pop().unwrap();

    println!("start-id: {}, end-id: {}", top_fragment.start_id, top_fragment.end_id);

    enfa_to_dot_directed_graph(&mut top_fragment.enfa, "enfa_automaton.dot");

    // make last state an end state
    //top_fragment.enfa.states.get_mut(&top_fragment.end_id).unwrap().end_state = true;

    // mark end state
    top_fragment.enfa.set_end_state(top_fragment.end_id, true);

    enfa_to_dot_directed_graph(&mut top_fragment.enfa, "enfa_automaton.dot");

    let mut dfa = enfa_to_dfa(&mut top_fragment.enfa, &mut alphabet);

    enfa_to_dot_directed_graph(&mut dfa, "dfa_automaton.dot");

    // DEBUG
    // let current_state_id = dfa.start_state_id;
    //println!("{} -> {}", current_state_id, new_state_id);

    let mut new_state_id = dfa.start_state_id;
    
    new_state_id = transition_dfa(&mut dfa, new_state_id, &RegexBuildingBlock::CharacterLiteral('a'));
    // new_state_id = transition_dfa(&mut dfa, new_state_id, &RegexBuildingBlock::CharacterLiteral('f'));
    // new_state_id = transition_dfa(&mut dfa, new_state_id, &RegexBuildingBlock::CharacterLiteral('f'));

    if dfa.is_end_state(new_state_id) {
        println!("ACCEPT! END STATE!");
    } else {
        println!("REJECTED! Not an end state!");
    }

    println!("done");
}

fn main_6() {
// fn main() {

    // d+c*

    let mut alphabet = HashSet::<RegexBuildingBlock>::new();

    let mut fragment_stack = FragmentStack::new();

    add_character_literal(&mut fragment_stack, RegexBuildingBlock::CharacterLiteral('d'), &mut alphabet);
    add_repeat_one_or_more(&mut fragment_stack);

    add_character_literal(&mut fragment_stack, RegexBuildingBlock::CharacterLiteral('c'), &mut alphabet);
    add_repeat_zero_or_more(&mut fragment_stack);

    add_concatenation(&mut fragment_stack);

    let mut top_fragment = fragment_stack.stack.pop().unwrap();

    // make last state an end state
    top_fragment.enfa.states.get_mut(&top_fragment.end_id).unwrap().end_state = true;

    enfa_to_dot_directed_graph(&mut top_fragment.enfa, "enfa_automaton.dot");

    let mut dfa = enfa_to_dfa(&mut top_fragment.enfa, &mut alphabet);

    enfa_to_dot_directed_graph(&mut dfa, "dfa_automaton.dot");
}

//fn main() {
fn main_5() {

    let mut alphabet = HashSet::<RegexBuildingBlock>::new();

    let mut fragment_stack = FragmentStack::new();

    // a|b|c+
    add_character_literal(&mut fragment_stack, RegexBuildingBlock::CharacterLiteral('a'), &mut alphabet);
    add_character_literal(&mut fragment_stack, RegexBuildingBlock::CharacterLiteral('b'), &mut alphabet);
    add_or(&mut fragment_stack);
    add_character_literal(&mut fragment_stack, RegexBuildingBlock::CharacterLiteral('c'), &mut alphabet);
    add_repeat_one_or_more(&mut fragment_stack);
    add_or(&mut fragment_stack);

    let mut top_fragment = fragment_stack.stack.pop().unwrap();

    // make last state an end state
    top_fragment.enfa.states.get_mut(&top_fragment.end_id).unwrap().end_state = true;

    enfa_to_dot_directed_graph(&mut top_fragment.enfa, "enfa_automaton.dot");

    let mut dfa = enfa_to_dfa(&mut top_fragment.enfa, &mut alphabet);

    enfa_to_dot_directed_graph(&mut dfa, "dfa_automaton.dot");
}

//fn main() {
fn main_4() {

    let mut alphabet = HashSet::new();

    let mut fragment_stack = FragmentStack::new();

    // a|b|c
    add_character_literal(&mut fragment_stack, RegexBuildingBlock::CharacterLiteral('a'), &mut alphabet);
    add_character_literal(&mut fragment_stack, RegexBuildingBlock::CharacterLiteral('b'), &mut alphabet);
    add_or(&mut fragment_stack);
    add_character_literal(&mut fragment_stack, RegexBuildingBlock::CharacterLiteral('c'), &mut alphabet);
    add_or(&mut fragment_stack);

    let mut top_fragment = fragment_stack.stack.pop().unwrap();

    // make last state an end state
    top_fragment.enfa.states.get_mut(&top_fragment.end_id).unwrap().end_state = true;

    enfa_to_dot_directed_graph(&mut top_fragment.enfa, "enfa_automaton.dot");
}

//fn main() {
fn main_3() {

    let mut alphabet = HashSet::new();

    let mut fragment_stack = FragmentStack::new();

    // af#* b+ |

    // a
    let regex_building_block = RegexBuildingBlock::CharacterLiteral('a');
    add_character_literal(&mut fragment_stack, regex_building_block, &mut alphabet);
    // f
    let regex_building_block = RegexBuildingBlock::CharacterLiteral('f');
    add_character_literal(&mut fragment_stack, regex_building_block, &mut alphabet);
    // #
    add_concatenation(&mut fragment_stack);
    // *
    add_repeat_zero_or_more(&mut fragment_stack);
    
    // b
    let regex_building_block = RegexBuildingBlock::CharacterLiteral('b');
    add_character_literal(&mut fragment_stack, regex_building_block, &mut alphabet);
    // +
    add_repeat_one_or_more(&mut fragment_stack);

    // // *
    // add_repeat_zero_or_more(&mut fragment_stack);

    // |
    add_or(&mut fragment_stack);

    let mut top_fragment = fragment_stack.stack.pop().unwrap();

    // make last state an end state
    top_fragment.enfa.states.get_mut(&top_fragment.end_id).unwrap().end_state = true;

    enfa_to_dot_directed_graph(&mut top_fragment.enfa, "enfa_automaton.dot");

    // NEXT STEPS:
    // look at main_nfa_to_dfa() and extract all code for eNFA->DFA conversion into a function
    // Apply this function here

    let mut dfa = enfa_to_dfa(&mut top_fragment.enfa, &mut alphabet);

    enfa_to_dot_directed_graph(&mut dfa, "dfa_automaton.dot");
}

//fn main() {
fn main_2() {

    let mut alphabet = HashSet::<RegexBuildingBlock>::new();

    let mut fragment_stack = FragmentStack::new();

    // af#*b+#  example: afafb

    // a
    let regex_building_block = RegexBuildingBlock::CharacterLiteral('a');
    add_character_literal(&mut fragment_stack, regex_building_block, &mut alphabet);
    // f
    let regex_building_block = RegexBuildingBlock::CharacterLiteral('f');
    add_character_literal(&mut fragment_stack, regex_building_block, &mut alphabet);
    // #
    add_concatenation(&mut fragment_stack);
    // *
    add_repeat_zero_or_more(&mut fragment_stack);
    
    // b
    let regex_building_block = RegexBuildingBlock::CharacterLiteral('b');
    add_character_literal(&mut fragment_stack, regex_building_block, &mut alphabet);
    // +
    add_repeat_one_or_more(&mut fragment_stack);

    // #
    add_concatenation(&mut fragment_stack);

    let mut top_fragment = fragment_stack.stack.pop().unwrap();

    // make last state an end state
    top_fragment.enfa.states.get_mut(&top_fragment.end_id).unwrap().end_state = true;

    enfa_to_dot_directed_graph(&mut top_fragment.enfa, "enfa_automaton.dot");
}

fn main_1() {

    let mut alphabet = HashSet::<RegexBuildingBlock>::new();

    println!("start");

    //
    // Creating objects
    //

    let mut fragment_stack = FragmentStack::new();

    //
    // CharacterLiteral 'a'
    //

    let regex_building_block = RegexBuildingBlock::CharacterLiteral('a');
    add_character_literal(&mut fragment_stack, regex_building_block, &mut alphabet);

    //
    // CharacterLiteral '+'
    //
    
    //add_repeat_one_or_more(&mut enfa, &mut fragment_stack);

    //
    // CharacterLiteral 'b'
    //

    let regex_building_block = RegexBuildingBlock::CharacterLiteral('b');
    add_character_literal(&mut fragment_stack, regex_building_block, &mut alphabet);

    //
    // CharacterLiteral '+'
    //

    add_repeat_one_or_more(&mut fragment_stack);
    
    //
    // Concatenation
    //

    add_concatenation(&mut fragment_stack);

    
    //
    // CharacterLiteral 'c'
    //

    let regex_building_block = RegexBuildingBlock::CharacterLiteral('c');
    add_character_literal(&mut fragment_stack, regex_building_block, &mut alphabet);

    //add_concatenation(&mut fragment_stack);

    //
    // Or
    //

    add_or(&mut fragment_stack);

    /*
    //
    // Star, Asterisk, Repeat[0, std::u8::MAX]
    //

    add_repeat_zero_or_more(&mut enfa, &mut fragment_stack);
    */

    /*
    // CharacterLiteral 'd'
    let regex_building_block = RegexBuildingBlock::CharacterLiteral('d');
    add_character_literal(&mut enfa, &mut fragment_stack, regex_building_block);
    // Or
    add_or(&mut enfa, &mut fragment_stack);
    */

    let mut top_fragment = fragment_stack.stack.pop().unwrap();

    // make last state an end state
    top_fragment.enfa.states.get_mut(&top_fragment.end_id).unwrap().end_state = true;

    enfa_to_dot_directed_graph(&mut top_fragment.enfa, "enfa_automaton.dot");

    println!("end");
}