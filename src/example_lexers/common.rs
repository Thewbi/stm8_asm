use std::collections::HashSet;

use crate::EpsilonNfa;
use crate::regex::enfa::enfa_copy;
use crate::State;
use crate::RegexBuildingBlock;
use crate::regex::enfa::Fragment;
use crate::regex::enfa::FragmentStack;
use crate::InfixPostfixConverter;
use crate::regex::enfa::recurse_postfix_build_fragment_stack;
use crate::Input;

pub fn add_token_definition(converter: &mut InfixPostfixConverter, 
    combined_fragment: &mut Fragment,
    alphabet: &mut HashSet::<RegexBuildingBlock>,
    regex_infix: &str, 
    token_name: &str, 
    token_id: usize) {
    
    converter.infix_to_postfix(regex_infix);
    let mut fragment_stack_return = FragmentStack::new();
    recurse_postfix_build_fragment_stack(&converter.arena, &converter.root_node_id, &mut fragment_stack_return, alphabet);
    converter.reset();
    let mut fragment_return = fragment_stack_return.stack.pop().unwrap();
    fragment_return.enfa.states.get_mut(&fragment_return.end_id).unwrap().token_id = token_id;
    fragment_return.enfa.states.get_mut(&fragment_return.end_id).unwrap().token_name = String::from(token_name);

    let (start_id_return, end_id_return) = enfa_copy(&mut combined_fragment.enfa, &mut fragment_return.enfa, fragment_return.end_id);

    combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_return);
}