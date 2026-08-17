/*
    // For this function to work, insert at least one rule into the identification_rules 
    // set of the grammar set prior to calling this function!
    //
    // This function will develop all rules in the identification_rules set into the closure 
    // of all rules that the parser can potentially activate on any input symbol when it is 
    // located in the state for which this function is called.
    // 
    // All these rules are inserted into the rules-set of the grammar state.
    //
    // This function fills the channel map with channels between rules.
    //
    // The states are created prior to calling this function. The states are created in the
    // same large loop that also calls this function.
    //
    // This function does not produce new states!
    pub fn unfold_grammar_state(&mut self, 
        grammar_rules: &Vec::<Rule<T>>,
        first: &BTreeMap<RuleElement::<T>, Vec::<RuleElement::<T>>>,
        nullable: &BTreeMap::<RuleElement::<T>, bool>,
        rule_channel_map: &mut HashMap::<usize, Vec::<Transition<T>>>,
    ) {

        let debug: bool = true;

        // DEBUG
        // if debug {
            if self.id == 18 {
                println!("{:?}", self);
                println!("test");
            }
        // }

        // scratchpad of rules to process
        let mut d_set = Vec::<Rule<T>>::new();
        d_set.append(&mut self.identification_rules.clone());

        // while scratchpad has rules on it, loop
        let mut done: bool = d_set.is_empty();
        while !done {

            // let mut current_rule: Rule<T> = d_set.pop().expect("Need at least one rule!");
            let mut current_rule: Rule<T> = d_set[0].clone();
            d_set.drain(0..1);

            // DEBUG
            if debug {
                println!("[unfold_grammar_state] State-ID: {}, current_rule: {}", self.id, current_rule);
            }

            // DEBUG
            //if self.id == 18 && current_rule.lhs == RuleElement::NonTerminal(String::from("cast_expression")) {
            //if self.id == 18 && current_rule.id == 73 {
            if self.id == 18 && current_rule.id == 72 {
                println!("test {:?}", current_rule);
            }

            // ignore consumed rules
            if current_rule.dot_idx >= current_rule.rhs.len() {
                done = d_set.is_empty();
                continue;
            }

            // the CLOSURE() operation will not develop rules that point to terminals since they
            // do not actively add a rule to the CLOSURE() itself but transition to other states (shift).
            match &current_rule.rhs[current_rule.dot_idx] {
                RuleElement::Terminal(terminal) => {
                    done = d_set.is_empty();
                    continue;
                }
                _ => {
                    // nop
                }
            }

            //
            // STEP 1 - collect all lookaheads for the RHS nonterminal
            //          Lookaheads are required for the parse table.
            //          In LALR(1) lookaheads are essential parts of a rule.
            //          The algorithm needs to build the rule plus it's lookaheads to produce valid rule items!
            //

            let mut current_lookahead = Vec::<RuleElement<T>>::new();

            // DEBUG
            if debug {
                println!("[unfold_grammar_state] Determining lookahead for Rule: {}. Rule has lookahead: {:?}", current_rule, current_rule.lookahead);
            }

            let mut empty_beta = false;

            // find beta, if there is no beta, lookahead is the rule's own lookahead
            if current_rule.dot_idx + 1 >= current_rule.rhs.len() {

                // empty beta
                if debug {
                    println!("[unfold_grammar_state] empty beta {:?}", current_rule.lookahead);
                }

                current_lookahead.append(&mut current_rule.lookahead);

                empty_beta = true;

            } else {

                // build FIRST(beta+rule.lookahead)

                // // DEBUG
                // println!("found beta");

                // // DEBUG
                // println!("[unfold_grammar_state] Current Rule: {:?}", current_rule);

                // BUG: beta is more than the first non terminal !!!!!!!!

                // Example: S -> A C B, #
                // The developing the rule for nonterminal A, needs to build First(beta+#)
                // and beta in this case is CB instead of just C!

                //let beta_idx = current_rule.dot_idx + 1;

                for beta_idx in (current_rule.dot_idx + 1)..current_rule.rhs.len() {

                    match &current_rule.rhs[beta_idx] {

                        RuleElement::NonTerminal(non_terminal) => {

                            // current_lookahead.push(grammar_rules[i].rhs[grammar_rules[i].dot_idx + 1].clone());
                            //panic!("test");

                            // // DEBUG
                            // println!("NonTerminal: {:?}, rule lookahead: {:?}", &current_rule.rhs[current_rule.dot_idx + 1], &current_rule.lookahead);

                            // TODO: retrieve FIRST(of nonterminal concat rule lookahead) and insert it into  current_lookahead
                            // TODO: what if concat rule lookahead has more than a single symbol????

                            //let temp = first.get(&current_rule.rhs[current_rule.dot_idx + 1]).expect("Compiler has no FIRST() information for NonTerminal: {}", current_rule.rhs[current_rule.dot_idx + 1]);

                            let temp_non_terminal = &current_rule.rhs[beta_idx];
                            let first_values_opt = first.get(temp_non_terminal);

                            println!("First-Set for non-terminal: '{:?}' is '{:?}'", temp_non_terminal, first_values_opt);

                            if current_rule.dot_idx + 1 >= current_rule.rhs.len() {
                                empty_beta = true;
                            }

                            match first_values_opt {
                                Some(first_values) => {
                                    if debug {
                                        println!("[unfold_grammar_state] first_values >> {:?}", first_values.clone());
                                        println!("[unfold_grammar_state] current_lookahead.append ++ {:?}", first_values.clone());
                                    }
                                    current_lookahead.append(&mut first_values.clone());
                                }
                                None => {
                                    panic!("[unfold_grammar_state] Compiler has no FIRST() information for NonTerminal: {:?}! Aborting!", current_rule.rhs[beta_idx]);
                                }
                            }

                            // if current nonterminal is nullable, proceed with the next symbol
                            // if the nonterminal is not nullable or a terminal is found, then
                            // the first operation returns that first character
                            if nullable.contains_key(&temp_non_terminal) && *nullable.get(&temp_non_terminal).unwrap() == false {
                                break;
                            }
                            
                        }

                        RuleElement::Terminal(terminal) => {

                            if debug {
                                println!("[unfold_grammar_state] current_lookahead.push + {:?}", current_rule.rhs[beta_idx].clone());
                            }

                            current_lookahead.push(current_rule.rhs[beta_idx].clone());

                            // experiment: if there is a terminal in beta, abort further lookahead search
                            break;
                        }

                        _ => { 
                            panic!("test");
                        }
                    }
                }
            }

            // DEBUG
            if debug {
                println!("[unfold_grammar_state] current lookahead: {:?}", current_lookahead);
            }

            // over all rules that unfold from the rule via REDUCE operations
            match &current_rule.rhs[current_rule.dot_idx] {

                // if the dot is points to a non-terminal, extend the rule set
                RuleElement::NonTerminal(non_terminal) => {

                    // DEBUG
                    if debug {
                        println!("");
                        println!("[unfold_grammar_state] Extending closure due to Rule: {} and NonTerminal '{}' with lookaheads '{:?}'", current_rule, non_terminal, current_lookahead);
                        println!("");
                    }

                    // DEBUG
                    // println!("non_terminal {}", non_terminal);
                    
                    // find all rules that have a LHS == the non-terminal and add them into the d_set
                    for i in 0..grammar_rules.len() {

                        // if this rule starts with the same nonterminal
                        if grammar_rules[i].lhs == RuleElement::<T>::NonTerminal(non_terminal.clone()) {

                            // DEBUG
                            if debug {
                                println!("");
                                println!("[unfold_grammar_state] Inserting into closure Rule: [{}] {} using lookaheads: {:?} because of source-rule-id: {}", grammar_rules[i].id, grammar_rules[i], current_lookahead, &current_rule.id);
                                println!("");
                            }

                            // let mut empty_beta = false;
                            // if grammar_rules[i].dot_idx + 1 >= grammar_rules[i].rhs.len() {
                            //     empty_beta = true;
                            // }

                            let mut contained_already = false;
                            for j in 0..self.rules.len() {

                                if self.rules[j] == grammar_rules[i] {

                                    if empty_beta {
                                        panic!("test");
                                    }

                                    // copy all lookahead symbols over!
                                    for la in &current_lookahead {

                                        if !self.rules[j].lookahead.contains(&la) {

                                            // DEBUG
                                            if debug {
                                                println!("[unfold_grammar_state] Inserting {:?} into rule {:?}", &la, &self.rules[j]);
                                            }

                                            //
                                            // Insert into rule_channel_map

                                            if !rule_channel_map.contains_key(&current_rule.id) {
                                                let channel_ends = Vec::<Transition<T>>::new();
                                                rule_channel_map.insert(current_rule.id, channel_ends);
                                            }
                                            // retrieve the vector of first symbols for the nonterminal and extend it
                                            let channel_ends = &mut rule_channel_map.get_mut(&current_rule.id).unwrap();

                                            // DEBUG
                                            if debug {
                                                println!("{:?}, {:?}", self.rules[j].id, RuleElement::<T>::NonTerminal(non_terminal.clone()));
                                                println!("");
                                            }

                                            channel_ends.push(Transition(self.rules[j].id, RuleElement::<T>::NonTerminal(non_terminal.clone())));

                                            //
                                            //

                                            // because d_set and self.rules are independent collections, we need to update both!
                                            let mut add_back = false;
                                            if d_set.contains(&self.rules[j]) {

                                                // https://stackoverflow.com/questions/26243025/how-to-remove-an-element-from-a-vector-given-the-element
                                                let index = d_set.iter().position(|x| *x == self.rules[j]).unwrap();
                                                d_set.remove(index);

                                                add_back = true;
                                            }

                                            self.rules[j].lookahead.push(la.clone());

                                            if add_back {
                                                d_set.push(self.rules[j].clone());
                                            }
                                        }
                                    }
                                       
                                    contained_already = true;
                                }
                            }

                            if contained_already {
                                continue;
                            }

                            // add new rule to state
                            let mut rule = grammar_rules[i].clone();

                            // CHECK THIS !!!!
                            // produce new id to distinguish all rules from each other for propagation
                            rule.id = RULE_COUNTER.fetch_add(1, Ordering::SeqCst);

                            // DEBUG
                            if debug {
                                // println!("");
                                println!("[unfold_grammar_state] Inserting into closure Rule: [{}] {} using lookaheads: {:?} because of source-rule-id: {}", rule.id, rule, current_lookahead, &current_rule.id);
                                println!("[unfold_grammar_state] Source rule: {:?}", current_rule);
                            }

                            //
                            // Insert into rule_channel_map

                            if !rule_channel_map.contains_key(&current_rule.id) {
                                let channel_ends = Vec::<Transition<T>>::new();
                                rule_channel_map.insert(current_rule.id, channel_ends);
                            }
                            // retrieve the vector of first symbols for the nonterminal and extend it
                            let channel_ends = &mut rule_channel_map.get_mut(&current_rule.id).unwrap();

                            channel_ends.push(Transition(rule.id, RuleElement::<T>::NonTerminal(non_terminal.clone())));

                            //
                            //

                            rule.lookahead.append(&mut current_lookahead.clone());

                            // only if beta is empty
                            if empty_beta {
                                rule.lookahead.append(&mut current_rule.lookahead.clone());
                            }
                            self.rules.push(rule.clone());

                            d_set.insert(0, rule);
                        }
                    }
                }

                _ => {
                    // nop
                }
            }

            done = d_set.is_empty();
        }
    }
*/




/*
    // //
    // // if (token-id: 110)
    // converter.infix_to_postfix("if");
    // let mut fragment_stack_if = FragmentStack::new();
    // recurse_postfix_build_fragment_stack(&converter.arena, &converter.root_node_id, &mut fragment_stack_if, &mut alphabet);
    // converter.reset();
    // let mut fragment_if = fragment_stack_if.stack.pop().unwrap();
    // fragment_if.enfa.states.get_mut(&fragment_if.end_id).unwrap().token_id = 110;
    // fragment_if.enfa.states.get_mut(&fragment_if.end_id).unwrap().token_name = String::from("IF");

    // //
    // // VOID (token-id: 200)
    // converter.infix_to_postfix("void");
    // let mut fragment_stack_void = FragmentStack::new();
    // recurse_postfix_build_fragment_stack(&converter.arena, &converter.root_node_id, &mut fragment_stack_void, &mut alphabet);
    // converter.reset();
    // let mut fragment_void = fragment_stack_void.stack.pop().unwrap();
    // fragment_void.enfa.states.get_mut(&fragment_void.end_id).unwrap().token_id = 200;
    // fragment_void.enfa.states.get_mut(&fragment_void.end_id).unwrap().token_name = String::from("VOID");

    // //
    // // INT (token-id: 210)
    // converter.infix_to_postfix("int");
    // let mut fragment_stack_int = FragmentStack::new();
    // recurse_postfix_build_fragment_stack(&converter.arena, &converter.root_node_id, &mut fragment_stack_int, &mut alphabet);
    // converter.reset();
    // let mut fragment_int = fragment_stack_int.stack.pop().unwrap();
    // fragment_int.enfa.states.get_mut(&fragment_int.end_id).unwrap().token_id = 210;
    // fragment_int.enfa.states.get_mut(&fragment_int.end_id).unwrap().token_name = String::from("INT");

    //
    // Whitespace
    // ' ' (toke-id: 15)
    let mut fragment_stack_whitespace = FragmentStack::new();
    add_character_literal(&mut fragment_stack_whitespace, RegexBuildingBlock::CharacterLiteral(' '), &mut alphabet);
    // the top fragment on the fragment stack contains the root of the eNFA
    let mut fragment_whitespace = fragment_stack_whitespace.stack.pop().unwrap();
    fragment_whitespace.enfa.states.get_mut(&fragment_whitespace.end_id).unwrap().token_id = 15;
    fragment_whitespace.enfa.states.get_mut(&fragment_whitespace.end_id).unwrap().token_name = String::from("WHITESPACE");

    //
    // OPENING_BRACKET (token-id: 20)
    converter.infix_to_postfix("\\(");
    let mut fragment_stack_opening_bracket = FragmentStack::new();
    recurse_postfix_build_fragment_stack(&converter.arena, &converter.root_node_id, &mut fragment_stack_opening_bracket, &mut alphabet);
    converter.reset();
    let mut fragment_opening_bracket = fragment_stack_opening_bracket.stack.pop().unwrap();
    fragment_opening_bracket.enfa.states.get_mut(&fragment_opening_bracket.end_id).unwrap().token_id = 20;
    fragment_opening_bracket.enfa.states.get_mut(&fragment_opening_bracket.end_id).unwrap().token_name = String::from("OPENING_BRACKET");

    //
    // CLOSING_BRACKET (token-id: 25)
    converter.infix_to_postfix("\\)");
    let mut fragment_stack_closing_bracket = FragmentStack::new();
    recurse_postfix_build_fragment_stack(&converter.arena, &converter.root_node_id, &mut fragment_stack_closing_bracket, &mut alphabet);
    converter.reset();
    let mut fragment_closing_bracket = fragment_stack_closing_bracket.stack.pop().unwrap();
    fragment_closing_bracket.enfa.states.get_mut(&fragment_closing_bracket.end_id).unwrap().token_id = 25;
    fragment_closing_bracket.enfa.states.get_mut(&fragment_closing_bracket.end_id).unwrap().token_name = String::from("CLOSING_BRACKET");

    //
    // OPENING_CURLY_BRACKET (token-id: 30)
    converter.infix_to_postfix("\\{");
    let mut fragment_stack_opening_squiggly_bracket = FragmentStack::new();
    recurse_postfix_build_fragment_stack(&converter.arena, &converter.root_node_id, &mut fragment_stack_opening_squiggly_bracket, &mut alphabet);
    converter.reset();
    let mut fragment_opening_squiggly_bracket = fragment_stack_opening_squiggly_bracket.stack.pop().unwrap();
    fragment_opening_squiggly_bracket.enfa.states.get_mut(&fragment_opening_squiggly_bracket.end_id).unwrap().token_id = 30;
    fragment_opening_squiggly_bracket.enfa.states.get_mut(&fragment_opening_squiggly_bracket.end_id).unwrap().token_name = String::from("OPENING_CURLY_BRACKET");

    //
    // CLOSING_CURLY_BRACKET (token-id: 35)
    converter.infix_to_postfix("\\}");
    let mut fragment_stack_closing_squiggly_bracket = FragmentStack::new();
    recurse_postfix_build_fragment_stack(&converter.arena, &converter.root_node_id, &mut fragment_stack_closing_squiggly_bracket, &mut alphabet);
    converter.reset();
    let mut fragment_closing_squiggly_bracket = fragment_stack_closing_squiggly_bracket.stack.pop().unwrap();
    fragment_closing_squiggly_bracket.enfa.states.get_mut(&fragment_closing_squiggly_bracket.end_id).unwrap().token_id = 35;
    fragment_closing_squiggly_bracket.enfa.states.get_mut(&fragment_closing_squiggly_bracket.end_id).unwrap().token_name = String::from("CLOSING_CURLY_BRACKET");

    //
    // OPENING_ANGULAR_BRACKET (token-id: 40)
    converter.infix_to_postfix("\\[");
    let mut fragment_stack_opening_angular_bracket = FragmentStack::new();
    recurse_postfix_build_fragment_stack(&converter.arena, &converter.root_node_id, &mut fragment_stack_opening_angular_bracket, &mut alphabet);
    converter.reset();
    let mut fragment_opening_angular_bracket = fragment_stack_opening_angular_bracket.stack.pop().unwrap();
    fragment_opening_angular_bracket.enfa.states.get_mut(&fragment_opening_angular_bracket.end_id).unwrap().token_id = 40;
    fragment_opening_angular_bracket.enfa.states.get_mut(&fragment_opening_angular_bracket.end_id).unwrap().token_name = String::from("OPENING_ANGULAR_BRACKET");

    //
    // CLOSING_ANGULAR_BRACKET (token-id: 45)
    converter.infix_to_postfix("\\]");
    let mut fragment_stack_closing_angular_bracket = FragmentStack::new();
    recurse_postfix_build_fragment_stack(&converter.arena, &converter.root_node_id, &mut fragment_stack_closing_angular_bracket, &mut alphabet);
    converter.reset();
    let mut fragment_closing_angular_bracket = fragment_stack_closing_angular_bracket.stack.pop().unwrap();
    fragment_closing_angular_bracket.enfa.states.get_mut(&fragment_closing_angular_bracket.end_id).unwrap().token_id = 45;
    fragment_closing_angular_bracket.enfa.states.get_mut(&fragment_closing_angular_bracket.end_id).unwrap().token_name = String::from("CLOSING_ANGULAR_BRACKET");

    //
    // Semicolon (token-id: 50)
    converter.infix_to_postfix(";");
    let mut fragment_stack_semicolon = FragmentStack::new();
    recurse_postfix_build_fragment_stack(&converter.arena, &converter.root_node_id, &mut fragment_stack_semicolon, &mut alphabet);
    converter.reset();
    let mut fragment_semicolon = fragment_stack_semicolon.stack.pop().unwrap();
    fragment_semicolon.enfa.states.get_mut(&fragment_semicolon.end_id).unwrap().token_id = 50;
    fragment_semicolon.enfa.states.get_mut(&fragment_semicolon.end_id).unwrap().token_name = String::from("SEMICOLON");

    //
    // Colon (token-id: 51)
    converter.infix_to_postfix(":");
    let mut fragment_stack_colon = FragmentStack::new();
    recurse_postfix_build_fragment_stack(&converter.arena, &converter.root_node_id, &mut fragment_stack_colon, &mut alphabet);
    converter.reset();
    let mut fragment_colon = fragment_stack_colon.stack.pop().unwrap();
    fragment_colon.enfa.states.get_mut(&fragment_colon.end_id).unwrap().token_id = 51;
    fragment_colon.enfa.states.get_mut(&fragment_colon.end_id).unwrap().token_name = String::from("COLON");

    // //
    // // QUESTION_MARK (token-id: 52)
    // converter.infix_to_postfix("?");
    // let mut fragment_stack_question_mark = FragmentStack::new();
    // recurse_postfix_build_fragment_stack(&converter.arena, &converter.root_node_id, &mut fragment_stack_question_mark, &mut alphabet);
    // converter.reset();
    // let mut fragment_question_mark = fragment_stack_question_mark.stack.pop().unwrap();
    // fragment_question_mark.enfa.states.get_mut(&fragment_question_mark.end_id).unwrap().token_id = 52;
    // fragment_question_mark.enfa.states.get_mut(&fragment_question_mark.end_id).unwrap().token_name = String::from("QUESTION_MARK");

    //
    // COMMA (token-id: 53)
    converter.infix_to_postfix(",");
    let mut fragment_stack_comma = FragmentStack::new();
    recurse_postfix_build_fragment_stack(&converter.arena, &converter.root_node_id, &mut fragment_stack_comma, &mut alphabet);
    converter.reset();
    let mut fragment_comma = fragment_stack_comma.stack.pop().unwrap();
    fragment_comma.enfa.states.get_mut(&fragment_comma.end_id).unwrap().token_id = 53;
    fragment_comma.enfa.states.get_mut(&fragment_comma.end_id).unwrap().token_name = String::from("COMMA");

    //
    // EQUALS_SIGN (token-id: 54)
    converter.infix_to_postfix("=");
    let mut fragment_stack_equals_sign = FragmentStack::new();
    recurse_postfix_build_fragment_stack(&converter.arena, &converter.root_node_id, &mut fragment_stack_equals_sign, &mut alphabet);
    converter.reset();
    let mut fragment_equals_sign = fragment_stack_equals_sign.stack.pop().unwrap();
    fragment_equals_sign.enfa.states.get_mut(&fragment_equals_sign.end_id).unwrap().token_id = 54;
    fragment_equals_sign.enfa.states.get_mut(&fragment_equals_sign.end_id).unwrap().token_name = String::from("EQUALS_SIGN");

    //
    // LessThan LT (token-id: 55)
    converter.infix_to_postfix("<");
    let mut fragment_stack_less_than = FragmentStack::new();
    recurse_postfix_build_fragment_stack(&converter.arena, &converter.root_node_id, &mut fragment_stack_less_than, &mut alphabet);
    converter.reset();
    let mut fragment_less_than = fragment_stack_less_than.stack.pop().unwrap();
    fragment_less_than.enfa.states.get_mut(&fragment_less_than.end_id).unwrap().token_id = 55;
    fragment_less_than.enfa.states.get_mut(&fragment_less_than.end_id).unwrap().token_name = String::from("LT");

    //
    // GreaterThan GT (token-id: 60)
    converter.infix_to_postfix(">");
    let mut fragment_stack_greater_than = FragmentStack::new();
    recurse_postfix_build_fragment_stack(&converter.arena, &converter.root_node_id, &mut fragment_stack_greater_than, &mut alphabet);
    converter.reset();
    let mut fragment_greater_than = fragment_stack_greater_than.stack.pop().unwrap();
    fragment_greater_than.enfa.states.get_mut(&fragment_greater_than.end_id).unwrap().token_id = 60;
    fragment_greater_than.enfa.states.get_mut(&fragment_greater_than.end_id).unwrap().token_name = String::from("GT");

    //
    // PLUS (token-id: 65)
    converter.infix_to_postfix("\\+");
    let mut fragment_stack_plus = FragmentStack::new();
    recurse_postfix_build_fragment_stack(&converter.arena, &converter.root_node_id, &mut fragment_stack_plus, &mut alphabet);
    converter.reset();
    let mut fragment_plus = fragment_stack_plus.stack.pop().unwrap();
    fragment_plus.enfa.states.get_mut(&fragment_plus.end_id).unwrap().token_id = 65;
    fragment_plus.enfa.states.get_mut(&fragment_plus.end_id).unwrap().token_name = String::from("PLUS");

    //
    // MINUS (token-id: 66)
    converter.infix_to_postfix("\\-");
    let mut fragment_stack_minus = FragmentStack::new();
    recurse_postfix_build_fragment_stack(&converter.arena, &converter.root_node_id, &mut fragment_stack_minus, &mut alphabet);
    converter.reset();
    let mut fragment_minus = fragment_stack_minus.stack.pop().unwrap();
    fragment_minus.enfa.states.get_mut(&fragment_minus.end_id).unwrap().token_id = 66;
    fragment_minus.enfa.states.get_mut(&fragment_minus.end_id).unwrap().token_name = String::from("MINUS");

    //
    // PERCENT (token-id: 67)
    converter.infix_to_postfix("%");
    let mut fragment_stack_percent = FragmentStack::new();
    recurse_postfix_build_fragment_stack(&converter.arena, &converter.root_node_id, &mut fragment_stack_percent, &mut alphabet);
    converter.reset();
    let mut fragment_percent = fragment_stack_percent.stack.pop().unwrap();
    fragment_percent.enfa.states.get_mut(&fragment_percent.end_id).unwrap().token_id = 67;
    fragment_percent.enfa.states.get_mut(&fragment_percent.end_id).unwrap().token_name = String::from("PERCENT");

    //
    // INC_OP (token-id: 68)
    converter.infix_to_postfix("\\+\\+");
    let mut fragment_stack_inc_op = FragmentStack::new();
    recurse_postfix_build_fragment_stack(&converter.arena, &converter.root_node_id, &mut fragment_stack_inc_op, &mut alphabet);
    converter.reset();
    let mut fragment_inc_op = fragment_stack_inc_op.stack.pop().unwrap();
    fragment_inc_op.enfa.states.get_mut(&fragment_inc_op.end_id).unwrap().token_id = 68;
    fragment_inc_op.enfa.states.get_mut(&fragment_inc_op.end_id).unwrap().token_name = String::from("INC_OP");

    // // DEBUG
    // enfa_to_dot_directed_graph(&mut fragment_fragment_whitespace.enfa, "fragment_hitespace_automaton.dot");

    //
    // Phase 2 - Combine all eNFA into a large eNFA
    //

    // // copy first keyword over (hello)
    // let (start_id_1, end_id_1) = enfa_copy(&mut combined_fragment.enfa, &mut fragment_1.enfa, fragment_1.end_id);
    // // copy second keyword over (world)
    // let (start_id_2, end_id_2) = enfa_copy(&mut combined_fragment.enfa, &mut fragment_2.enfa, fragment_2.end_id);
    // // copy third keyword over (int)
    // let (start_id_3, end_id_3) = enfa_copy(&mut combined_fragment.enfa, &mut fragment_3.enfa, fragment_3.end_id);
    // // copy fourth keyword over (interop)
    // let (start_id_4, end_id_4) = enfa_copy(&mut combined_fragment.enfa, &mut fragment_4.enfa, fragment_4.end_id);
    // // // copy 5th keyword over (ab)
    // // let (start_id_5, end_id_5) = enfa_copy(&mut combined_fragment.enfa, &mut fragment_5.enfa, fragment_5.end_id);
    // // copy 6th keyword over (identifier)
    // let (start_id_6, end_id_6) = enfa_copy(&mut combined_fragment.enfa, &mut fragment_6.enfa, fragment_6.end_id);
    // let (start_id_7, end_id_7) = enfa_copy(&mut combined_fragment.enfa, &mut fragment_7.enfa, fragment_7.end_id);

    let (start_id_identifier, end_id_identifier)                                = enfa_copy(&mut combined_fragment.enfa, &mut fragment_identifier.enfa, fragment_identifier.end_id);
    let (start_id_numeric, end_id_numeric)                                      = enfa_copy(&mut combined_fragment.enfa, &mut fragment_numeric.enfa, fragment_numeric.end_id);
    //let (start_id_return, end_id_return)                                        = enfa_copy(&mut combined_fragment.enfa, &mut fragment_return.enfa, fragment_return.end_id);
    // let (start_id_if, end_id_if)                                                = enfa_copy(&mut combined_fragment.enfa, &mut fragment_if.enfa, fragment_if.end_id);
    // let (start_id_void, end_id_void)                                            = enfa_copy(&mut combined_fragment.enfa, &mut fragment_void.enfa, fragment_void.end_id);
    // let (start_id_int, end_id_int)                                              = enfa_copy(&mut combined_fragment.enfa, &mut fragment_int.enfa, fragment_int.end_id);
    let (start_id_whitespace, end_id_whitespace)                                = enfa_copy(&mut combined_fragment.enfa, &mut fragment_whitespace.enfa, fragment_whitespace.end_id);
    let (start_id_opening_bracket, end_id_opening_bracket)                      = enfa_copy(&mut combined_fragment.enfa, &mut fragment_opening_bracket.enfa, fragment_opening_bracket.end_id);
    let (start_id_closing_bracket, end_id_closing_bracket)                      = enfa_copy(&mut combined_fragment.enfa, &mut fragment_closing_bracket.enfa, fragment_closing_bracket.end_id);
    let (start_id_opening_squiggly_bracket, end_id_opening_squiggly_bracket)    = enfa_copy(&mut combined_fragment.enfa, &mut fragment_opening_squiggly_bracket.enfa, fragment_opening_squiggly_bracket.end_id);
    let (start_id_closing_squiggly_bracket, end_id_closing_squiggly_bracket)    = enfa_copy(&mut combined_fragment.enfa, &mut fragment_closing_squiggly_bracket.enfa, fragment_closing_squiggly_bracket.end_id);
    let (start_id_opening_angular_bracket, end_id_opening_angular_bracket)      = enfa_copy(&mut combined_fragment.enfa, &mut fragment_opening_angular_bracket.enfa, fragment_opening_angular_bracket.end_id);
    let (start_id_closing_angular_bracket, end_id_closing_angular_bracket)      = enfa_copy(&mut combined_fragment.enfa, &mut fragment_closing_angular_bracket.enfa, fragment_closing_angular_bracket.end_id);
    let (start_id_semicolon, end_id_semicolon)                                  = enfa_copy(&mut combined_fragment.enfa, &mut fragment_semicolon.enfa, fragment_semicolon.end_id);
    let (start_id_colon, end_id_colon)                                          = enfa_copy(&mut combined_fragment.enfa, &mut fragment_colon.enfa, fragment_colon.end_id);
    // let (start_id_question_mark, end_id_question_mark)                          = enfa_copy(&mut combined_fragment.enfa, &mut fragment_question_mark.enfa, fragment_question_mark.end_id);
    let (start_id_comma, end_id_comma)                                          = enfa_copy(&mut combined_fragment.enfa, &mut fragment_comma.enfa, fragment_comma.end_id);
    let (start_id_equals_sign, end_id_equals_sign)                              = enfa_copy(&mut combined_fragment.enfa, &mut fragment_equals_sign.enfa, fragment_equals_sign.end_id);
    let (start_id_less_than, end_id_less_than)                                  = enfa_copy(&mut combined_fragment.enfa, &mut fragment_less_than.enfa, fragment_less_than.end_id);
    let (start_id_greater_than, end_id_greater_than)                            = enfa_copy(&mut combined_fragment.enfa, &mut fragment_greater_than.enfa, fragment_greater_than.end_id);
    let (start_id_plus, end_id_plus)                                            = enfa_copy(&mut combined_fragment.enfa, &mut fragment_plus.enfa, fragment_plus.end_id);
    let (start_id_minus, end_id_minus)                                          = enfa_copy(&mut combined_fragment.enfa, &mut fragment_minus.enfa, fragment_minus.end_id);
    let (start_id_percent, end_id_percent)                                      = enfa_copy(&mut combined_fragment.enfa, &mut fragment_percent.enfa, fragment_percent.end_id);
    let (start_id_inc_op, end_id_inc_op)                                        = enfa_copy(&mut combined_fragment.enfa, &mut fragment_inc_op.enfa, fragment_inc_op.end_id);

    // add epsilon transitions to all the individual keyword eNFAs
    combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_identifier);
    combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_numeric);
    combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_whitespace);
    // combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_return);
    // combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_if);
    // combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_void);
    // combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_int);
    combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_opening_bracket);
    combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_closing_bracket);
    combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_opening_squiggly_bracket);
    combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_closing_squiggly_bracket);
    combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_opening_angular_bracket);
    combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_closing_angular_bracket);
    combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_semicolon);
    combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_colon);
    // combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_question_mark);
    combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_comma);
    combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_equals_sign);
    combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_less_than);
    combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_greater_than);
    combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_plus);
    combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_minus);
    combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_percent);
    combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_inc_op);
*/

/*
    // void main() {}
    // VOID IDENTIFIER OPENING_BRACKET CLOSING_BRACKET OPENING_CURLY_BRACKET CLOSING_CURLY_BRACKET
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("VOID")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("IDENTIFIER")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("OPENING_BRACKET")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("CLOSING_BRACKET")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("OPENING_CURLY_BRACKET")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("CLOSING_CURLY_BRACKET")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Closure);
*/

    // // void main() { EXPRESSION_STOP; }
    // // VOID IDENTIFIER OPENING_BRACKET CLOSING_BRACKET OPENING_CURLY_BRACKET EXPRESSION_STOP SEMICOLON CLOSING_CURLY_BRACKET
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("VOID")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("IDENTIFIER")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("OPENING_BRACKET")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("CLOSING_BRACKET")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("OPENING_CURLY_BRACKET")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("EXPRESSION_STOP")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("SEMICOLON")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("CLOSING_CURLY_BRACKET")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Closure);

    // void main() { EXPRESSION_STOP; }
    // VOID IDENTIFIER OPENING_BRACKET CLOSING_BRACKET OPENING_CURLY_BRACKET EXPRESSION_STOP SEMICOLON CLOSING_CURLY_BRACKET
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("VOID")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("IDENTIFIER")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("OPENING_BRACKET")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("CLOSING_BRACKET")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("OPENING_CURLY_BRACKET")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("CAST_STOP")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("SEMICOLON")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("CLOSING_CURLY_BRACKET")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Closure);

    // void main() { SIZEOF ( VOID ); }
    // VOID IDENTIFIER OPENING_BRACKET CLOSING_BRACKET OPENING_CURLY_BRACKET SIZEOF OPENING_BRACKET VOID CLOSING_BRACKET SEMICOLON CLOSING_CURLY_BRACKET
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("VOID")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("IDENTIFIER")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("OPENING_BRACKET")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("CLOSING_BRACKET")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("OPENING_CURLY_BRACKET")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("SIZEOF")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("OPENING_BRACKET")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("VOID")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("CLOSING_BRACKET")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("SEMICOLON")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("CLOSING_CURLY_BRACKET")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Closure);

    // // void main() { IDENTIFIER = IDENTIFIER; }
    // // VOID IDENTIFIER OPENING_BRACKET CLOSING_BRACKET OPENING_CURLY_BRACKET IDENTIFIER EQUALS_SIGN IDENTIFIER SEMICOLON CLOSING_CURLY_BRACKET
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("VOID")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("IDENTIFIER")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("OPENING_BRACKET")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("CLOSING_BRACKET")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("OPENING_CURLY_BRACKET")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("IDENTIFIER")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("EQUALS_SIGN")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("IDENTIFIER")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("SEMICOLON")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("CLOSING_CURLY_BRACKET")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Closure);



    // let mut parse_table = HashMap::<usize, HashMap::<RuleElement<String>, ParseTableCell<usize>>>::new();

    // // state-id 0
    // let mut parse_table_row = HashMap::<RuleElement<String>, ParseTableCell<usize>>::new();
    // parse_table_row.insert(RuleElement::NonTerminal(String::from("a")), ParseTableCell::<usize>::Shift(2));
    // parse_table_row.insert(RuleElement::NonTerminal(String::from("z")), ParseTableCell::<usize>::Shift(4));
    // parse_table_row.insert(RuleElement::Terminal(String::from("S")), ParseTableCell::<usize>::Goto(1));
    // parse_table_row.insert(RuleElement::Terminal(String::from("B")), ParseTableCell::<usize>::Goto(3));
    // parse_table.insert(0, parse_table_row);

    // // state-id 1
    // let mut parse_table_row = HashMap::<RuleElement<String>, ParseTableCell<usize>>::new();
    // parse_table_row.insert(RuleElement::NonTerminal(String::from("#")), ParseTableCell::<usize>::Accept);
    // parse_table.insert(1, parse_table_row);

    // // state-id 2
    // let mut parse_table_row = HashMap::<RuleElement<String>, ParseTableCell<usize>>::new();
    // parse_table_row.insert(RuleElement::NonTerminal(String::from("z")), ParseTableCell::<usize>::Shift(7));
    // parse_table_row.insert(RuleElement::Terminal(String::from("A")), ParseTableCell::<usize>::Goto(5));
    // parse_table_row.insert(RuleElement::Terminal(String::from("B")), ParseTableCell::<usize>::Goto(6));
    // parse_table.insert(2, parse_table_row);

    // // state-id 3
    // let mut parse_table_row = HashMap::<RuleElement<String>, ParseTableCell<usize>>::new();
    // parse_table_row.insert(RuleElement::NonTerminal(String::from("c")), ParseTableCell::<usize>::Shift(8));
    // parse_table.insert(3, parse_table_row);

    // // state-id 4
    // let mut parse_table_row = HashMap::<RuleElement<String>, ParseTableCell<usize>>::new();
    // parse_table_row.insert(RuleElement::NonTerminal(String::from("c")), ParseTableCell::<usize>::Reduce(14));
    // parse_table.insert(4, parse_table_row);

    // // state-id 5
    // let mut parse_table_row = HashMap::<RuleElement<String>, ParseTableCell<usize>>::new();
    // parse_table_row.insert(RuleElement::NonTerminal(String::from("c")), ParseTableCell::<usize>::Shift(9));
    // parse_table.insert(5, parse_table_row);

    // // state-id 6
    // let mut parse_table_row = HashMap::<RuleElement<String>, ParseTableCell<usize>>::new();
    // parse_table_row.insert(RuleElement::NonTerminal(String::from("d")), ParseTableCell::<usize>::Shift(10));
    // parse_table.insert(6, parse_table_row);

    // // state-id 7
    // let mut parse_table_row = HashMap::<RuleElement<String>, ParseTableCell<usize>>::new();
    // parse_table_row.insert(RuleElement::NonTerminal(String::from("c")), ParseTableCell::<usize>::Reduce(20));
    // parse_table_row.insert(RuleElement::NonTerminal(String::from("d")), ParseTableCell::<usize>::Reduce(19));
    // parse_table.insert(7, parse_table_row);

    // // state-id 8
    // let mut parse_table_row = HashMap::<RuleElement<String>, ParseTableCell<usize>>::new();
    // parse_table_row.insert(RuleElement::NonTerminal(String::from("#")), ParseTableCell::<usize>::Reduce(21));
    // parse_table.insert(8, parse_table_row);

    // // state-id 9
    // let mut parse_table_row = HashMap::<RuleElement<String>, ParseTableCell<usize>>::new();
    // parse_table_row.insert(RuleElement::NonTerminal(String::from("#")), ParseTableCell::<usize>::Reduce(22));
    // parse_table.insert(9, parse_table_row);

    // // state-id 10
    // let mut parse_table_row = HashMap::<RuleElement<String>, ParseTableCell<usize>>::new();
    // parse_table_row.insert(RuleElement::NonTerminal(String::from("#")), ParseTableCell::<usize>::Reduce(23));
    // parse_table.insert(10, parse_table_row);

    // let parse_table_row = parse_table.get(&0);
    // let parser_step = parse_table_row.expect("Parse Table is broken!").get(&RuleElement::Terminal(String::from("S"))).unwrap();




    /*
    let lexer_debug: bool = true;

    let mut current_state_id = dfa.start_state_id;
    let mut last_state_id = dfa.start_state_id;

    let mut current_character: char = 'x';
    let mut lookahead_character: char = 'y';
    let mut has_lookahead_character = false;

    let mut token_string_buffer = String::from("");
    for character in str.chars() {

        current_character = lookahead_character;
        lookahead_character = character;

        if !has_lookahead_character {
            has_lookahead_character = true;
            continue;
        }

        let mut char_consumed = false;
        while !char_consumed {

            last_state_id = current_state_id;

            if lexer_debug {
                println!("[LEXER] Input: '{}', lookahead: '{}'", current_character, lookahead_character);
            }

            // try to transition the large lexer DFA to produce a token for the input
            current_state_id = transition_dfa(&mut dfa, current_state_id, &RegexBuildingBlock::CharacterLiteral(current_character));

            if dfa.is_end_state(current_state_id) {

                // println!("STATE '{}' END STATE!", current_state_id);
                // println!("ACCEPTING '{}'! END STATE! Token-Id: {}", token_string_buffer, dfa.states[&current_state_id].token_id);

                token_string_buffer.push(current_character);

                char_consumed = true;

            } else if dfa.is_trap_state(current_state_id) {

                // check if there is a valid transition for the next character
                // greedily consume it and do not directly feed a half finished token to the parser
                println!("[LEXER] Lookahead character is: '{}'", lookahead_character);
                // if  {

                // }

                // reset the lexer's DFA back to the start state and 
                // try to accept the symbol again which was read from input already
                char_consumed = false;
                current_state_id = dfa.start_state_id;

                if lexer_debug {
                    println!("[LEXER] Emitting '{}', Token-Id: {}, Token-Name: {}", token_string_buffer, dfa.states[&last_state_id].token_id, dfa.states[&last_state_id].token_name);
                    println!("");
                }

                let terminal = RuleElement::Terminal(dfa.states[&last_state_id].token_name.clone());

                //if lexer_debug {
                    println!("[LEXER] {:?} ---> {:?}", token_string_buffer, terminal);
                //}

                match dfa.states[&last_state_id].token_id {
                    
                    NEWLINE_TOKEN_ID | WHITESPACE_TOKEN_ID => {
                        // ignore NEWLINE and WHITESPACE
                        // nop
                    }
                    _ => {
                        // pass token to the lexer
                        provide_input(&mut parser, 
                            &grammar_state_hashmap, 
                            &mut step, 
                            &terminal);
                    }
                }

                token_string_buffer.clear();

            } else {
                // println!("STATE '{}' NOT END STATE!", current_state_id);

                token_string_buffer.push(current_character);

                char_consumed = true;
            }
        }
    }
    */






    /*
                // collect all rules which the normal rule points to within the same state
                // retrieve all channels for the current rule
                let dest_rule_transitions = rule_channel_map.get(&src_rule_id).unwrap();
                for transition in dest_rule_transitions {

                    let target_state = rule_id_to_state_id_map.get(&transition.0).unwrap();

                    // needs to be within same state
                    if *target_state == state_id {

                        // // push target rule id
                        // if !processed_rule_ids.contains(&transition.0) {

                            println!("{} -> {}", src_rule_id, &transition.0);

                            let mut source_rule_idx = 0;
                            let mut target_rule_idx = 0;

                            // insert lookahead into rule if not contained already
                            for jj in 0..state.rules.len() {
                                if state.rules[jj].id == src_rule_id {
                                    println!("source rule found!");
                                    source_rule_idx = jj;
                                }
                                if state.rules[jj].id == transition.0 {
                                    println!("target rule found!");
                                    target_rule_idx = jj;
                                }
                            }

                            let mut ttttt = state.rules[source_rule_idx].lookahead.clone();
                            state.rules[target_rule_idx].lookahead.append(&mut ttttt);

                            if !local_rule_ids.contains(&transition.0) {
                                local_rule_ids.push(transition.0);
                            }
                        // }
                    }
                }*/






                /*
    let first_state: &mut GrammarState<String> = grammar_state_hashmap.get_mut(&first_state_id).unwrap();

    println!("{:?}", &first_state);

    for g in 0..first_state.rules.len() {

        let src_rule_id = first_state.rules[g].id;

        let dest_rule_transitions = rule_channel_map.get(&src_rule_id).unwrap();
        for transition in dest_rule_transitions {

            let target_state = rule_id_to_state_id_map.get(&transition.0).unwrap();

            // needs to be within same state
            if *target_state == first_state_id {

                // // push target rule id
                // if !processed_rule_ids.contains(&transition.0) {

                    println!("{} -> {}", src_rule_id, &transition.0);

                    let mut source_rule_idx = 0;
                    let mut target_rule_idx = 0;

                    // insert lookahead into rule if not contained already
                    for jj in 0..first_state.rules.len() {
                        if first_state.rules[jj].id == src_rule_id {
                            println!("source rule found!");
                            source_rule_idx = jj;
                        }
                        if first_state.rules[jj].id == transition.0 {
                            println!("target rule found!");
                            target_rule_idx = jj;
                        }
                    }

                    let mut ttttt = first_state.rules[source_rule_idx].lookahead.clone();
                    first_state.rules[target_rule_idx].lookahead.append(&mut ttttt);

                    // if !local_rule_ids.contains(&transition.0) {
                    //     local_rule_ids.push(transition.0);
                    // }
                // }
            }
        }
    }
        println!("{:?}", &first_state);
    */



    // // DEBUG
                                // if dest_state_id == 50 && *la == RuleElement::Terminal(String::from("ELSE")) {
                                //     println!("({} -> {}) Dirty: {} because '{:?}' has been pushed into one of it's identifying rules", src_state_id, dest_state_id, dest_state_id, la.clone());
                                //     println!("test");
                                // }

                                // if dest_state_id == 31 && *la == RuleElement::Terminal(String::from("ELSE")) {
                                //     println!("({} -> {}) Dirty: {} because '{:?}' has been pushed into one of it's identifying rules", src_state_id, dest_state_id, dest_state_id, la.clone());
                                //     println!("test");
                                // }

                                // if dest_state_id == 22 && *la == RuleElement::Terminal(String::from("ELSE")) {
                                //     println!("({} -> {}) Dest-Rule: {}, Dirty: {} because '{:?}' has been pushed into one of it's identifying rules", src_state_id, dest_state_id, dest_rule_id.0, dest_state_id, la.clone());
                                //     println!("test");
                                // }

                                // if dest_state_id == 56 && *la == RuleElement::Terminal(String::from("ELSE")) {
                                //     println!("({} -> {}) Dirty: {} because '{:?}' has been pushed into one of it's identifying rules", src_state_id, dest_state_id, dest_state_id, la.clone());
                                //     println!("test");
                                // }

                                // if dest_state_id == 48 && *la == RuleElement::Terminal(String::from("ELSE")) {
                                //     println!("({} -> {}) Dirty: {} because '{:?}' has been pushed into one of it's identifying rules", src_state_id, dest_state_id, dest_state_id, la.clone());
                                //     println!("test");
                                // }

                                // if dest_state_id == 27 && *la == RuleElement::Terminal(String::from("ELSE")) {
                                //     println!("({} -> {}) Dirty: {} because '{:?}' has been pushed into one of it's identifying rules", src_state_id, dest_state_id, dest_state_id, la.clone());
                                //     println!("test");
                                // }

                                // if dest_state_id == 21 && *la == RuleElement::Terminal(String::from("ELSE")) {
                                //     println!("({} -> {}) Dirty: {} because '{:?}' has been pushed into one of it's identifying rules", src_state_id, dest_state_id, dest_state_id, la.clone());
                                //     println!("test");
                                // }





                            /*                
                //
                // Step 3 - check if the channel points to a normal rule
                //

                //
                // This is the same code as for identifying rules above.
                // Try to find 
                //

                println!("{:?}", dest_state);

                if dest_rule_id.0 == 33 {
                    println!("test");
                }

                for i in 0..dest_state.rules.len() {

                    if dest_state.rules[i].id == dest_rule_id.0 {

                        // copy lookaheads into dest rule
                        let temp_rule = src_rule.first().unwrap();
                        println!("{}", temp_rule);
                        for la in &temp_rule.lookahead {

                            if *la == RuleElement::NonTerminal(String::from(")")) {
                                println!("test");
                            }

                            // do not forward the end symbol within the same state, only inter states
                            //if src_state_id == dest_state_id {
                            //if *src_state_id == 0 as usize {
                                // if *la == RuleElement::Closure {
                                //     continue;
                                // }
                            //}

                            // do not forward in start state
                            // if src_state_id == dest_state_id && *src_state_id == 0 as usize {
                            //     continue;
                            // }

                            // within the same state only propagate if state currently has the dirty flag
                            if *src_state_id == dest_state_id && !dirty_state_ids.contains(&dest_state_id) {
                                // println!("no change to {}", dest_state_id);
                                // println!("");
                                continue;
                            }
                            
                            println!("Updating dirty state: {} {:?}", dest_state_id, la.clone());
                            if !dest_state.rules[i].lookahead.contains(&la) {
                                dest_state.rules[i].lookahead.push(la.clone());
                                change_detected = true;
                            }
                        }
                    }
                }

                // println!("Test");
*/



// // void main() { RETURN 0; }
    // // VOID IDENTIFIER OPENING_BRACKET CLOSING_BRACKET OPENING_CURLY_BRACKET RETURN NUMERIC SEMICOLON CLOSING_CURLY_BRACKET
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("VOID")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("IDENTIFIER")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("OPENING_BRACKET")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("CLOSING_BRACKET")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("OPENING_CURLY_BRACKET")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("RETURN")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("NUMERIC")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("SEMICOLON")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("CLOSING_CURLY_BRACKET")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Closure);
/*
    let mut consumed = false;

    consumed = false;
    while !consumed {
        println!("");
        println!("Step {}", step);
        consumed = parser.consume(RuleElement::Terminal(String::from("a")), &grammar_state_hashmap);
        step = step + 1;
    }

    consumed = false;
    while !consumed {
        println!("");
        println!("Step {}", step);
        consumed = parser.consume(RuleElement::Terminal(String::from("z")), &grammar_state_hashmap);
        step = step + 1;
    }

    consumed = false;
    while !consumed {
        println!("");
        println!("Step {}", step);
        consumed = parser.consume(RuleElement::Terminal(String::from("c")), &grammar_state_hashmap);
        step = step + 1;
    }

    consumed = false;
    while !consumed {
        println!("");
        println!("Step {}", step);
        //consumed = parser.consume(RuleElement::Terminal(String::from("#")), &grammar_state_hashmap);
        consumed = parser.consume(RuleElement::Closure, &grammar_state_hashmap);
        step = step + 1;
    }
*/

/*
    consumed = false;
    while !consumed {
        println!("");
        println!("Step {}", step);
        consumed = parser.consume(RuleElement::Terminal(String::from("a")), &grammar_state_hashmap);
        step = step + 1;
    }

    consumed = false;
    while !consumed {
        println!("");
        println!("Step {}", step);
        consumed = parser.consume(RuleElement::Terminal(String::from("c")), &grammar_state_hashmap);
        step = step + 1;
    }

    consumed = false;
    while !consumed {
        println!("");
        println!("Step {}", step);
        consumed = parser.consume(RuleElement::Terminal(String::from("b")), &grammar_state_hashmap);
        step = step + 1;
    }

    consumed = false;
    while !consumed {
        println!("");
        println!("Step {}", step);
        //consumed = parser.consume(RuleElement::Terminal(String::from("#")), &grammar_state_hashmap);
        consumed = parser.consume(RuleElement::Closure, &grammar_state_hashmap);
        step = step + 1;
    }
*/

/*
    // IF ( EXPRESSION ) TEST_STMT ELSE TEST_STMT
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("IF")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("(")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("EXPRESSION")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from(")")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("TEST_STMT")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("ELSE")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("TEST_STMT")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Closure);
*/

/*
    // IF ( EXPRESSION ) TEST_STMT ELSE IF ( EXPRESSION ) TEST_STMT ELSE TEST_STMT
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("IF")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("(")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("EXPRESSION")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from(")")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("TEST_STMT")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("ELSE")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("IF")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("(")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("EXPRESSION")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from(")")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("TEST_STMT")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("ELSE")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("TEST_STMT")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Closure);
*/

/*
    // * id = id
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("*")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("id")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("=")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("id")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Closure);
*/

/*
    //  n - ( n )
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("n")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("-")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("(")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("n")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from(")")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Closure);
*/

/*
    // a b a b
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("a")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("b")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("a")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("b")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Closure);
*/

/*
    // a z d #
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("a")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("z")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("d")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Closure);
*/

/*
    // a c b #
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("a")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("c")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("b")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Closure);
*/

/*
    // d a h g #
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("d")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("a")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("h")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("g")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Closure);
*/





/*
// Equality gets a special implementation just because Terminals store their grammar type
// along with their actual value. For example an INT terminal has the value 5.
// As in rust, the second value becomes part of the type system, the standard equality
// implementation compares the grammar type of a token as well as the actual value.
// In this implementation, the second value is reused as a container for data and
// should not be part of the terminal's type! So this custom equality implementation
// just looks at the grammar type for terminals.
impl<T: std::cmp::PartialEq> PartialEq<RuleElement<T>> for RuleElement<T> {

    fn eq(&self, other: &RuleElement<T>) -> bool {

        match &self {
            RuleElement::<T>::Terminal(self_terminal_identifier, self_terminal_data_value) => {
                match &other {
                    RuleElement::<T>::Terminal(other_terminal_identifier, other_terminal_data_value) => {
                        let result = self_terminal_identifier == other_terminal_identifier;
                        result
                    }
                    _ => {
                        //PartialEq::eq(self, other)
                        false
                    }
                }
            }
            RuleElement::<T>::NonTerminal(self_nonterminal_identifier) => {
                match &other {
                    RuleElement::<T>::NonTerminal(other_nonterminal_identifier) => {
                        let result = self_nonterminal_identifier == other_nonterminal_identifier;
                        result
                    }
                    _ => {
                        false
                    }
                }
            }
            RuleElement::<T>::Epsilon => {
                match &other {
                    RuleElement::<T>::Epsilon => {
                        true
                    }
                    _ => {
                        false
                    }
                }
            }
            RuleElement::<T>::Dot => {
                match &other {
                    RuleElement::<T>::Dot => {
                        true
                    }
                    _ => {
                        false
                    }
                }
            }
            RuleElement::<T>::AcceptingStateTransition => {
                match &other {
                    RuleElement::<T>::AcceptingStateTransition => {
                        true
                    }
                    _ => {
                        false
                    }
                }
            }
            RuleElement::<T>::Closure => {
                match &other {
                    RuleElement::<T>::Closure => {
                        true
                    }
                    _ => {
                        false
                    }
                }
            }
            RuleElement::<T>::Unused => {
                match &other {
                    RuleElement::<T>::Unused => {
                        true
                    }
                    _ => {
                        false
                    }
                }
            }
            RuleElement::<T>::Unknown => {
                match &other {
                    RuleElement::<T>::Unknown => {
                        true
                    }
                    _ => {
                        false
                    }
                }
            }
        }
    }
}
*/


/*
// TODO: the lookahead character is not used at all!
// Remove it! It makes the parser loop more complicated
fn consume_character(dfa: &mut EpsilonNfa::<State, RegexBuildingBlock>, 
    mut current_state_id: usize, 
    token_string_buffer: &mut String, 
    current_character: char, 
    lookahead_character: char,
    step: &mut usize,
    parser: &mut Parser::<String>,
    grammar_state_hashmap: &BTreeMap<usize, GrammarState<String>>,
    string_buffer: &mut String,
    debug_node_stack: &mut Vec::<DebugNode>) -> usize {

    let lexer_debug = false;

    // // check if there is a valid transition for the next character
    // // greedily consume it and do not directly feed a half finished token to the parser
    // if lexer_debug {
    //     println!("[LEXER.TRAP_STATE] Lookahead character is: '{}'", lookahead_character);
    // }

    // let mut current_state_id = dfa.start_state_id;
    // let mut last_state_id = dfa.start_state_id;
    let mut next_state_id = current_state_id;

    let mut char_consumed = false;
    while !char_consumed {

        // last_state_id = current_state_id;

        if lexer_debug {
            println!("[LEXER] State; '{}', Input: '{}', lookahead: '{}'", current_state_id, current_character, lookahead_character);
        }

        //
        // try to transition the large lexer DFA to produce a token for the input.
        // If the input has no valid transition, the DFA transitions into a trap state.
        //

        next_state_id = transition_dfa(dfa, current_state_id, &RegexBuildingBlock::CharacterLiteral(current_character));

        if lexer_debug {
            println!("[LEXER] From State: '{}', To State: '{}'", current_state_id, next_state_id);
        }

        //
        // Next, check where the DFA has transitioned to
        //

        if dfa.is_trap_state(next_state_id) {

            if lexer_debug {
                println!("[LEXER.TRAP_STATE] Emitting '{}', Token-Id: {}, Token-Name: {}", token_string_buffer, dfa.states[&current_state_id].token_id, dfa.states[&current_state_id].token_name);
                println!("");
            }

            // create a Token / Terminal
            let terminal = RuleElement::Terminal(dfa.states[&current_state_id].token_name.clone());

            if lexer_debug {
                println!("[LEXER.TRAP_STATE] {:?} ---> {:?}", token_string_buffer, terminal);
            }

            match dfa.states[&current_state_id].token_id {
                
                NEWLINE_TOKEN_ID | WHITESPACE_TOKEN_ID => {
                    // ignore NEWLINE and WHITESPACE
                    if lexer_debug {
                        println!("[LEXER.TRAP_STATE] NOT Passing token to parser: {:?}, {:?}", token_string_buffer, terminal);
                    }
                }

                IDENTIFIER_TOKEN_ID => {
                    if lexer_debug {
                        println!("[LEXER.TRAP_STATE] Passing token to parser: {:?}, {:?}", token_string_buffer, terminal);
                    }

                    // TODO: check some type of datastructure for token here!!!!!!!
                    // asdfasfdsdf
                    if token_string_buffer == "point_t" {

                        // pass token to the lexer
                        provide_input(parser, 
                            grammar_state_hashmap, 
                            step, 
                            &RuleElement::Terminal(String::from("TYPE_NAME")),
                            &token_string_buffer,
                            string_buffer,
                            debug_node_stack);

                    } else {
                        // pass token to the lexer
                        provide_input(parser, 
                            grammar_state_hashmap, 
                            step, 
                            &terminal,
                            &token_string_buffer,
                            string_buffer,
                            debug_node_stack);
                    }
                }

                _ => {
                    if lexer_debug {
                        println!("[LEXER.TRAP_STATE] Passing token to parser: {:?}, {:?}", token_string_buffer, terminal);
                    }

                    // pass token to the lexer
                    provide_input(parser, 
                        grammar_state_hashmap, 
                        step, 
                        &terminal,
                        &token_string_buffer,
                        string_buffer,
                        debug_node_stack);
                }
            }

            // reset the lexer's DFA back to the start state and 
            // try to accept the symbol again which was read from input already
            char_consumed = false;
            current_state_id = dfa.start_state_id;
            token_string_buffer.clear();

        } else if dfa.is_end_state(next_state_id) { 
            
            //
            // if the state is normal or an end state, just consume the character
            //

            // DEBUG
            if lexer_debug {
                println!("[LEXER] Emitting '{}', Token-Id: {}, Token-Name: {}", token_string_buffer, dfa.states[&next_state_id].token_id, dfa.states[&next_state_id].token_name);
            }

            token_string_buffer.push(current_character);

            char_consumed = true;

        } else {

            //
            // if the state is normal or an end state, just consume the character
            //

            // DEBUG
            // println!("STATE '{}' NOT END STATE!", current_state_id);

            token_string_buffer.push(current_character);

            char_consumed = true;
        }
    }

    next_state_id
}
*/

/*
let temp_rule_element_1 = RuleElement::<String>::Terminal(String::from("abc"));
    let temp_rule_element_2 = RuleElement::<String>::Terminal(String::from("abc"));

    let mut temp_table = HashMap::<RuleElement<String>, usize>::new();
    temp_table.insert(temp_rule_element_1, 1usize);

    if temp_table.contains_key(&temp_rule_element_2) {
        println!("Test");
    } else {
        println!("Test2");
    }
        */



        





// let mut stack_offset_value = 0i32;

                // match &asm_ast_instruction.src.operand_type {

                //     AsmAstOperandType::Stack(stack_offset) => {
                //         // println!("Stack, offset:{}", stack_offset);
                //     }

                //     AsmAstOperandType::Pseudo(pseudo_name) => {
                //         // println!("Pseudo");

                //         if self.replace_pseudo {

                //             if self.stack_offset_map.contains_key(pseudo_name) {

                //                 stack_offset_value = *self.stack_offset_map.get(pseudo_name).unwrap();

                //             } else {

                //                 self.stack_offset = self.stack_offset - 4;
                //                 self.stack_offset_map.insert(pseudo_name.to_string(), self.stack_offset);
                                
                //                 stack_offset_value = self.stack_offset;
                                
                //             }
                //         }

                //         asm_ast_instruction.src = AsmAstOperand { operand_type: AsmAstOperandType::Stack(stack_offset_value) };
                //     }

                //     AsmAstOperandType::Imm(immediate_value) => {
                //         // println!("Imm");
                //     }

                //     AsmAstOperandType::Reg(register_name) => {
                //         // println!("Reg, register_name:{:?}", register_name);
                //     }

                //     // AsmAstOperandType::Reg(AsmAstReg),
                //     // AsmAstOperandType::Pseudo(String),
                //     // AsmAstOperandType::Stack(i32),

                //     // ValueElement::Constant(value) => {
                //     //     println!("Constant: {:?}", value);
                //     // }

                //     // ValueElement::Variable(value) => {
                //     //     // println!("Variable: {:?}", value);

                //     //     if self.replace_pseudo {

                //     //         if self.stack_offset_map.contains_key(value) {

                //     //             value_test = *self.stack_offset_map.get(value).unwrap();

                //     //         } else {

                //     //             self.stack_offset = self.stack_offset - 4;
                //     //             self.stack_offset_map.insert(value.to_string(), self.stack_offset);
                                
                //     //             value_test = self.stack_offset;
                                
                //     //         }
                //     //     }
                //     // }

                //     _ => {
                //         // panic!("Test");
                //         panic!("{}", format!("Unhandled InstructionType {:?}!\n", asm_ast_instruction.src.operand_type ).as_str());
                //     }

                //     // asm_ast_instruction.src = ValueElement::Variable("contains".to_string());
                //     // asm_ast_instruction.src = ValueElement::Variable(value_test.to_string());
                //     // asm_ast_instruction.src = ValueElement::Variable(self.stack_offset.to_string());
                // }

                





                // match &asm_ast_instruction.dst.operand_type {

                //     AsmAstOperandType::Stack(stack_offset) => {
                //         // println!("Stack, offset:{}", stack_offset);
                //     }

                //     AsmAstOperandType::Pseudo(pseudo_name) => {
                //         // println!("Pseudo");

                //         if self.replace_pseudo {

                //             if self.stack_offset_map.contains_key(pseudo_name) {

                //                 stack_offset_value = *self.stack_offset_map.get(pseudo_name).unwrap();

                //             } else {

                //                 self.stack_offset = self.stack_offset - 4;
                //                 self.stack_offset_map.insert(pseudo_name.to_string(), self.stack_offset);
                                
                //                 stack_offset_value = self.stack_offset;
                                
                //             }
                //         }

                //         asm_ast_instruction.dst = AsmAstOperand { operand_type: AsmAstOperandType::Stack(stack_offset_value) };
                //     }

                //     AsmAstOperandType::Imm(immediate_value) => {
                //         // println!("Imm");
                //     }

                //     AsmAstOperandType::Reg(register_name) => {
                //         // println!("Reg, register_name:{:?}", register_name);
                //     }

                //     // AsmAstOperandType::Reg(AsmAstReg),
                //     // AsmAstOperandType::Pseudo(String),
                //     // AsmAstOperandType::Stack(i32),

                //     // ValueElement::Constant(value) => {
                //     //     println!("Constant: {:?}", value);
                //     // }

                //     // ValueElement::Variable(value) => {
                //     //     // println!("Variable: {:?}", value);

                //     //     if self.replace_pseudo {

                //     //         if self.stack_offset_map.contains_key(value) {

                //     //             value_test = *self.stack_offset_map.get(value).unwrap();

                //     //         } else {

                //     //             self.stack_offset = self.stack_offset - 4;
                //     //             self.stack_offset_map.insert(value.to_string(), self.stack_offset);
                                
                //     //             value_test = self.stack_offset;
                                
                //     //         }
                //     //     }
                //     // }

                //     _ => {
                //         // panic!("Test");
                //         panic!("{}", format!("Unhandled InstructionType {:?}!\n", asm_ast_instruction.dst.operand_type ).as_str());
                //     }

                //     // asm_ast_instruction.src = ValueElement::Variable("contains".to_string());
                //     // asm_ast_instruction.src = ValueElement::Variable(value_test.to_string());
                //     // asm_ast_instruction.src = ValueElement::Variable(self.stack_offset.to_string());
                // }



// let mut stack_offset_value = 0i32;

                // match &asm_ast_instruction.dst.operand_type {

                //     AsmAstOperandType::Stack(stack_offset) => {
                //         // println!("Stack, offset:{}", stack_offset);
                //     }

                //     AsmAstOperandType::Pseudo(pseudo_name) => {
                //         // println!("Pseudo");

                //         if self.replace_pseudo {

                //             if self.stack_offset_map.contains_key(pseudo_name) {

                //                 stack_offset_value = *self.stack_offset_map.get(pseudo_name).unwrap();

                //             } else {

                //                 self.stack_offset = self.stack_offset - 4;
                //                 self.stack_offset_map.insert(pseudo_name.to_string(), self.stack_offset);
                                
                //                 stack_offset_value = self.stack_offset;
                                
                //             }
                //         }

                //         asm_ast_instruction.src = AsmAstOperand { operand_type: AsmAstOperandType::Stack(stack_offset_value) };
                //     }

                //     AsmAstOperandType::Imm(immediate_value) => {
                //         // println!("Imm");
                //     }

                //     _ => {
                //         // panic!("Test");
                //         panic!("{}", format!("Unhandled InstructionType {:?}!\n", asm_ast_instruction.src.operand_type ).as_str());
                //     }
                // }


                



/*
        match ast_node.operator_type {

            AstNodeOperatorType::Addition => {
                println!("visit!");
                // binary_instruction.binary_operator = BinaryOperator::Add;
            }

            AstNodeOperatorType::Subtraction => {
                println!("visit!");
                // binary_instruction.binary_operator = BinaryOperator::Subtract;
            }

            AstNodeOperatorType::Multiplication => {
                println!("visit!");
                // binary_instruction.binary_operator = BinaryOperator::Multiply;
            }

            AstNodeOperatorType::Division => {
                println!("visit!");
                // binary_instruction.binary_operator = BinaryOperator::Division;
            }

            AstNodeOperatorType::Remainder => {
                println!("visit!");
                // binary_instruction.binary_operator = BinaryOperator::Remainder;
            }

            AstNodeOperatorType::LessThan => {
                println!("visit!");
                // binary_instruction.binary_operator = BinaryOperator::LessThan;
            }

            AstNodeOperatorType::GreaterThan => {
                println!("visit!");
                // binary_instruction.binary_operator = BinaryOperator::GreaterThan;
            }

            AstNodeOperatorType::Equal => {
                println!("visit!");
                // binary_instruction.binary_operator = BinaryOperator::Equal;
            }

            AstNodeOperatorType::NotEqual => {
                println!("visit!");
                // binary_instruction.binary_operator = BinaryOperator::NotEqual;
            }

            AstNodeOperatorType::LessThanOrEqual => {
                println!("visit!");
                // binary_instruction.binary_operator = BinaryOperator::LessThanOrEqual;
            }

            AstNodeOperatorType::GreaterThanOrEqual => {
                println!("visit!");
                // binary_instruction.binary_operator = BinaryOperator::GreaterThanOrEqual;
            }

            AstNodeOperatorType::NotApplicable => {
                println!("visit!");
            }

            _ => {
                panic!("{}", format!("Unhandled ast_node.operator_type {:?}!\n", ast_node.operator_type).as_str());
            }
        }
        */