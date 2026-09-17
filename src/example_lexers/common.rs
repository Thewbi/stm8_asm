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

// This function will extend the input combined_fragment by a whole new sub-DFA for
// the input regex_infix. In order to extend the combined_fragment the following steps
// are executed:
//
// 1. Convert the input regex_infix from infix to postfix notation using the converter.
// 2. The postfix notation is used as a guidance rule for building fragments on the
//    fragment stack. The fragment stack eventually contains a single fragment that
//    points to the start and end state of an automaton for the input regex.
//    See README.md in the base folder for a detailed explanation of the algorithms used.
// 3. Finalize the resulting fragment. State id and token name are inserted.
// 4. Insert the new fragment to all prior regexes stored in the combined_fragment.
// 5. Make the new fragment accessible from the old start state using an epsilon transition
//
// The result is a NFA which is able to accept the new regex.
// SideNote: The NFA can be converted to a DFA in another step.
pub fn add_token_definition(converter: &mut InfixPostfixConverter,
    combined_fragment: &mut Fragment,
    alphabet: &mut HashSet::<RegexBuildingBlock>,
    regex_infix: &str,
    token_name: &str,
    token_id: usize) {

    // 1. convert the input regex_infix from infix to postfix notation using the converter
    converter.infix_to_postfix(regex_infix);

    // 2. The postfix notation is used as a guidance rule for building fragments on the
    //    fragment stack. The fragment stack eventually contains a single fragment that
    //    points to the start and end state of an automaton for the input regex.
    let mut fragment_stack_return = FragmentStack::new();
    recurse_postfix_build_fragment_stack(&converter.arena, &converter.root_node_id, &mut fragment_stack_return, alphabet);
    converter.reset();

    // 3. finalize the resulting fragment
    let mut fragment_return = fragment_stack_return.stack.pop().unwrap();
    fragment_return.enfa.states.get_mut(&fragment_return.end_id).unwrap().token_id = token_id;
    fragment_return.enfa.states.get_mut(&fragment_return.end_id).unwrap().token_name = String::from(token_name);

    // 4. combine the new fragment into the combined fragment to insert the new DFA to the existing DFA
    //    which represents all prior regexes added so far
    let (start_id_return, end_id_return) = enfa_copy(&mut combined_fragment.enfa, &mut fragment_return.enfa, fragment_return.end_id);

    // 5. make the new fragment accessible from the old start state using an epsilon transition
    combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_return);
}