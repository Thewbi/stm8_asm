// filename: main_lr_1.rs

#![allow(
dead_code,
unused_imports,
unused_must_use,
unused_variables,
unused_assignments
)]

use std::collections::{HashMap, HashSet, BTreeSet, BTreeMap};
use std::hash::Hash;
use std::{
    sync::atomic::{AtomicUsize, Ordering}
};

use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;

fn main() {

    println!("start");

    // https://cyberzhg.github.io/toolbox/lr1
    // 
    // S -> S S + 
    // S -> S S * 
    // S -> a

    // https://cyberzhg.github.io/toolbox/lr1
    //
    // S -> X X
    // X -> a X
    // X -> b

    // https://cyberzhg.github.io/parsing-toys/tools/cfg_lr1.html#eyJncmFtbWFyIjoiIiwiaW5wdXQiOiIifQ==

    // INPUT: VOID IDENTIFIER ( ) { SUPER SEMICOLON }

translation_unit -> function_definition

function_definition -> declaration_specifiers declarator compound_statement

declaration_specifiers -> type_specifier declaration_specifiers
declaration_specifiers -> type_specifier
declaration_specifiers -> type_qualifier declaration_specifiers
declaration_specifiers -> type_qualifier

type_specifier -> VOID
type_specifier -> INT

type_qualifier -> CONST
type_qualifier -> VOLATILE

declarator -> direct_declarator

direct_declarator -> IDENTIFIER direct_declarator
direct_declarator -> IDENTIFIER
direct_declarator -> ( declarator )
direct_declarator -> ( )

compound_statement -> { declaration_or_statement_list }
compound_statement -> { }

declaration_or_statement_list -> declaration declaration_or_statement_list
declaration_or_statement_list -> declaration
declaration_or_statement_list -> statement declaration_or_statement_list
declaration_or_statement_list -> statement

declaration -> declaration_specifiers init_declarator_list SEMICOLON
declaration -> declaration_specifiers SEMICOLON

init_declarator_list -> init_declarator COMMA init_declarator_list
init_declarator_list -> init_declarator

init_declarator -> declarator EQUALS_SIGN initializer
init_declarator -> declarator

statement -> expression_statement

expression_statement -> expression SEMICOLON
expression_statement -> SEMICOLON

//expression -> SUPER
expression -> assignment_expression

assignment_expression -> unary_expression assignment_operator assignment_expression
assignment_expression -> conditional_expression // this rule causes deep-dive with loop

unary_expression -> postfix_expression 
unary_expression -> INC_OP unary_expression
unary_expression -> DEC_OP unary_expression
unary_expression -> unary_operator cast_expression
unary_expression -> SIZEOF unary_expression
unary_expression -> SIZEOF ( type_name )

unary_operator -> AMPERSAND
unary_operator -> ASTERISK
unary_operator -> PLUS
unary_operator -> MINUS
unary_operator -> TILDE
unary_operator -> EXCLAMATION_MARK

type_name -> specifier_qualifier_list

specifier_qualifier_list -> type_specifier

cast_expression -> OPENING_BRACES type_name CLOSING_BRACES cast_expression
cast_expression -> unary_expression // prevents construction

postfix_expression -> primary_expression postfix_expression_list
postfix_expression -> primary_expression

postfix_expression_list -> INC_OP postfix_expression_list
postfix_expression_list -> INC_OP

primary_expression -> IDENTIFIER
primary_expression -> HEX_NUMERIC
primary_expression -> NUMERIC

assignment_operator -> EQUALS_SIGN
assignment_operator -> MUL_ASSIGN

// -----------------------------------------------------------

conditional_expression -> logical_or_expression QUESTION_MARK expression COLON conditional_expression
conditional_expression -> logical_or_expression

logical_or_expression -> logical_and_expression OR_OP logical_or_expression
logical_or_expression -> logical_and_expression

logical_and_expression -> inclusive_or_expression AND_OP logical_and_expression
logical_and_expression -> inclusive_or_expression

inclusive_or_expression -> exclusive_or_expression BIN_OR_OP inclusive_or_expression
inclusive_or_expression -> exclusive_or_expression

exclusive_or_expression -> and_expression CIRCUMFLEX exclusive_or_expression
exclusive_or_expression -> and_expression

and_expression -> equality_expression AMPERSAND and_expression
and_expression -> equality_expression

equality_expression -> relational_expression EQ_OP equality_expression
equality_expression -> relational_expression NE_OP equality_expression
equality_expression -> relational_expression

relational_expression -> shift_expression LT relational_expression
relational_expression -> shift_expression GT relational_expression
relational_expression -> shift_expression LTE relational_expression
relational_expression -> shift_expression GTE relational_expression
relational_expression -> shift_expression

shift_expression -> additive_expression LEFT_OP shift_expression
shift_expression -> additive_expression RIGHT_OP shift_expression
shift_expression -> additive_expression

additive_expression -> multiplicative_expression PLUS additive_expression
additive_expression -> multiplicative_expression MINUS additive_expression
additive_expression -> multiplicative_expression

multiplicative_expression -> cast_expression ASTERISK multiplicative_expression
multiplicative_expression -> cast_expression SLASH multiplicative_expression
multiplicative_expression -> cast_expression PERCENT multiplicative_expression
multiplicative_expression -> cast_expression





    println!("end");
}