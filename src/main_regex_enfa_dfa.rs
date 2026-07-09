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
    recurse_postfix_build_fragment_stack(&converter.arena, &converter.root_node_id, &mut fragment_stack_6, &mut alphabet);


    
    

    // the top fragment on the fragment stack contains the root of the eNFA
    let mut fragment_6 = fragment_stack_6.stack.pop().unwrap();
    // if fragment_6.end_id == 0 {
    //     // DEBUG - print to dot file format for debugging with https://dreampuf.github.io/GraphvizOnline
    //     enfa_to_dot_directed_graph(&mut fragment_6.enfa, "enfa_automaton.dot");
    //     panic!("test");
    // }
    // assign a token id to eNFA so it will assign that token id to all token it accepts
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
    // fragment_6.enfa.states.get_mut(&fragment_6.end_id).unwrap().token_id = 500;

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



/*
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
*/

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