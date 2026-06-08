/*****************************************************************
 * 
 * This main file parses a .asm source code file just like an
 * assembler would. 
 * 
 * If you want to process a .hex file, use the main.rs_hex file.
 * 
******************************************************************/

use std::fs;

use pest::{Parser, iterators::{Pair, Pairs}};
use pest_derive::Parser;

//mod ast;
//use crate::{ast::visitor::Visitor};

//mod encoder;

//mod cpu;
//use crate::{cpu::cortex_m4::CortexM4};

#[derive(Parser)]
//#[grammar = "asm.pest"]
#[grammar = "c.pest"]
//#[grammar = "comments.pest"]
pub struct CSVParser;

fn recurse_pairs(/*visitor: &mut Visitor,*/ pairs: &mut Pairs<'_, Rule>, indent: usize) {

    // because ident_list is silent, the iterator will contain idents
    for mut pair in pairs {
        recurse_pair(/*visitor,*/ &mut pair, indent);
    }
}

// a pair is a combination of the rule which matched and a span of input
// print!("{}Rule: {:?}", indent_string, pair.as_rule());
// //println!("Span:    {:?}", pair.as_span());
// println!(", Text: '{}'", pair.as_str().trim());
fn recurse_pair(/*visitor: &mut Visitor,*/ pair: &mut Pair<'_, Rule>, indent: usize) {

    let _indent_string = "  ".repeat(indent);

    let pair_as_rule = pair.as_rule();

    /*
    //
    // before the recursion
    //

    match pair_as_rule {

        _ => { println!("BEFORE RECURSION> No Rule matches!"); }

    }
    */

    for mut inner_pair in pair.clone().into_inner() {
        recurse_pair(/*visitor,*/ &mut inner_pair, indent+1);
    }

    //
    // after the recursion
    //

    match pair_as_rule {
       
        Rule::single_line_comment_rule => {
            println!("single_line_comment_rule: {}", pair.as_str());
        }
 /*
        Rule::comment => {
            println!("comment: {}", pair.as_str());
        }
         */
/*
        Rule::single_line_comment_rule => {
            // println!("SINGLE_LINE_COMMENT FOUND!");
            println!("single_line_comment_rule: {}", pair.as_str());
        }

        Rule::multi_line_comment_rule => {
            // println!("MULTI_LINE_COMMENT FOUND!");
            println!("multi_line_comment_rule: {}", pair.as_str());
        }
*/
        


/*
        Rule::translation_unit => {
            println!("translation_unit: {}", pair.as_str());
        }
            */

        Rule::function_definition => {
            println!("function_definition: {}", pair.as_str());
        }

        /*
        Rule::conditional_execution => {
            // println!("conditional_execution FOUND!");
            println!("conditional_execution: {}", pair.as_str());
        }

        Rule::condition_flags_update_suffix => {
            // println!("condition_flags_update_suffix FOUND!");
            println!("condition_flags_update_suffix: {}", pair.as_str());
        }

        Rule::label => {
            // println!("label FOUND!");
            println!("label: {}", pair.as_str());
        }

        Rule::param => {
            // println!("param FOUND!");
            println!("param: {}", pair.as_str());
        }
*/
        

        Rule::IDENTIFIER => {
            // println!("IDENTIFIER FOUND!");
            println!("IDENTIFIER: {}", pair.as_str());

            //visitor.do_somtehing()
        }
/*
        Rule::register_list => {
            // println!("register_list FOUND!");
            println!("register_list: {}", pair.as_str());
        }
*/
 /*       Rule::cpu_instruction_line => {
            //println!("cpu_instruction_line: {}", pair.as_str());
        }

        Rule::assignment_expression => {
            println!("assignment_expression: {}", pair.as_str());
        }

        Rule::decimal_literal => {
            println!("decimal_literal: {}", pair.as_str());
        }

        Rule::expression_statement => {
            println!("expression_statement: {}", pair.as_str());
        },

        Rule::member_expression => {
            println!("member_expression: {}", pair.as_str());
        },

        Rule::call_expression => {
            println!("call_expression: {}", pair.as_str());
        },

        Rule::cover_call_expression_and_async_arrow_head => {
            println!("cover_call_expression_and_async_arrow_head: {}", pair.as_str());
        }

        Rule::object_literal => {
            println!("object_literal: {}", pair.as_str());
        },

        Rule::switch_statement => {
            println!("switch_statement: {}", pair.as_str());
        },

        Rule::string_literal => {
            println!("string_literal: {}", pair.as_str());
        },

        Rule::function_declaration => {
            println!("function_declaration: {}", pair.as_str());
        },

        Rule::lexical_declaration => {
            println!("lexical_declaration: {}", pair.as_str());
        }

        Rule::expression => {
            println!("expression: {}", pair.as_str());
        }

        Rule::lexical_binding => {
            println!("lexical_binding: {}", pair.as_str());
            // for mut inner_pair in pair.clone().into_inner() {
            //     match pair.as_rule() {
            //         _ => {}
            //     }
            // }
            for child in pair.clone().into_inner() {
                println!("Child: {:?}", child.as_rule());
                println!("Text: {:?}", child.as_str());
            }

            // identifier
            let child_0 = pair.clone().into_inner().nth(0);
            println!("child_0: {:?}", child_0.unwrap().as_str());

            // assigned expression
            let child_1 = pair.clone().into_inner().nth(1);
            if child_1.is_some() {
                println!("child_1: {:?}", child_1.unwrap().as_str());
            }

            // TODO: after ascending from the recursion, take the
            // latest node as assigned expression that was created by the recursion
        },
 */
        _ => { /*println!("No Rule matches!");*/ }
    }

}

fn main() {

    //let read_asm_file:bool = false;
    //let read_asm_file:bool = true;
    //if read_asm_file {

        let filename = "res/C/snippets/main.c";

        let src = fs::read_to_string(&filename).expect("Failed to read file");
        let parse_result = CSVParser::parse(Rule::translation_unit, &src);
        //let parse_result = CSVParser::parse(Rule::comments, &src);

        //println!("{:?}", parse_result);

        // there should be a single root node in the parsed tree
        let res: Result<Pairs<'_, Rule>, pest::error::Error<Rule>> = parse_result;

        // retrieve pairs
        let mut pairs = res.expect("Parsing failed");

        //let mut visitor: Visitor = Visitor::new();

        recurse_pairs(/*&mut visitor,*/ &mut pairs, 0);

        //let mut cortex_m4_cpu: CortexM4 = CortexM4::new();

        //for asm_line in vtor.asm_lines {

           //println!("asm_line: '{}'", asm_line);

            //cortex_m4_cpu.execute(&asm_line);

            //if cortex_m4_cpu.halt() {
            //    break;
            //}
        //}
   // }

}