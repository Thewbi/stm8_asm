use std::{
    collections::{ BTreeMap, HashMap }, fmt, fs::{self, File}, io::{
        BufRead, BufReader, BufWriter, Write,
    }, panic::AssertUnwindSafe, path::Path, result,
};

use crate::{
        PpIfFrame, common::string_handling::rem_first_and_last, example_lexers::preprocessor_lexer::PP_COMMA_TOKEN_ID, lexer::lexer::{Lexer, Token}, parser::rule::RuleElement::Terminal, preprocessor::{
            define_symbol::DefineSymbol,
            expression_parser::ExpressionParser,
            preprocessor_operating_mode::PreprocessorOperatingMode
        }, regex::{
            arena::{Arena, Node, NodeId, VisitMode, recurse_arena_dot},
            enfa::{EpsilonNfa, State},
            regex_building_block::RegexBuildingBlock
        }
};

use crate::example_lexers::preprocessor_lexer::PP_BACKSLASH_TOKEN_ID;
use crate::example_lexers::preprocessor_lexer::PP_IDENTIFIER_TOKEN_ID;
use crate::example_lexers::preprocessor_lexer::PP_NUMERIC_TOKEN_ID;
use crate::example_lexers::preprocessor_lexer::PP_IFDEF_TOKEN_ID;
use crate::example_lexers::preprocessor_lexer::PP_IFNDEF_TOKEN_ID;
use crate::example_lexers::preprocessor_lexer::PP_WHITESPACE_TOKEN_ID;
use crate::example_lexers::preprocessor_lexer::PP_SINGLELINE_COMMENT_START_TOKEN_ID;
use crate::example_lexers::preprocessor_lexer::PP_MULTILINE_COMMENT_START_TOKEN_ID;
use crate::example_lexers::preprocessor_lexer::PP_MULTILINE_COMMENT_END_TOKEN_ID;
use crate::example_lexers::preprocessor_lexer::PP_NEWLINE_TOKEN_ID;
use crate::example_lexers::preprocessor_lexer::PP_DEFINE_TOKEN_ID;
use crate::example_lexers::preprocessor_lexer::PP_UNDEF_TOKEN_ID;
use crate::example_lexers::preprocessor_lexer::PP_IF_TOKEN_ID;
use crate::example_lexers::preprocessor_lexer::PP_ELIF_TOKEN_ID;
use crate::example_lexers::preprocessor_lexer::PP_ELSE_TOKEN_ID;
use crate::example_lexers::preprocessor_lexer::PP_ENDIF_TOKEN_ID;
use crate::example_lexers::preprocessor_lexer::PP_ERROR_TOKEN_ID;
use crate::example_lexers::preprocessor_lexer::PP_INCLUDE_TOKEN_ID;

use crate::example_lexers::preprocessor_lexer::PP_OPENING_BRACKET_TOKEN_ID;
use crate::example_lexers::preprocessor_lexer::PP_CLOSING_BRACKET_TOKEN_ID;
use crate::example_lexers::preprocessor_lexer::PP_CLOSED_BRACKET_PAIR_TOKEN_ID;
use crate::example_lexers::preprocessor_lexer::PP_PRAGMA_TOKEN_ID;
use crate::example_lexers::preprocessor_lexer::PP_ELLIPSIS_TOKEN_ID;

use crate::example_lexers::preprocessor_lexer::PP_PPF_DEFINED_TOKEN_ID;
use crate::example_lexers::preprocessor_lexer::PP_EXCLAMATION_MARK_TOKEN_ID;

static ERROR_ON_DUPLICATE_HEADER_FILES: bool = false;
static USE_PLAIN_TEXT_REPLACE: bool = false;

pub enum PpiIncludeMode {
    // include from local code base
    LOCAL,
    // include from include path passed to the preprocessor
    GLOBAL,
    // explode
    UNKNOWN
}

impl fmt::Display for PpiIncludeMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PpiIncludeMode::GLOBAL => write!(f, "GLOBAL"),
            PpiIncludeMode::LOCAL => write!(f, "LOCAL"),
            PpiIncludeMode::UNKNOWN => write!(f, "UNKNOWN"),
        }
    }
}

pub struct Preprocessor<'a> {
    pub filename: String,
    pub preprocessor_dfa: &'a EpsilonNfa<State, RegexBuildingBlock>,
    pub include_path: &'a Vec<String>,
    pub debug: bool,
}

impl<'a> Preprocessor<'a> {

    pub fn new(preprocessor_dfa_param: &'a EpsilonNfa<State, RegexBuildingBlock>,
        include_path_param: &'a Vec<String>,
        filename_param: String
    ) -> Self {
        Preprocessor {
            preprocessor_dfa: preprocessor_dfa_param,
            filename: filename_param,
            include_path: include_path_param,
            debug: false,
        }
    }

    pub fn process(&self,
        writer: &mut BufWriter<File>,
        defined_symbol_map: &mut HashMap<String, DefineSymbol>)
    {
        // the expression parser is used to parse well-formed text to an AST for evaluation
        // Well-formed text appears in all PPI that need to be evaluated such as #if, #elif
        // Note that the #define PPI only has well-formed text in the macro_interface part
        // but the macro_definition can be free-form text of any kind and does not have to
        // be well-formed
        let mut expression_parser = ExpressionParser::new();
        let mut preprocessor_operating_mode = PreprocessorOperatingMode::Normal;
        let mut buffered_token_option: Option<Token> = None;
        let mut definition_string_buffer: String = String::new();
        let mut ignore_next_newline: bool = false;
        let mut has_definition: bool = false;
        let mut macro_name: String = String::from("UNKNOWN");
        let mut pp_if_frame_stack = Vec::<PpIfFrame>::new();
        let mut ifdef_token: Token = Token::new();
        let mut single_line_comment_mode: bool = false;
        let mut multi_line_comment_mode: bool = false;
        let mut include_file_buffer: String = String::new();
        let mut error_msg_buffer: String = String::new();
        let mut undef_has_been_performed: bool = false;

        // TODO: check if the path is absolute or relative

        let data: String = match fs::read_to_string(self.filename.clone()) {
            Ok(data) => {
                data
            }
            Err(err) => {
                panic!("[ERR][PREPROCESSOR] Cannot load file: '{}'! Error: {}", self.filename.clone(), err);
            }
        };

        // create a lexer which returns the required token for preprocessing
        // the underlying input file into an output file
        let lexer_debug: bool = false;
        let lexer_token_debug: bool = false;
        let mut preprocessor_lexer: Lexer = Lexer::new(data,
            &self.preprocessor_dfa,
            lexer_debug,
            lexer_token_debug);
        preprocessor_lexer.file = self.filename.clone();
        preprocessor_lexer.line = 1;

        let mut lexer_done: bool = false;
        while !lexer_done {

            // retrieve the next token
            // if the lexer has no more token it will return 'None' instead of 'Some'
            let token_option: Option<Token>;
            if buffered_token_option.is_some() {
                token_option = buffered_token_option;
                buffered_token_option = None;
            } else {
                token_option = preprocessor_lexer.next();
            }

            if let Some(token) = token_option {

                let token_text = token.text.clone();

                // DEBUG
                if self.debug {
                    println!("Token consumed: {}", token_text);
                }

                // PRE-PREPROCESSING: Handle single line comments
                //
                // Handle comments before the actual content processing starts
                // to keep all comment handling out of the processing
                //
                if !multi_line_comment_mode {
                    match token.token_id {
                        PP_SINGLELINE_COMMENT_START_TOKEN_ID => {
                            single_line_comment_mode = true;
                            // output a space
                            //write!(writer, "2\n");
                            continue;
                        }
                        PP_NEWLINE_TOKEN_ID => {
                            single_line_comment_mode = false;
                        }
                        _ => {
                            if single_line_comment_mode {
                                continue;
                            }
                        }
                    }
                }

                // PRE-PREPROCESSING: Handle multi line comments
                //
                // Handle comments before the actual content processing starts
                // to keep all comment handling out of the processing
                //
                if !single_line_comment_mode {
                    match token.token_id {
                        PP_MULTILINE_COMMENT_START_TOKEN_ID => {
                            multi_line_comment_mode = true;
                            // // output a space
                            // write!(writer, "2-start\n");
                            continue;
                        }
                        PP_MULTILINE_COMMENT_END_TOKEN_ID => {
                            multi_line_comment_mode = false;
                            // // output a space
                            // write!(writer, "2-end\n");
                            continue;
                        }
                        PP_NEWLINE_TOKEN_ID => {
                            if multi_line_comment_mode {
                                // output a space
                                write!(writer, "\n");
                                continue;
                            }
                        }
                        _ => {
                            if multi_line_comment_mode {
                                continue;
                            }
                        }
                    }
                }

                // PRE-PREPROCESSING: Handle backslash
                //
                // the backslash character is used to extend a single define
                // across several lines. If the backslash is used, the next
                // newline is ignored
                //
                // example:
                // #define _Analysis_mode_(mode) \
                //     typedef _Analysis_mode_impl_(mode) int \
                //         __GENSYM(__prefast_analysis_mode_flag);
                match token.token_id {
                    PP_BACKSLASH_TOKEN_ID => {
                        ignore_next_newline = true;
                        // skip the backslash
                        continue;
                    }
                    PP_NEWLINE_TOKEN_ID => {
                        if ignore_next_newline {
                            ignore_next_newline = false;
                            // skip the newline
                            continue;
                        }
                    }
                    _ => {
                        // nothing
                    }
                }

                // DEBUG
                if self.debug {
                    println!("Current Token: '{}', preprocessor_operating_mode: {}", token, preprocessor_operating_mode);
                }

                match preprocessor_operating_mode {

                    PreprocessorOperatingMode::Normal => {

                        // in normal mode, check the first token for PreProcessor Instructions (PPI)
                        // if there is no PPI, process the following token in NORMAL mode
                        // if there is a PPI, switch to the respective mode
                        match token.token_id {

                            //
                            // #define PPI - PreProcessor Instructions
                            //
                            PP_DEFINE_TOKEN_ID => {
                                // #define (<interface>)? <definition>
                                // if <interface> is present, it needs to be wellformed

                                // reset
                                definition_string_buffer.clear();
                                expression_parser.reset();

                                let mut out_enable: bool = true;

                                // the body of an if-PPI is only enabled if it is enabled and
                                // if all if-PPI below it on the stack are also enabled!
                                for frame in &pp_if_frame_stack {
                                    out_enable = frame.output_enabled;
                                    if !out_enable {
                                        break;
                                    }
                                }

                                if out_enable {
                                    preprocessor_operating_mode = PreprocessorOperatingMode::DefineWaitingForName;
                                }
                            }

                            //
                            // #undef PPI - PreProcessor Instructions
                            //
                            PP_UNDEF_TOKEN_ID => {

                                // reset
                                definition_string_buffer.clear();
                                expression_parser.reset();

                                let mut out_enable: bool = true;

                                // the body of an if-PPI is only enabled if itself is enabled and
                                // if all if-PPI below it on the stack are also enabled!
                                for frame in &pp_if_frame_stack {
                                    out_enable = frame.output_enabled;
                                    if !out_enable {
                                        break;
                                    }
                                }

                                if out_enable {
                                    preprocessor_operating_mode = PreprocessorOperatingMode::Undefine;
                                } else {
                                    // the if-frame is disabled
                                    // do not perform the define but consume and ignore all token
                                    // up to the next newline which returns the state back to
                                    // normal
                                    match token.token_id {
                                        PP_WHITESPACE_TOKEN_ID => {
                                            preprocessor_operating_mode = PreprocessorOperatingMode::Normal;
                                        }
                                        _ => {
                                            // ignore
                                        }
                                    }
                                }
                            }

                            //
                            // DEFINED-checking (#ifdef, #ifndef)
                            //
                            PP_IFDEF_TOKEN_ID => {
                                preprocessor_operating_mode = PreprocessorOperatingMode::Ifdef;
                            }
                            PP_IFNDEF_TOKEN_ID => {
                                preprocessor_operating_mode = PreprocessorOperatingMode::Ifndef;
                            }

                            //
                            // #if PPI - PreProcessor Instructions
                            //
                            PP_IF_TOKEN_ID => {
                                // #if <expression>

                                // reset
                                definition_string_buffer.clear();
                                expression_parser.reset();

                                preprocessor_operating_mode = PreprocessorOperatingMode::If;
                            }
                            PP_ELIF_TOKEN_ID => {
                                // #elif <expression>
                                // panic!("NORMAL > ELIF")
                                //pp_if_frame_stack.pop();

                                preprocessor_operating_mode = PreprocessorOperatingMode::Elif;
                            }
                            PP_ELSE_TOKEN_ID => {
                                // #else
                                //panic!("NORMAL > ELIF")
                                // pp_if_frame_stack.pop();
                                preprocessor_operating_mode = PreprocessorOperatingMode::Else;
                            }
                            PP_ENDIF_TOKEN_ID => {
                                // remove if-frame from the if-stack
                                // remove the if stack frame which had been pushed by a preceding #if or #elif
                                pp_if_frame_stack.pop();

                                preprocessor_operating_mode = PreprocessorOperatingMode::Normal;
                            }

                            //
                            // Pragma
                            //
                            PP_PRAGMA_TOKEN_ID => {
                                preprocessor_operating_mode = PreprocessorOperatingMode::Pragma;
                            }

                            //
                            // Include
                            //
                            PP_INCLUDE_TOKEN_ID => {
                                preprocessor_operating_mode = PreprocessorOperatingMode::Include;
                            }

                            //
                            // Error
                            //
                            PP_ERROR_TOKEN_ID => {
                                let mut out_enabled: bool = true;

                                // the body of an if-PPI is only enabled if it is enabled and
                                // if all if-PPI below it on the stack are also enabled!
                                for frame in &pp_if_frame_stack {
                                    out_enabled = frame.output_enabled;
                                    if !out_enabled {
                                        break;
                                    }
                                }

                                if out_enabled {

                                    preprocessor_operating_mode = PreprocessorOperatingMode::Error;

                                }
                            }

                            //
                            // not PPI, not PPF, Normal token
                            //
                            _ => {
                                // not PPI, not PPF

                                // DEBUG
                                if self.debug {
                                    println!("not PPI, not PPF: '{}'", token.text);
                                }

                                // DEBUG build ASTNODE
                                if self.debug {
                                    // This is for testing the expression parser using normal input
                                    // from a .pp file (pp == preprocessor)
                                    expression_parser.process_token(token.clone());
                                }

                                //
                                // Handle if stack frame
                                //

                                let mut out_enabled: bool = true;

                                // the body of an if-PPI is only enabled if it is enabled and
                                // if all if-PPI below it on the stack are also enabled!
                                for frame in &pp_if_frame_stack {
                                    out_enabled = frame.output_enabled;
                                    if !out_enabled {
                                        break;
                                    }
                                }

                                if out_enabled {

                                    // check if a terminal is in fact a macro
                                    // If it is a macro, the macro is encountered in plain output area.
                                    // Check if token is part of the data store and replace it.
                                    // Output the token to the result.

                                    // DEBUG
                                    if self.debug {
                                        println!("OUT: '{}'", token.text);
                                    }

                                    let mut output_text = String::new();

                                    match token.token_id {
                                        PP_WHITESPACE_TOKEN_ID => {
                                            output_text = token.text;
                                        }
                                        PP_NEWLINE_TOKEN_ID => {
                                            output_text = token.text;
                                        }
                                        _ => {

                                            if USE_PLAIN_TEXT_REPLACE {

                                                output_text = token.text.clone();
                                                if defined_symbol_map.contains_key(&output_text) {
                                                    output_text = self.start_replace_loop(
                                                        &mut preprocessor_lexer, &token, defined_symbol_map, &output_text);
                                                }

                                            } else {

                                                let macro_name_token = token.clone();
                                                output_text = token.text;
                                                if defined_symbol_map.contains_key(&output_text) {

                                                    // println!("plain text token recognized as symbol/macro: {}", output_text);

                                                    let macro_symbol = defined_symbol_map.get(&output_text).unwrap();

                                                    println!("plain text token '{}' recognized as symbol/macro: {:?}", output_text, macro_symbol);

                                                    let mut formal_parameter_count = macro_symbol.formal_parameter_map.len();

                                                    // DEBUG
                                                    if self.debug {
                                                        println!("parameter_count: {}", formal_parameter_count);
                                                    }

                                                    // build a custom define_symbol for the macro/symbol
                                                    let mut define_symbol: DefineSymbol = DefineSymbol::new();
                                                    define_symbol.name = output_text.clone();

                                                    // macro name
                                                    define_symbol.expression_parser.process_token(macro_name_token);

                                                    // println!("balanced: {}", define_symbol.expression_parser.is_tree_balanced());

                                                    loop {

                                                        // let token = preprocessor_lexer.next().unwrap();
                                                        if let Some(token) = preprocessor_lexer.next() {
                                                            if token.text != "\\" {
                                                                define_symbol.expression_parser.process_token(token);

                                                                let balanced = define_symbol.expression_parser.is_tree_balanced();
                                                                // println!("balanced: {}", balanced);

                                                                if balanced {
                                                                    break;
                                                                }
                                                            }
                                                        } else {
                                                            break;
                                                        }
                                                    }

                                                    // DEBUG:
                                                    if self.debug {
                                                        define_symbol.expression_parser.print_dot();
                                                    }

                                                    /*
                                                    let mut consumed_parameter = 0;
                                                    if formal_parameter_count > 0 {

                                                        // opening bracket
                                                        let token_opening_bracket = preprocessor_lexer.next().unwrap();
                                                        define_symbol.expression_parser.process_token(token_opening_bracket);

                                                        // all actual parameters
                                                        while formal_parameter_count > 0 {

                                                            let token = preprocessor_lexer.next().unwrap();

                                                            // DEBUG
                                                            if self.debug {
                                                                println!("{}", token.token_id);
                                                            }

                                                            match token.token_id {

                                                                PP_NUMERIC_TOKEN_ID | PP_IDENTIFIER_TOKEN_ID => {
                                                                    // put next actual parameter into the symbol
                                                                    define_symbol.expression_parser.process_token(token);

                                                                    consumed_parameter = consumed_parameter + 1;

                                                                    formal_parameter_count = formal_parameter_count - 1;
                                                                }

                                                                _ => {
                                                                    // ignore
                                                                    println!("test");
                                                                }
                                                            }
                                                        }

                                                        // closing bracket
                                                        let token_closing_bracket = preprocessor_lexer.next().unwrap();
                                                        define_symbol.expression_parser.process_token(token_closing_bracket);
                                                    }
                                                    */

                                                    // start a loop which keeps replacing symbols until
                                                    // there are no more symbols to replace!
                                                    // TODO: Edge Case: Brake cycles!
                                                    let mut replaced: bool = true;
                                                    while replaced {

                                                        // DEBUG - BEFORE: output AST before replacement
                                                        // if self.debug {
                                                            define_symbol.expression_parser.print_dot();
                                                        // }

                                                        replaced = self.replace_symbols(
                                                            &mut define_symbol.expression_parser.arena,
                                                            &define_symbol.expression_parser.ptr_node_id,
                                                            defined_symbol_map,
                                                            &self.preprocessor_dfa,
                                                            &preprocessor_lexer,
                                                            self.debug);

                                                        // DEBUG - AFTER: output AST after replacement
                                                        // if self.debug {
                                                            //define_symbol.expression_parser.print_dot();

                                                            let root_id = define_symbol.expression_parser.arena.find_root();
                                                            print_arena_dot(&define_symbol.expression_parser.arena, root_id);
                                                        // }
                                                    }

                                                    // DEBUG - AFTER: output AST after replacement
                                                    if self.debug {
                                                        define_symbol.expression_parser.print_dot();
                                                    }

                                                    fn output_prefix(data: &Token) -> String {
                                                        return String::new();
                                                    }
                                                    fn output_infix(data: &Token) -> String {
                                                        match data.token_id {
                                                            PP_CLOSED_BRACKET_PAIR_TOKEN_ID => {
                                                                return String::from("(");
                                                            }
                                                            _ => {
                                                                // if data.text == "#" {
                                                                //     return String::from(" hAsHtAg ");
                                                                // }
                                                                return String::from(&data.text);
                                                            }
                                                        }
                                                    }
                                                    fn output_postfix(data: &Token) -> String {
                                                        match data.token_id {
                                                            PP_CLOSED_BRACKET_PAIR_TOKEN_ID => {
                                                                return String::from(")");
                                                            }
                                                            _ => {
                                                                return String::new();
                                                            }
                                                        }
                                                    }

                                                    let mut string_buffer: String = String::new();
                                                    define_symbol.expression_parser.arena.recurse_arena(
                                                        &output_prefix,
                                                        &output_infix,
                                                        &output_postfix,
                                                        &NodeId { index: define_symbol.expression_parser.arena.find_root() },
                                                        &mut string_buffer
                                                    );

                                                    output_text = string_buffer;
                                                }

                                            }
                                        }
                                    }

                                    // 3. Write data
                                    write!(writer, "{}", output_text);
                                }
                            }
                        }
                    }

                    PreprocessorOperatingMode::DefineWaitingForName => {
                        match token.token_id {
                            PP_WHITESPACE_TOKEN_ID => {
                                // ignore
                            }
                            _ => {
                                macro_name = token.text.clone();
                                expression_parser.process_token(token);
                                preprocessor_operating_mode = PreprocessorOperatingMode::DefineIfcOrDefinition;
                            }
                        }
                    }

                    PreprocessorOperatingMode::DefineIfcOrDefinition => {

                        // the next node is either
                        // 1. a '(' if parameters exist in the macro interface (= in the define)
                        // 2. some token which will become part of the
                        //    macro defintion and in this case no parameters exist
                        // 3. a newline which means the symbol
                        //    is defined to the value 0 (see C-specification on preprocessors)

                        match token.token_id {

                            PP_WHITESPACE_TOKEN_ID => {
                                // ignore
                            }

                            PP_NEWLINE_TOKEN_ID => {
                                // DEBUG
                                if self.debug {
                                    println!("INSERTING EMPTY DEFINE STRUCT INTO DATASTORE!");
                                    // println!("Definition: '{}'", definition_string_buffer);
                                }

                                let mut defined_symbol = DefineSymbol::new();
                                defined_symbol.name = String::from("UNKNOWN");
                                defined_symbol.definition = String::from("0");
                                if let Some(node) = expression_parser.arena.next() {
                                    defined_symbol.name = node.data.text.clone();
                                }

                                // insert the symbol into the symbol map
                                defined_symbol_map.insert(defined_symbol.name.clone(), defined_symbol);

                                // back to NORMAL mode because the define has been consumed
                                preprocessor_operating_mode = PreprocessorOperatingMode::Normal;

                                // reset
                                definition_string_buffer.clear();

                                // back to NORMAL mode
                                preprocessor_operating_mode = PreprocessorOperatingMode::Normal;

                                // output a space
                                write!(writer, "\n");
                            }

                            PP_OPENING_BRACKET_TOKEN_ID => {
                                expression_parser.process_token(token);

                                // enter DEFINE_IFC mode (because there is a parameter list)
                                preprocessor_operating_mode = PreprocessorOperatingMode::DefineIfc;
                            }

                            PP_CLOSING_BRACKET_TOKEN_ID => {
                                panic!("[PREPROCESSOR] Malformed #define found!");
                            }

                            _ => {
                                has_definition = true;

                                // insert word into definition
                                if definition_string_buffer.len() > 0 {
                                    definition_string_buffer.push_str(" ");
                                }
                                definition_string_buffer.push_str(token_text.as_str());

                                // enter DEFINE_DEFINITION mode (because there is no parameter list)
                                preprocessor_operating_mode = PreprocessorOperatingMode::DefineDefinition;
                            }
                        }
                    }

                    PreprocessorOperatingMode::DefineIfc => {

                        match token.token_id {

                            PP_WHITESPACE_TOKEN_ID => {
                                // ignore
                            }
                            PP_NEWLINE_TOKEN_ID => {
                                panic!("[PREPROCESSOR] Malformed #define found!");
                            }
                            PP_CLOSING_BRACKET_TOKEN_ID => {
                                expression_parser.process_token(token);

                                // DEBUG - AFTER
                                if self.debug {
                                    expression_parser.print_dot();
                                }

                                if !expression_parser.is_tree_balanced() {
                                    // nothing
                                } else {
                                    // enter DEFINE_DEFINITION mode (because the parameter list is consumed)
                                    preprocessor_operating_mode = PreprocessorOperatingMode::DefineDefinition;
                                }
                            }
                            _ => {

                                // DEBUG - BEFORE
                                if self.debug {
                                    expression_parser.print_dot();
                                }

                                expression_parser.process_token(token);

                                // DEBUG - AFTER
                                if self.debug {
                                    expression_parser.print_dot();
                                }
                            }
                        }
                    }

                    PreprocessorOperatingMode::DefineDefinition => {

                        // DEBUG
                        //println!("Define-Mode: {:?}, Token: {}", define_mode.to_string(), token);

                        match token.token_id {

                            PP_WHITESPACE_TOKEN_ID => {
                                // ignore
                            }
                            PP_NEWLINE_TOKEN_ID => {
                                // newline encountered. The one line allocated to PPI is over.
                                //
                                // Create define struct and insert into data store

                                // #define <interface> <definition>
                                // leave definition phase, enter NONE phase
                                // define_mode = DefineMode::NONE;
                                // old_define_mode = DefineMode::NONE;

                                // DEBUG
                                if self.debug {
                                    println!("INSERTING DEFINE STRUCT INTO DATASTORE!");
                                    // println!("Definition: '{}'", definition_string_buffer);
                                }

                                // DEBUG
                                // expression_parser.print_dot();
                                // expression_parser.print_console();

                                if !has_definition {

                                    // example
                                    // #define EXPR_A (2 + 3)
                                    // here, there is no interface other than the macro's name
                                    // the definition '(2 + 3)' has been falsely been parsed into
                                    // the interface! The variable has_definition is true, if
                                    // there is a definitino. If there is no definition but there
                                    // is an interace, the interface is moved over to become the
                                    // definition

                                    // first reset the state so that iteration can be performed
                                    // since state is used to remember which nodes in the AST
                                    // have been iterated over already
                                    expression_parser.arena.set_visit_mode_for_all_nodes(expression_parser.ptr_node_id.index, VisitMode::VisitLeft);

                                    // set the start node id from where the iteration in the AST should start
                                    expression_parser.arena.iterator_current_node_id = expression_parser.ptr_node_id.index;

                                    // the left node conatins the macro name which should not be
                                    // become part of the macro defintion, so remove it
                                    expression_parser.arena.remove_left(&expression_parser.ptr_node_id);

                                    // // start from RHS, skip root!
                                    // let right_node = expression_parser.arena.get_right_id(&expression_parser.ptr_node_id).unwrap().clone();
                                    // let right_node_id = right_node.index;
                                    // expression_parser.arena.iterator_current_node_id = right_node_id;

                                    // // iterate over all nodes
                                    // let mut expr_done: bool = false;
                                    // while !expr_done {
                                    //     if let Some(node) = expression_parser.arena.next() {
                                    //         definition.push_str(node.data.text.clone().as_str());
                                    //     } else {
                                    //         expr_done = true;
                                    //     }
                                    // }

                                    // convert the data that has been the interface into
                                    // a string and assign that string to the definition
                                    let mut string_buffer: String = String::new();

                                    self.to_string_1(&expression_parser.arena,
                                        &expression_parser.ptr_node_id,
                                        &mut string_buffer
                                    );

                                    if self.debug {
                                        println!("{}", string_buffer);
                                    }

                                    let mut defined_symbol = DefineSymbol::new();
                                    defined_symbol.name = macro_name.clone();
                                    defined_symbol.formal_parameter_map.clear(); // no interface
                                    defined_symbol.definition = string_buffer;

                                    let mut out_enable: bool = true;

                                    // the body of an if-PPI is only enabled if it is enabled and
                                    // if all if-PPI below it on the stack are also enabled!
                                    for frame in &pp_if_frame_stack {
                                        out_enable = frame.output_enabled;
                                        if !out_enable {
                                            break;
                                        }
                                    }

                                    if out_enable {
                                        // insert the symbol into the symbol map
                                        defined_symbol_map.insert(defined_symbol.name.clone(), defined_symbol);
                                    }

                                } else {

                                    //
                                    // The AST parsed for the macro's interface is now
                                    // traversed. The traversal will extract the macro's
                                    // name and all parameters
                                    //

                                    // first reset the state so that iteration can be performed
                                    // since state is used to remember which nodes in the AST
                                    // have been iterated over already
                                    expression_parser.arena.set_visit_mode_for_all_nodes(expression_parser.ptr_node_id.index, VisitMode::VisitLeft);

                                    // set the start node id from where the iteration in the AST should start
                                    expression_parser.arena.iterator_current_node_id = expression_parser.ptr_node_id.index;

                                    let mut defined_symbol = DefineSymbol::new();
                                    defined_symbol.name = String::from("UNKNOWN");
                                    defined_symbol.definition = definition_string_buffer.to_owned();

                                    let mut terminal_index:i32 = -1;

                                    // iterate over all nodes. Nodes are returned in in-order
                                    // Each node has optional left and right children.
                                    // In-order means the node itself is output after visiting
                                    // the left and before visiting the right child.
                                    // In this order, outputting the AST yields the original String
                                    // the AST was parsed from.
                                    let mut expr_done: bool = false;
                                    while !expr_done {

                                        if let Some(node) = expression_parser.arena.next() {

                                            // filter away all token which are not terminals
                                            match node.data.token_id {

                                                PP_IDENTIFIER_TOKEN_ID => {
                                                    // DEBUG
                                                    if self.debug {
                                                        println!("{:?}", node);
                                                    }

                                                    // first terminal is the name of the interface
                                                    // all subsequent terminals are parameters
                                                    if terminal_index == -1 {
                                                        defined_symbol.name = node.data.text.clone();
                                                        terminal_index = terminal_index + 1;
                                                    } else {
                                                        defined_symbol.formal_parameter_map.insert(terminal_index as usize, node.data.text.clone());
                                                        terminal_index = terminal_index + 1;
                                                    }
                                                }

                                                PP_ELLIPSIS_TOKEN_ID => {
                                                    // first terminal is the name of the interface
                                                    // all subsequent terminals are parameters
                                                    if terminal_index == -1 {
                                                        defined_symbol.name = node.data.text.clone();
                                                        terminal_index = terminal_index + 1;
                                                    } else {
                                                        defined_symbol.formal_parameter_map.insert(terminal_index as usize, node.data.text.clone());
                                                        terminal_index = terminal_index + 1;
                                                    }
                                                }

                                                _ => {
                                                }
                                            }
                                        } else {
                                            expr_done = true;
                                        }
                                    }

                                    let mut out_enable: bool = true;

                                    // the body of an if-PPI is only enabled if it is enabled and
                                    // if all if-PPI below it on the stack are also enabled!
                                    for frame in &pp_if_frame_stack {
                                        out_enable = frame.output_enabled;
                                        if !out_enable {
                                            break;
                                        }
                                    }

                                    if out_enable {
                                        // insert the symbol into the symbol map
                                        defined_symbol_map.insert(defined_symbol.name.clone(), defined_symbol);
                                    }
                                }

                                // back to NORMAL mode because the #define has been consumed
                                preprocessor_operating_mode = PreprocessorOperatingMode::Normal;

                                // output a space
                                write!(writer, "\n");

                                // reset
                                expression_parser.reset();
                                definition_string_buffer.clear();
                                macro_name = String::from("");
                                has_definition = false;

                            }
                            _ => {

                                // something other than space has been inserted into the definition
                                // so there is a definition
                                has_definition = true;

                                // insert word into definition
                                if definition_string_buffer.len() > 0 {
                                    definition_string_buffer.push_str(" ");
                                }
                                definition_string_buffer.push_str(token_text.as_str());
                            }
                        }
                    }

                    PreprocessorOperatingMode::Undefine => {

                        // DEBUG
                        if self.debug {
                            println!("{}", token.text);
                        }

                        match token.token_id {

                            PP_NEWLINE_TOKEN_ID => {
                                if undef_has_been_performed {
                                    preprocessor_operating_mode = PreprocessorOperatingMode::Normal;
                                    undef_has_been_performed = false;
                                } else {
                                    panic!("[ERR-PREPROCESSOR] Cannot process #undef! File: {}, Line: {}", preprocessor_lexer.file, preprocessor_lexer.line-1);
                                }
                            }
                            PP_WHITESPACE_TOKEN_ID => {

                            }
                            _ => {
                                if defined_symbol_map.contains_key(&token.text) {
                                    defined_symbol_map.remove(&token.text);
                                }
                                undef_has_been_performed = true;
                            }
                        }
                    }

                    PreprocessorOperatingMode::If | PreprocessorOperatingMode::Elif => {

                        match token.token_id {

                            PP_WHITESPACE_TOKEN_ID => {
                                // ignore
                            }
                            PP_EXCLAMATION_MARK_TOKEN_ID => {
                                // DEBUG - BEFORE
                                if self.debug {
                                    expression_parser.print_dot();
                                }

                                expression_parser.process_token(token);

                                // DEBUG - AFTER
                                if self.debug {
                                    expression_parser.print_dot();
                                }
                            }
                            PP_PPF_DEFINED_TOKEN_ID => {
                                expression_parser.process_token(token);
                            }
                            PP_NEWLINE_TOKEN_ID => {
                                let mut if_statement_already_enabled: bool = false;

                                if preprocessor_operating_mode == PreprocessorOperatingMode::Elif {

                                    // remove the if stack frame which had been pushed by a
                                    // preceding #if or #elif
                                    let frame = pp_if_frame_stack.pop().unwrap();
                                    if frame.if_statement_already_enabled {

                                        // a prior branch was enabled.
                                        // This means that this #elif-branch should be disabled because
                                        // only a single branch can be enabled per if
                                        if_statement_already_enabled = true;
                                    }
                                }

                                // DEBUG - before
                                if self.debug {
                                    expression_parser.print_dot();
                                    // expression_parser.print_console();
                                }

                                // DEBUG
                                if self.debug {
                                    println!("RootNode: {}", expression_parser.ptr_node_id.index);
                                }

                                if if_statement_already_enabled {

                                    // add a new if frame which is disabled since the entire
                                    // if-statement is already covered
                                    let mut frame: PpIfFrame = PpIfFrame::new();
                                    frame.output_enabled = false;
                                    frame.if_statement_already_enabled = true;
                                    pp_if_frame_stack.push(frame);

                                } else {

                                    // start a loop which keeps replacing symbols until
                                    // there are no more symbols to replace!
                                    // TODO Edge Case: Brake cycles!
                                    let mut replaced:bool = true;
                                    while replaced {

                                        // DEBUG - BEFORE: output AST before replacement
                                        if self.debug {
                                            expression_parser.print_dot();
                                        }

                                        replaced = self.replace_symbols(&mut expression_parser.arena,
                                            &expression_parser.ptr_node_id,
                                            defined_symbol_map,
                                            &self.preprocessor_dfa,
                                            &preprocessor_lexer,
                                            self.debug);

                                        // DEBUG - AFTER: output AST after replacement
                                        if self.debug {
                                            expression_parser.print_dot();
                                        }
                                    }

                                    // DEBUG - AFTER: output AST after replacement
                                    if self.debug {
                                        expression_parser.print_dot();
                                    }

                                    // evaluate the expression
                                    let eval_value = self.evaluate_expression(&expression_parser.arena,
                                        &expression_parser.ptr_node_id,
                                        &defined_symbol_map,
                                        self.debug);

                                    // DEBUG
                                    if self.debug {
                                        println!("eval_value: {}", eval_value);
                                    }

                                    // add a new if frame!
                                    let mut frame: PpIfFrame = PpIfFrame::new();
                                    frame.output_enabled = false;
                                    frame.if_statement_already_enabled = false;
                                    if eval_value == 1 {
                                        frame.output_enabled = true;
                                        frame.if_statement_already_enabled = true;
                                    }
                                    pp_if_frame_stack.push(frame);
                                }

                                expression_parser.reset();

                                // back to NORMAL mode because the #if has been consumed
                                preprocessor_operating_mode = PreprocessorOperatingMode::Normal;

                                // output a space
                                write!(writer, "\n");
                            }
                            _ => {
                                expression_parser.process_token(token);
                            }
                        }
                    }

                    PreprocessorOperatingMode::Else => {
                        // remove the if stack frame which had been pushed by a preceding #if or #elif
                        let mut frame = pp_if_frame_stack.pop().unwrap();
                        // the else branch unconditionally outputs it's contents
                        // unless a prior branch of the if has been activated
                        frame.output_enabled = true;
                        if frame.if_statement_already_enabled {
                            frame.output_enabled = false;
                        }
                        // push frame back
                        pp_if_frame_stack.push(frame);
                        // back to NORMAL mode because the #if has been consumed
                        preprocessor_operating_mode = PreprocessorOperatingMode::Normal;
                    }

                    PreprocessorOperatingMode::Endif => {
                        // remove the if stack frame which had been pushed by a preceding #if or #elif
                        pp_if_frame_stack.pop();
                    }

                    PreprocessorOperatingMode::Pragma => {
                        match token.token_id {
                            PP_WHITESPACE_TOKEN_ID => {
                                // ignore
                            }
                            PP_NEWLINE_TOKEN_ID => {
                                // back to NORMAL mode because the #if has been consumed
                                preprocessor_operating_mode = PreprocessorOperatingMode::Normal;
                            }
                            _ => {
                                // ignoring pragmas for now
                            }
                        }
                    }

                    PreprocessorOperatingMode::Ifdef => {
                        match token.token_id {
                            PP_WHITESPACE_TOKEN_ID => {
                                // ignore
                            }
                            PP_NEWLINE_TOKEN_ID => {

                                let mut eval_value = 0;

                                // check if the symbol is defined
                                if defined_symbol_map.contains_key(&ifdef_token.text) {
                                    eval_value = 1;
                                }

                                // add a new if frame!
                                let mut frame: PpIfFrame = PpIfFrame::new();
                                frame.output_enabled = false;
                                frame.if_statement_already_enabled = false;
                                if eval_value == 1 {
                                    frame.output_enabled = true;
                                    frame.if_statement_already_enabled = true;
                                }
                                pp_if_frame_stack.push(frame);

                                // back to NORMAL mode because the #if has been consumed
                                preprocessor_operating_mode = PreprocessorOperatingMode::Normal;

                                // output a space
                                write!(writer, "\n");
                            }
                            _ => {
                                ifdef_token = token;
                            }
                        }
                    }

                    PreprocessorOperatingMode::Ifndef => {
                        match token.token_id {
                            PP_WHITESPACE_TOKEN_ID => {
                                // ignore
                            }
                            PP_NEWLINE_TOKEN_ID => {

                                let mut eval_value = 0;

                                // check if the symbol is NOT defined
                                if defined_symbol_map.contains_key(&ifdef_token.text) {
                                    eval_value = 0;
                                } else {
                                    eval_value = 1;
                                }

                                // add a new if frame!
                                let mut frame: PpIfFrame = PpIfFrame::new();
                                frame.output_enabled = false;
                                frame.if_statement_already_enabled = false;
                                if eval_value == 1 {
                                    frame.output_enabled = true;
                                    frame.if_statement_already_enabled = true;
                                }
                                pp_if_frame_stack.push(frame);

                                // back to NORMAL mode because the #if has been consumed
                                preprocessor_operating_mode = PreprocessorOperatingMode::Normal;
                            }
                            _ => {
                                //expression_parser.process_token(token);
                                ifdef_token = token;
                            }
                        }
                    }

                    PreprocessorOperatingMode::Include => {
                        match token.token_id {

                            // execut the #include PPI
                            PP_NEWLINE_TOKEN_ID => {

                                // DEBUG
                                if self.debug {
                                    println!("including file: {}", include_file_buffer);
                                }

                                // when it comes to resolving relative paths to include files,
                                // there is a difference between
                                // #include "something.h"
                                // and
                                // #include <something.h>
                                //
                                // The quoted version causes the preprocessor to look for
                                // the header file in the local directory. This means in order
                                // to turn the relative path into an absolute path, the
                                // preprocessor prepends the current directory to the relative path
                                // to create the absolute path. The current directory is PWD, the
                                // directory in which the executable has been executed.
                                //
                                // The angle-bracketed version causes the preprocessor to turn
                                // relative into absolute paths by refering to the include path.
                                // The include path is a list of absolute paths. The preprocessor
                                // iterates this list and uses each path to produce an absolute
                                // path from the relative path. It tries to load the include file
                                // from that absolute path.
                                // Using IncludePath is referred to as GLOBAL includes!
                                let mut ppi_include_mode: PpiIncludeMode = PpiIncludeMode::UNKNOWN;
                                if include_file_buffer.starts_with("<") {
                                    // use include path
                                    ppi_include_mode = PpiIncludeMode::GLOBAL;
                                } else {
                                    // include from local code base
                                    ppi_include_mode = PpiIncludeMode::LOCAL;
                                }

                                let trimmed = rem_first_and_last(&include_file_buffer);

                                // ignore some header files!
                                if trimmed != "CodeAnalysis/sourceannotations.h" {

                                    let mut absolute_path = String::new();
                                    let mut final_include_filename = String::new();

                                    // turn relative paths into absolute paths
                                    match ppi_include_mode {

                                        PpiIncludeMode::GLOBAL => {

                                            //
                                            // include using the include path
                                            //

                                            let mut file_found_counter: usize = 0;

                                            // resolve the file for each path in the include path.
                                            // If the file is found in two or more directories, explode
                                            // in order to safe the user from include-hell!
                                            for include_path_element in self.include_path {

                                                absolute_path = include_path_element.clone();

                                                absolute_path.push_str("\\");
                                                absolute_path.push_str(trimmed);

                                                println!("{:?}", &absolute_path);

                                                let path = Path::new(absolute_path.as_str());
                                                if path.exists() {
                                                    if file_found_counter > 0 {

                                                        println!("");
                                                        println!("Found here - {}", final_include_filename);
                                                        println!("Found here - {}", absolute_path);

                                                        if ERROR_ON_DUPLICATE_HEADER_FILES {
                                                            panic!("Include file '{}' is found in more than one folder in the include path!", trimmed);
                                                        }
                                                    }
                                                    file_found_counter = file_found_counter + 1;

                                                    final_include_filename = absolute_path;
                                                }
                                            }

                                            if file_found_counter == 0 {
                                                panic!("Cannot find file '{}' in include path!", trimmed)
                                            }
                                        }
                                        PpiIncludeMode::LOCAL => {

                                            //
                                            // include relative to the local code base
                                            //

                                            todo!();
                                        }
                                        _=> {
                                            panic!("Cannot process include mode: {}", ppi_include_mode);
                                        }
                                    }

                                    let preprocessor: Preprocessor = Preprocessor::new(
                                        &self.preprocessor_dfa,
                                        &self.include_path,
                                        String::from(final_include_filename));
                                    preprocessor.process(writer, defined_symbol_map);
                                }

                                include_file_buffer.clear();

                                // back to NORMAL mode because the #include has been consumed
                                preprocessor_operating_mode = PreprocessorOperatingMode::Normal;
                            }
                            _ => {
                                // println!("Token: {}", token.text);
                                include_file_buffer.push_str(token.text.clone().trim());
                            }
                        }
                    }

                    PreprocessorOperatingMode::Error => {

                        match token.token_id {
                            // execut the #include PPI
                            PP_NEWLINE_TOKEN_ID => {
                                panic!("[PREPROCESSOR::error]: '{}'", error_msg_buffer.as_str());
                            }
                            _ => {
                                error_msg_buffer.push_str(token.text.as_str());
                            }
                        }
                    }
                }

            } else {
                lexer_done = true;
            }
        }
    }

    pub fn start_replace_loop(&self,
        preprocessor_lexer: &mut Lexer,
        initial_token: &Token,
        defined_symbol_map: &mut HashMap<String, DefineSymbol>,
        data: &String) -> String
    {
        let mut result_string_buffer: String = data.clone();
        let mut first_iteration: bool = true;
        let mut before_iteration_text: String = String::new();

        let mut iteration: usize = 0;

        let mut cycle_detection_set: Vec<String> = Vec::new();

        loop {

            println!("Iteration: {}, Input: '{}'", iteration, result_string_buffer);

            iteration = iteration + 1;

            let lexer_debug: bool = false;
            let lexer_token_debug: bool = false;
            let mut new_preprocessor_lexer: Lexer = Lexer::new(result_string_buffer.clone(),
                    &self.preprocessor_dfa,
                    lexer_debug,
                    lexer_token_debug);

            let mut output_text_before = String::new();

            if first_iteration {
                // nothing
            } else {
                new_preprocessor_lexer = Lexer::new(result_string_buffer.clone(),
                    &self.preprocessor_dfa,
                    lexer_debug,
                    lexer_token_debug);
                new_preprocessor_lexer.file = self.filename.clone();
                new_preprocessor_lexer.line = 1;
            }

            let mut lexer_done: bool = false;
            while !lexer_done {

                let mut token_option: Option<Token> = None;

                if first_iteration {
                    token_option = Some(initial_token.clone());
                    before_iteration_text = initial_token.text.clone();
                    output_text_before = defined_symbol_map.get(&initial_token.text.clone()).unwrap().definition.clone();
                } else {
                    token_option = new_preprocessor_lexer.next();
                }

                if let Some(token) = token_option {

                    // 1. check if the token is a macro/symbol and start next iteration if not
                    if !defined_symbol_map.contains_key(&token.text) {
                        continue;
                    }

                    // DEBUG
                    if self.debug {
                        println!("plain text token recognized as symbol/macro: {}", &token.text);
                    }

                    if cycle_detection_set.contains(&token.text) {
                        println!("[ERR][PREPROCESSOR] Cycle Detected while replacing '{}'", &token.text);
                        println!("[ERR][PREPROCESSOR] List of visited token is: {:?}", &cycle_detection_set);
                        panic!("[ERR][PREPROCESSOR] Cycle Detected! Already replaced token is: {}", &token.text);
                    }

                    cycle_detection_set.push(token.text.clone());

                    // 2. retrieve macro/symbol definition
                    let macro_symbol = defined_symbol_map.get(&token.text).unwrap();

                    // DEBUG
                    if self.debug {
                        println!("plain text token '{}' recognized as symbol/macro: {:?}", token.text, macro_symbol);
                    }

                    //
                    // parse the macro call into an AST
                    //

                    let formal_parameter_count = macro_symbol.formal_parameter_map.len();

                    // DEBUG
                    if self.debug {
                        println!("parameter_count: {}", formal_parameter_count);
                    }

                    // create new instance because the defintion will be manipulated
                    // by replacing formal with actual parameters and the original
                    // definition in the defined_symbol_map is supposed to remain unchanged
                    let mut define_symbol: DefineSymbol = DefineSymbol::new();
                    define_symbol.name = token.text.clone();
                    define_symbol.definition = macro_symbol.definition.clone();
                    define_symbol.formal_parameter_map = macro_symbol.formal_parameter_map.clone();

                    println!("++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++");

                    // if there are formal parameters, let the lexer consume them.
                    // Each parameter might be a complex subtree itself!
                    if formal_parameter_count > 0 {

                        let mut token = Token::new();
                        if first_iteration {
                            token = preprocessor_lexer.next().unwrap();
                            // println!("1: '{}'", token.text);
                            define_symbol.expression_parser.process_token(token);

                            // DEBUG - print dot of macro/symbol
                            // let root_id = define_symbol.expression_parser.arena.find_root();
                            // self.print_arena_dot(&define_symbol.expression_parser.arena, root_id);
                            while !define_symbol.expression_parser.is_tree_balanced() {
                                let mut token = Token::new();
                                token = preprocessor_lexer.next().unwrap();
                                println!("2: '{}'", token.text);
                                define_symbol.expression_parser.process_token(token);

                                // DEBUG - print dot of macro/symbol
                                // let root_id = define_symbol.expression_parser.arena.find_root();
                                // self.print_arena_dot(&define_symbol.expression_parser.arena, root_id);
                            }
                        } else {
                            // token = new_preprocessor_lexer.next().unwrap();
                            // for token in new_preprocessor_lexer {
                            //     define_symbol.expression_parser.process_token(token);
                            // }

                            loop {
                                if let Some(token) = new_preprocessor_lexer.next() {
                                    define_symbol.expression_parser.process_token(token);
                                } else {
                                    break;
                                }
                            }
                        }

                    }

                    // DEBUG - print dot of macro/symbol
                    // let root_id = define_symbol.expression_parser.arena.find_root();
                    // self.print_arena_dot(&define_symbol.expression_parser.arena, root_id);

                    //
                    // build actual parameter map
                    //

                    let formal_parameter_count = macro_symbol.formal_parameter_map.len();

                    // DEBUG
                    if self.debug {
                        println!("parameter_count: {}", formal_parameter_count);
                    }

                    // DEBUG
                    // for token in define_symbol.expression_parser.arena.clone() {
                    //     println!("{}", token.data);
                    // }

                    let mut replace_map = HashMap::<String, String>::new();

                    if formal_parameter_count > 0 {
                        self.fill_parameter_map(&mut define_symbol, &mut replace_map);
                        // // DEBUG - output formal parameters
                        // for (key, value) in replace_map.into_iter() {
                        //     println!("{} / {}", key, value);
                        // }
                    }

                    // DEBUG - output definition
                    if self.debug {
                        println!("define_symbol.definition: {}", define_symbol.definition);
                    }

                    let mut output_text = define_symbol.definition.clone();
                    for (key, value) in replace_map.into_iter() {
                        output_text = output_text.replace(key.as_str(), value.as_str());
                    }

                    if output_text != before_iteration_text {
                        before_iteration_text = output_text_before.clone();
                        result_string_buffer = result_string_buffer.replace(token.text.as_str(), output_text.as_str());
                        first_iteration = false;
                        lexer_done = true;
                        continue;
                    } else {
                        return output_text;
                    }

                } else {
                    lexer_done = true;
                    return result_string_buffer;
                }
            }

            first_iteration = false;
        }
    }

    pub fn fill_parameter_map(&self,
        define_symbol: &DefineSymbol,
        replace_map: &mut HashMap::<String, String>)
    {
        let current_parameter_index: usize = 0;

        let arena = &define_symbol.expression_parser.arena;

        // retrieve RHS of root
        let root_id = arena.find_root();
        if let Some(right_node_id) = arena.get_right_id(&NodeId { index:root_id }) {

            let rhs_node = &arena.nodes[right_node_id.index];
            if rhs_node.data.text == "," {
                self.fill_parameter_map_comma(define_symbol, replace_map, current_parameter_index, right_node_id);
            } else {
                let formal_parameter_name = define_symbol.formal_parameter_map.get(&current_parameter_index);
                replace_map.insert(formal_parameter_name.unwrap().clone(), rhs_node.data.text.clone());
            }
        }
    }

    pub fn fill_parameter_map_comma(&self,
        define_symbol: &DefineSymbol,
        replace_map: &mut HashMap::<String, String>,
        current_parameter_index: usize,
        node_id: &NodeId) -> usize
    {
        let arena = &define_symbol.expression_parser.arena;
        let current_node = &arena.nodes[node_id.index];

        let mut new_parameter_index = current_parameter_index;

        if current_node.data.text == "," {

            if let Some(lhs_node) = arena.get_left_id(node_id) {
                new_parameter_index = self.fill_parameter_map_comma(define_symbol, replace_map, new_parameter_index, lhs_node);
                new_parameter_index = new_parameter_index + 1;
            }

            if let Some(rhs_node) = arena.get_right_id(node_id) {
                new_parameter_index = self.fill_parameter_map_comma(define_symbol, replace_map, new_parameter_index, rhs_node);
            }

        } else {

            fn output_prefix(data: &Token) -> String {
                return String::new();
            }
            fn output_infix(data: &Token) -> String {
                match data.token_id {
                    PP_CLOSED_BRACKET_PAIR_TOKEN_ID => {
                        return String::from("(");
                    }
                    _ => {
                        return String::from(&data.text);
                    }
                }
            }
            fn output_postfix(data: &Token) -> String {
                match data.token_id {
                    PP_CLOSED_BRACKET_PAIR_TOKEN_ID => {
                        return String::from(")");
                    }
                    _ => {
                        return String::new();
                    }
                }
            }

            let root_id_index = define_symbol.expression_parser.arena.find_root();
            let root_id = NodeId { index: root_id_index };
            let mut string_buffer: String = String::new();
            define_symbol.expression_parser.arena.recurse_arena(
                &output_prefix,
                &output_infix,
                &output_postfix,
                node_id,
                &mut string_buffer
            );

            let formal_parameter_name = define_symbol.formal_parameter_map.get(&current_parameter_index);
            replace_map.insert(formal_parameter_name.unwrap().clone(), string_buffer);

            //println!("{}:{} -> {}", formal_parameter_name.unwrap().clone(), current_parameter_index, string_buffer);
        }

        new_parameter_index
    }

    pub fn to_string_1(&self,
        arena: &Arena<Token>,
        node_id: &NodeId,
        string_buffer: &mut String)
    {
        let node: &Node<Token> = &arena.nodes[node_id.index];

        let mut has_left_child:bool = false;
        let mut has_right_child:bool = false;

        let mut left_child_id:usize = 0;
        let mut right_child_id:usize = 0;

        // LHS - output left child
        if let Some(left_id) = &node.left {
            self.to_string_1(arena, left_id, string_buffer);
            has_left_child = true;
            left_child_id = left_id.index;
        }

        if string_buffer.len() > 0 {
            string_buffer.push_str(" ");
        }

        if node.data.text == "()" {
            string_buffer.push_str("(");
        } else {
            string_buffer.push_str(node.data.text.clone().as_str());
        }

        // RHS - output right child
        if let Some(right_id) = &node.right {
            self.to_string_1(arena, right_id, string_buffer);
            has_right_child = true;
            right_child_id = right_id.index;
        }

        if node.data.text == "()" {
            string_buffer.push_str(" )");
        }
    }

    fn node_to_text(&self,
        arena: &Arena<Token>, node: &Node<Token>, node_id: &NodeId, string_buffer: &mut String) {

        // // LHS - replace symbols inside left child
        // if let Some(left_id) = &node.left {
        //     has_left_child = true;
        //     left_result = self.replace_symbols(arena,
        //         left_id, defined_symbol_map, preprocessor_dfa, preprocessor_lexer, debug);
        //     left_child_id = left_id.index;
        // }

        // // RHS - replace symbols inside right child
        // if let Some(right_id) = &node.right {
        //     has_right_child = true;
        //     right_result = self.replace_symbols(arena,
        //         right_id, defined_symbol_map, preprocessor_dfa, preprocessor_lexer, debug);
        //     right_child_id = right_id.index;
        // }

        fn output_prefix(data: &Token) -> String {
            return String::new();
        }
        fn output_infix(data: &Token) -> String {
            match data.token_id {
                PP_CLOSED_BRACKET_PAIR_TOKEN_ID => {
                    return String::from("(");
                }
                _ => {
                    return String::from(&data.text);
                }
            }
            // data.terminal.to_string()
        }
        fn output_postfix(data: &Token) -> String {
            match data.token_id {
                PP_CLOSED_BRACKET_PAIR_TOKEN_ID => {
                    return String::from(")");
                }
                _ => {
                    return String::new();
                }
            }
        }

        // let mut string_buffer: String = String::new();
        arena.recurse_arena(
            &output_prefix,
            &output_infix,
            &output_postfix,
            &node_id,
            string_buffer
        );
    }

    fn extract_actual_parameters(&self,
        arena: &Arena<Token>, node: &Node<Token>, node_id: &NodeId,
        replace_map: &mut HashMap::<String, String>,
        formal_parameter_map: &BTreeMap::<usize, String>,
        actual_parameter_index: &mut usize)
    {
        // println!("node.data.text: {}", node.data.text);

        if node.data.text == "," {

            let mut has_left_child:bool = false;
            let mut has_right_child:bool = false;

            let mut left_result:bool = false;
            let mut right_result:bool = false;

            let mut left_child_id:usize = 0;
            let mut right_child_id:usize = 0;

            // LHS - replace symbols inside left child
            if let Some(left_node_id) = &node.left {
                has_left_child = true;

                left_child_id = left_node_id.index;
                let left_node = &arena.nodes[left_child_id];

                self.extract_actual_parameters(
                    arena,
                    &left_node,
                    left_node_id,
                    replace_map,
                    formal_parameter_map,
                    actual_parameter_index
                );
            }

            // RHS - replace symbols inside right child
            if let Some(right_node_id) = &node.right {
                has_right_child = true;

                right_child_id = right_node_id.index;
                let right_node = &arena.nodes[right_child_id];

                self.extract_actual_parameters(
                    arena,
                    &right_node,
                    right_node_id,
                    replace_map,
                    formal_parameter_map,
                    actual_parameter_index
                );
            }
        } else {
            let mut value: String = String::new();
            self.node_to_text(&arena, &node, &node_id, &mut value);
            println!("Actual parameter found: '{}'", value);

            let key = formal_parameter_map.get(actual_parameter_index).unwrap();

            // DEBUG
            println!("[Param-Replace-Map: Index: {}] key: '{}', value: '{}'",
                actual_parameter_index, key, value);

            // insert formal parameter as key and
            // actual parameter-value as value into the
            // parameter replace map
            replace_map.insert(key.clone(), value);

            *actual_parameter_index = *actual_parameter_index + 1usize;
        }
    }

    /// Replaces macro names by their macro definition resolving formal
    /// parameters by actual parameters.
    ///
    /// NOTE: One exception is implemented: Macro names inside the
    /// PPF defined are not replaced by the definition. The reason
    /// is that during evaluation, the PPF defined needs to check if
    /// the Macro is defined or not.
    pub fn replace_symbols(&self,
        // the arena contains the macro name and it contains the actual parameters!
        // This means for a macro #define SQUARE(x) it has to contain the actual
        // parameter passed into x. If SQUARE(4) is called, the arena needs to contain
        // an AST which represents () - 4
        arena: &mut Arena<Token>,
        node_id: &NodeId,
        defined_symbol_map: &mut HashMap::<String, DefineSymbol>,
        preprocessor_dfa: &EpsilonNfa<State, RegexBuildingBlock>,
        preprocessor_lexer: &Lexer,
        debug: bool) -> bool
    {
        // DEBUG
        // if debug {
            println!("replace!");
        // }

        let mut has_left_child:bool = false;
        let mut has_right_child:bool = false;

        let mut left_result:bool = false;
        let mut right_result:bool = false;

        let mut left_child_id:usize = 0;
        let mut right_child_id:usize = 0;

        // if there is no AST to replace symbols in, early out
        if arena.nodes.len() == 0 {
            return false;
        }

        // // DEBUG
        // if debug {
        //     self.print_arena_dot(&arena);
        // }

        let node: &Node<Token> = &arena.nodes[node_id.index].clone();

        // retrieve the token which is stored inside the current node
        let token: &Token = &node.data;

        // #defined can only be evaluated if the contained macro remains
        // as it is without replacing it.
        // Therefore when #defined PPF is executed, exit early without
        // replacing #defined.
        match &token.terminal {

            Terminal(val) => {

                // DEBUG
                // println!("val: {}", val);

                // #defined can only be evaluated if we leave the macro
                // as it is without replacing it. Therefore abort here
                // for the #defined PPF
                if *val == String::from("PPF_DEFINED") {
                    return false;
                }
            }
            _ => {

            }
        }

        // LHS - replace symbols inside left child
        if let Some(left_id) = &node.left {
            has_left_child = true;
            left_result = self.replace_symbols(arena,
                left_id, defined_symbol_map, preprocessor_dfa, preprocessor_lexer, debug);
            left_child_id = left_id.index;
        }

        // RHS - replace symbols inside right child
        if let Some(right_id) = &node.right {
            has_right_child = true;
            right_result = self.replace_symbols(arena,
                right_id, defined_symbol_map, preprocessor_dfa, preprocessor_lexer, debug);
            right_child_id = right_id.index;
        }

        // retrieve the token which is stored inside the current node
        let token: &Token = &node.data;

        // DEBUG
        if debug {
            println!("token: {:?}", token);
        }

        // check if this literal is not a text but a preprocessor defined macro/symbol
        // If it is a macro/symbol, replace it
        match &token.terminal {

            Terminal(val) => {

                // DEBUG
                if debug {
                    println!("val: {}", val);
                }

                if *val == String::from("IDENTIFIER") {

                    // DEBUG
                    if debug {
                        println!("token.text: {}", token.text);
                    }

                    // check if this string of text is a macro name
                    if defined_symbol_map.contains_key(&token.text) {

                        let defined_symbol_option = defined_symbol_map.get_mut(&token.text);
                        if let Some(define_symbol) = defined_symbol_option {

                            //
                            // PHASE 0
                            //
                            // parse the definition into an AST, or reuse an AST optimally
                            // so that later actual parameters can be inserted into the
                            // AST and further symbols can be replaced.
                            //

                            let file = preprocessor_lexer.file.clone();
                            let line = preprocessor_lexer.line;
                            self.parse_definition_into_AST(define_symbol,
                                preprocessor_dfa, &file, line, debug);

                            //
                            // replace the formal parameter occurences by actual parameters
                            //
                            // define_symbol is the formal symbol_definition with formal parameters.
                            // In it's formal_parameter_map contains the amount of formal parameters.
                            //
                            // the actual parameters need to be retrieved by ascending to the parent
                            // and then iterating the RHS subtree which contains the list of actual
                            // parameters
                            //

                            //
                            // PHASE 1
                            //
                            // determine the actual parameter values and perform semantic
                            // analysis / type checking
                            //

                            let formal_parameter_count = define_symbol.formal_parameter_map.len();

                            // DEBUG
                            if debug {
                                println!("Expecting {} parameters.", formal_parameter_count);
                            }

                            // only extract and check for formal parameters if they are actually there!
                            // Some macros do not have formal parameters
                            // e.g. "#define EXPR_B 10"
                            if formal_parameter_count > 0 {

                                // clone because an iterator is used later and the iterator
                                // wants to own the object it iterates over exclusively so that
                                // the underlying object does not change during iteration
                                let mut arena_clone = arena.clone();

                                // // DEBUG
                                // if debug {
                                //     self.print_arena_dot(&arena_clone);
                                // }

                                let end_token = node.data.clone();

                                // retrieve parent
                                let parent_node: &Node<Token> = &arena_clone.nodes[node.parent.unwrap().index];

                                // descend into right child of parent
                                // the right node contains the list of actual parameters in
                                // comma-separated form
                                // the left node contains the macro/symbol's name
                                let right_node_id = parent_node.right.unwrap();

                                // DEBUG
                                if debug {
                                    println!("{:?}", &right_node_id);
                                }

                                let right_node: &Node<Token> = &arena_clone.nodes[right_node_id.index];

                                // DEBUG
                                if debug {
                                    println!("{:?}", &right_node);
                                }

                                //
                                // Start to iterate the RHS subtree which contains all the
                                // actual parameters
                                //

                                // set the start node id from where the iteration in the AST should start
                                arena_clone.iterator_current_node_id = right_node_id.index;

                                // first reset the state so that iteration can be performed
                                // since state is used to remember which nodes in the AST
                                // have been iterated over already
//                                arena_clone.set_visit_mode_for_all_nodes(right_node_id.index, VisitMode::VisitLeft);

                                // DEBUG
                                //println!("{}", define_symbol.name);

                                // DEBUG - output the entire tree
                                // if debug {
                                    print_arena_dot(&arena_clone, arena_clone.find_root());
                                // }

                                let mut replace_map = HashMap::<String, String>::new();
                                let mut actual_parameter_index: usize = 0;

                                self.extract_actual_parameters(&arena_clone, right_node, &right_node_id, &mut replace_map, &define_symbol.formal_parameter_map, &mut actual_parameter_index);

/*
                                // walk the tree to fill the parameter replace map
                                for token_node in arena_clone {

                                    let token = token_node.data;

                                    // the iterator over the tree nodes is implemented in a weird way.
                                    // It will loop back to the left node once it has iterated over the
                                    // actual parameters. To stop iterating after the formal parameters,
                                    // the left node is identified here as a signal that all formal
                                    // parameters have been processed.
                                    //
                                    // TODO: find a cleaner way to extract the formal parameters.
                                    // They are the leaves of the subtree...
                                    if token == end_token {
                                        break;
                                    }

                                    // DEBUG
                                    // if debug {
                                        println!("{}", token.text);
                                    // }

                                    if token.text == "," {

                                        // ignore comma (parameter separator)

                                    }
                                    else if token.text == "#" {

                                        panic!("######");

                                    }
                                    else {

                                        // fill formal and actual values into the replace_map
                                        let key_option = define_symbol.formal_parameter_map.get(&actual_parameter_index);
                                        let mut key = String::new();
                                        if let Some(key_from_option) = key_option {
                                            key = key_from_option.clone();
                                        } else {
                                            println!("\n\n---\n{:?}\n---\n\n", define_symbol);
                                            panic!("File: {}, Line: {}", token.file, token.line);
                                        }

                                        // let value = token.text.clone();

                                        // convert the entire subtree into a string
                                        let value = "";
                                        while

                                        // DEBUG
                                        println!("[Param-Replace-Map: Index: {}] key: '{}', value: '{}'", actual_parameter_index, key, value);

                                        // insert formal parameter as key and
                                        // actual parameter-value as value into the
                                        // parameter replace map
                                        replace_map.insert(key, value);

                                        // increment index
                                        actual_parameter_index = actual_parameter_index + 1;
                                    }
                                }
 */
                                // DEBUG print parameter replace map
                                // if self.debug {
                                    println!("^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^");
                                    for (key, value) in replace_map.clone().into_iter() {
                                        println!("key: {}, value: {}", key, value);
                                    }
                                    println!("^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^");
                                // }

                                // semantic analysis
                                if actual_parameter_index != formal_parameter_count {
                                    panic!("[ERR] Incorrect use of preprocessor macro \"{}\"!
                                        Parameter count mismatch! actual_parameter_count:{},
                                        formal_parameter_count:{}",
                                        define_symbol.name, actual_parameter_index, formal_parameter_count);
                                }

                                //
                                // PHASE 2
                                //
                                // replace formal parameters by actual parameters
                                //

                                // this is the AST that represents the defintion of the macro/symbol
                                // that is currently resolved
                                let mut arena_replace_clone = define_symbol.expression_parser.arena.clone();

                                // DEBUG - output the entire tree
                                // if debug {
                                    print_arena_dot(&arena_replace_clone, arena_replace_clone.find_root());
                                // }

                                // find the id of the root node of the macro/symbol definition AST
                                let root_index = arena_replace_clone.find_root();
                                // loop {
                                //     if let Some(root_index_temp) = arena_replace_clone.nodes[root_index].parent {
                                //         root_index = root_index_temp.index;
                                //     } else {
                                //         break;
                                //     }
                                // }

                                // DEBUG - before
                                // if debug {
                                    print_arena_dot(&arena_replace_clone, root_index);
                                // }

                                self.replace_parameters(&mut arena_replace_clone, root_index, &replace_map);

                                // DEBUG - after
                                // if debug {
                                    print_arena_dot(&arena_replace_clone, root_index);
                                // }

                                //
                                // PHASE 3
                                //
                                // replace the original node of the macro by the AST that
                                // represents it's definition
                                //

                                *arena = arena_replace_clone.clone();

                                /*
                                // let root_node: &Node<Token> =
                                //     &arena_replace_clone.nodes[define_symbol.expression_parser.ptr_node_id.index];

                                let root_node: &Node<Token> =
                                    &arena_replace_clone.nodes[root_index];

                                // DEBUG
                                if debug {
                                    println!("Replacing node by AST ...");
                                }

                                // retrieve parent
                                let parent_node_id = node.parent.unwrap();
                                let parent_node: &Node<Token> = &arena.nodes[parent_node_id.index];

                                // make the dst parent node the same as the source parent node
                                arena.change_payload(node_id, root_node.data.clone());

                                // copy LHS subtree and RHS subtree from src to dst
                                self.copy_tree(arena,
                                    &parent_node_id,
                                    &arena_replace_clone,
                                    &define_symbol.expression_parser.ptr_node_id);
                                */

                                return true;

                            } else {

                                //
                                // PHASE 3
                                //
                                // replace the original node of the macro by the AST that
                                // represents it's definition
                                //

                                let root_node: &Node<Token> =
                                    &define_symbol.expression_parser.arena.nodes[define_symbol.expression_parser.ptr_node_id.index];

                                // DEBUG
                                if debug {
                                    println!("Replacing node by AST ...");
                                }

                                // make the dst parent node the same as the source parent node
                                arena.change_payload(node_id, root_node.data.clone());

                                // copy LHS subtree and RHS subtree from src to dst
                                self.copy_tree(arena,
                                    node_id,
                                    &define_symbol.expression_parser.arena,
                                    &define_symbol.expression_parser.ptr_node_id);

                                // a symbol has been replaced.
                                // Start a new iteration to replace more symbols which
                                // might have been introduced by the replaced symbol
                                return true;
                            }
                        }
                    }
                    return false;

                } else if *val == String::from("PPF_DEFINED") {

                    // DEBUG
                    if debug {
                        println!("token.text: {}", token.text);
                    }

                    return false;
                }
            }

            _ => {

            }
        }

        return left_result || right_result;
    }

    pub fn replace_parameters(&self,
        arena: &mut Arena<Token>,
        root_index: usize,
        replace_map: &HashMap::<String, String>)
    {
        // retrieve the current node
        let root_node: &mut Node<Token> = &mut arena.nodes[root_index].clone();

        if let Some(left_node_id) = root_node.left {
            self.replace_parameters(arena, left_node_id.index, replace_map);
        }
        if let Some(right_node_id) = root_node.right {
            self.replace_parameters(arena, right_node_id.index, replace_map);
        }

        if replace_map.contains_key(&root_node.data.text) {
            let value = replace_map.get(&root_node.data.text).unwrap();
            arena.nodes[root_index].data.text = value.clone();
        }
    }

    /// will attach the LHS subtree of the src node to the LHS side of the dst node
    /// replacing the existing LHS subtree.
    /// will attach the RHS subtree of the src node to the RHS side of the dst node
    /// replacing the existing RHS subtree.
    pub fn copy_tree(&self,

        dst_arena: &mut Arena<Token>,
        dst_node_id: &NodeId,

        src_arena: &Arena<Token>,
        src_node_id: &NodeId
    ) {
        let src_node: &Node<Token> = &src_arena.nodes[src_node_id.index];
        let dst_node: &mut Node<Token> = &mut dst_arena.nodes[dst_node_id.index];

        // if the src has a LHS, create a LHS in the dst, replacing the existing LHS subtree
        if let Some(left_node_id) = src_node.left {
            let left_node: &Node<Token> = &src_arena.nodes[left_node_id.index];
            let dst_left_node_id = dst_arena.add_left(dst_node_id, left_node.data.clone());
            self.copy_tree(dst_arena, &dst_left_node_id, src_arena, &left_node_id);
        } else {
            dst_arena.remove_left(dst_node_id);
        }

        // if the src has a RHS, create a RHS in the dst
        if let Some(right_node_id) = src_node.right {
            let right_node: &Node<Token> = &src_arena.nodes[right_node_id.index];
            let dst_right_node_id = dst_arena.add_right(dst_node_id, right_node.data.clone());
            self.copy_tree(dst_arena, &dst_right_node_id, src_arena, &right_node_id);
        } else {
            dst_arena.remove_right(dst_node_id);
        }
    }

    pub fn evaluate_expression(&self,
        arena: &Arena<Token>,
        node_id: &NodeId,
        defined_symbol_map: &HashMap::<String, DefineSymbol>,
        debug: bool) -> i32
    {
        // DEBUG
        if debug {
            let root = arena.find_root();
            print_arena_dot(arena, root);
        }

        let node: &Node<Token> = &arena.nodes[node_id.index];

        let token: &Token = &node.data;

        // DEBUG
        if debug {
            println!("token: {:?}", token);
        }

        let mut has_left_child: bool = false;
        let mut has_right_child: bool = false;

        let mut left_result: i32 = 0;
        let mut right_result: i32 = 0;

        let mut left_child_id: usize = 0;
        let mut right_child_id: usize = 0;

        // LHS - output left child
        if let Some(left_id) = &node.left {
            has_left_child = true;
            left_result = self.evaluate_expression(arena, left_id, defined_symbol_map, debug);
            left_child_id = left_id.index;
        }

        // RHS - output right child
        if let Some(right_id) = &node.right {
            has_right_child = true;
            right_result = self.evaluate_expression(arena, right_id, defined_symbol_map, debug);
            right_child_id = right_id.index;
        }

        // DEBUG
        if debug {
            println!("left_result: {:?}", left_result);
            println!("right_result: {:?}", right_result);
        }

        match &token.terminal {

            Terminal(val) => {

                // DEBUG
                if debug {
                    println!("val: {}", val);
                }

                if *val == String::from("IDENTIFIER") {

                    // DEBUG
                    if debug {
                        println!("token.text: {}", token.text);
                    }

                    // if the symbol is contained in the symbol_map return the value
                    if defined_symbol_map.contains_key(&token.text) {
                        let defineSymbol: &DefineSymbol = defined_symbol_map.get(&token.text).unwrap();
                        let definition = defineSymbol.definition.trim();

                        // DEBUG
                        if debug {
                            println!("definition: '{}'", definition);
                        }

                        match definition.parse::<i32>() {
                            Result::Ok(numeric_value) => {
                                return numeric_value;
                            }
                            Result::Err(_) => {
                                // TODO before returning true, try to evaluate the definition!
                                // e.g. if the definition is 1 + 1, then this could be evaluated
                                // to 2 and 2 could be returned!
                                return 0;
                            }
                        }
                    } else {
                        match token.text.parse::<i32>() {
                            Result::Ok(numeric_value) => {
                                return numeric_value;
                            }
                            Result::Err(_) => {
                                // TODO before returning true, try to evaluate the definition!
                                // e.g. if the definition is 1 + 1, then this could be evaluated
                                // to 2 and 2 could be returned!
                                return 0;
                            }
                        }
                    }

                } else if *val == String::from("PPF_DEFINED") {

                    // DEBUG
                    if debug {
                        println!("token.text: {}", token.text);
                    }

                    let right_node: &Node<Token> = &arena.nodes[right_child_id];

                    // DEBUG
                    if debug {
                        println!("right_node.data.text: {}", right_node.data.text);
                    }

                    // brackets are optional:
                    // #if defined MACRO_1
                    // #if defined(MACRO_1)
                    //
                    // jump over brackets and directly access the macro identifier

                    let mut macro_identifier: String = String::new();

                    if right_node.data.text == "()" {
                        let right_sub_node: &Node<Token> = &arena.nodes[right_node.right.unwrap().index];
                        macro_identifier = right_sub_node.data.text.clone();
                    } else {
                        macro_identifier = right_node.data.text.clone();
                    }

                    if defined_symbol_map.contains_key(&macro_identifier) {
                        return 1;
                    } else {
                        return 0;
                    }

                } else if *val == String::from("EXCLAMATION_MARK") {

                    // zero means false and is inverted to true
                    if right_result == 0 {
                        return 1;
                    };
                    // any non-zero value means true and is inverted to false
                    return 0;

                } else if *val == String::from("NUMERIC") {

                    return token.text.parse::<i32>().unwrap_or(0);

                } else if *val == String::from("PLUS") {

                    return left_result + right_result;

                } else if *val == String::from("ASTERISK") {

                    return left_result * right_result;

                } else if *val == String::from("OPENING_BRACKET") {

                    return right_result;

                } else if *val == String::from("EQ_OP") {

                    if right_result == left_result {
                        return 1;
                    };
                    return 0;

                } else if *val == String::from("NE_OP") {

                    if right_result != left_result {
                        return 1;
                    };
                    return 0;

                } else if *val == String::from("AND_OP") {

                    if right_result != 0 && left_result != 0 {
                        return 1;
                    };
                    return 0;

                } else if *val == String::from("OR_OP") {

                    if right_result != 0 || left_result != 0 {
                        return 1;
                    };
                    return 0;

                } else if *val == String::from("GT") {

                    if right_result > left_result {
                        return 1;
                    };
                    return 0;

                } else if *val == String::from("LT") {

                    if right_result < left_result {
                        return 1;
                    };
                    return 0;

                } else {
                    panic!("Unhandled: {}", *val);
                    //println!("Unhandled: {}", *val);
                }
            }

            _ => {
                panic!("test");
            }
        }
    }

    /// first, checks if the macro definition is already available
    /// as an AST. If not, the macro definition is parsed into an
    /// AST and stored into the definition.
    ///
    /// In general a definition does not have to be well-formed
    /// and this means it cannot be converted into an AST in the
    /// general case. Use this function only if the defintion is
    /// well-formed. For example, one way to figure out if a
    /// macro definition is well-formed is if the symbol is used
    /// in a #if, #elif PPI in which case it has to be well-formed
    /// because it needs to be evaluated. Only well-formed
    /// definitions can be evaluated
    fn parse_definition_into_AST(&self,
        // has to be mutable, because this function will parse the definition into an AST
        // and that AST is stored inside the expression parser which therefore will be
        // mutated which means the Define Symbol will be mutated
        define_symbol: &mut DefineSymbol,
        preprocessor_dfa: &EpsilonNfa<State, RegexBuildingBlock>,
        file: &String,
        line: usize,
        debug: bool)
    {
        // the expression parser stores the AST in it's internal arena.
        // If the arena is empty, the macro defintion has not been
        // parsed into an AST yet!
        if define_symbol.expression_parser.arena.is_empty() {

            // DEBUG
            if debug {
                println!("macro definition:'{}'", define_symbol.definition);
            }

            let lexer_debug: bool = false;
            let lexer_token_debug: bool = debug;

            let mut internal_lexer: Lexer = Lexer::new(
                define_symbol.definition.clone(),
                preprocessor_dfa,
                lexer_debug,
                lexer_token_debug);

            internal_lexer.file = file.clone();
            internal_lexer.line = line;

            for token in internal_lexer {
                define_symbol.expression_parser.process_token(token);
            }
        }

        // DEBUG
        if self.debug {
            let root_id = define_symbol.expression_parser.arena.find_root();
            print_arena_dot(&define_symbol.expression_parser.arena, root_id);
        }
    }
}

pub fn print_arena_dot(arena: &Arena<Token>, node_id: usize) {

        let mut ast_string_buffer = String::from("");

        let root = NodeId { index: node_id };

        // serialize the AST into .dot graphviz format
        ast_string_buffer.push_str("digraph {\n");
        recurse_arena_dot(&arena, &root, &mut ast_string_buffer);
        ast_string_buffer.push_str("}");

        // 1. Create or overwrite the file
        let file = File::create("dot/preprocessor_tree.dot").expect("Create file failed!");

        // 2. Wrap the file in a BufWriter
        let mut writer = BufWriter::new(file);

        // 3. Write data
        write!(writer, "{}", ast_string_buffer);

        // 4. Explicitly flush the remaining data to disk
        writer.flush().expect("flush failed!");
    }

