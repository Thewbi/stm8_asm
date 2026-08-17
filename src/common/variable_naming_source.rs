use std::cell::RefCell;

use std::sync::atomic::AtomicUsize;
use crate::Ordering;

use std::collections::{HashMap, HashSet, BTreeSet, BTreeMap};

use core::error::Error;

// https://stackoverflow.com/questions/32935808/generate-sequential-ids-for-each-instance-of-a-struct
static VAR_COUNTER: AtomicUsize = AtomicUsize::new(0);

// 
// he VariableNamingSource is used to
// output unique variable names and maintains a map from user choosen 
// varible name to unique variable name.
//
// The VariableNamingSource maintains a stack of mappings
// between user choosen varible name to unique variable name.
// This stack of mappings is used to implement block scopes in which
// variables can be defined and are valid only within the scope they
// are defined in.
//

struct VarnameMapEntry {

    varname: String,

    // If a variable was declared in the current scope, it is new by defintion. 
    // If a entry is originating from lower down the stack it is not new by definition.
    is_new: bool,

     // used to control whether the user-choosen identifier is replaced by a artificial 
     // unique name (for local variables) or if it is kept (for global symbols such as 
     // functions that need to have the same name accross compilation units so the linker can link them)
    is_external_linkage: bool,
}

impl Clone for VarnameMapEntry {
    fn clone(&self) -> Self {
        VarnameMapEntry { 
            varname: self.varname.clone(),
            is_new: false, // make the clone an old instance by default
            is_external_linkage: false,
        }
    }
}

pub struct VariableNamingSource {
    varname_map_stack: Vec::<RefCell<HashMap::<String, VarnameMapEntry>>>, // vector/stack of varname maps
}

impl VariableNamingSource {

    pub fn new() -> VariableNamingSource {

        let instance = VariableNamingSource {
            varname_map_stack: Vec::<RefCell<HashMap::<String, VarnameMapEntry>>>::new(),
        };

        instance
    }

    pub fn enter_scope(&mut self) {
        // DEBUG
        // println!("[VariableNamingSource::enter_scope]");

        let mut varname_map = HashMap::<String, VarnameMapEntry>::new();

        // adding a new scope means copying the current scope and setting all variables to the state is_new = false
        let stack_size = self.varname_map_stack.len();
        if stack_size > 0 {
            let varname_map_old = self.varname_map_stack[stack_size - 1].borrow();
            varname_map = varname_map_old.clone();
        }

        let ref_cell = RefCell::new(varname_map);
        self.varname_map_stack.push(ref_cell);
    }

    pub fn exit_scope(&mut self) {
        // DEBUG
        // println!("[VariableNamingSource::exit_scope]");

        self.varname_map_stack.pop();
    }

    pub fn is_variable_name_defined(&mut self, varname: &String) -> bool {

        // look into the topmost map 
        // (which automatically contains ALL variable from ALL levels! No need to go down the stack)
        if let Some(varname_map_refcell) = self.varname_map_stack.last() {

            let varname_map = varname_map_refcell.borrow_mut();
            if varname_map.contains_key(varname) {

                let entry = varname_map.get(varname).unwrap();

                // conflict if the identifier was newly defined in this scope and at the same time has no external linkage
                // (= this means it refers to another object which has that name already!)
                //
                // More information: Nora Sandler, page 174ff
                return entry.is_new && !entry.is_external_linkage;
            }
        }

        // not defined
        false
    }

    // enum Result<T, E> {
    //  Ok(T),
    //  Err(E),
    // }
    pub fn get_replaced_variable_name(&mut self, varname: &String) -> Result<String, String> {
        
        // retrieve the topmost variable name map
        let stack_size = self.varname_map_stack.len();
        if stack_size > 0 {
            let varname_map = self.varname_map_stack[stack_size - 1].borrow();

            // DEBUG
            println!("Trying to resolve variable: \"{}\"", varname);

            if !varname_map.contains_key(varname) {
                return Err(format!("Variable \"{}\" not defined!", varname).into());
            }

            return Ok(varname_map.get(varname).unwrap().varname.clone());
        }

        return Err(format!("Variable \"{}\" not defined!", varname).into());
    }

    pub fn new_temp_var(&mut self) -> String {

        let temp = VAR_COUNTER.fetch_add(1, Ordering::SeqCst);

        let mut t = String::from("tmp.");
        t.push_str(temp.to_string().as_str());

        t
    }

    pub fn new_user_defined_var(&mut self, varname: &String) -> String {

        let in_use = self.is_variable_name_defined(&varname);
        if in_use {
            panic!("Variable Name \"{}\" is used already!", varname);
        }

        let temp = VAR_COUNTER.fetch_add(1, Ordering::SeqCst);

        let mut new_unique_varname = String::from("userdef_var.");
        new_unique_varname.push_str(temp.to_string().as_str());

        // map var name to new temp name
        if let Some(varname_map) = self.varname_map_stack.last() {

            let varname_map_entry: VarnameMapEntry = VarnameMapEntry { 
                varname: new_unique_varname.clone(),
                is_new: true, // new define
                is_external_linkage: false, // a variable is NOT external linkage by default
            };
            varname_map.borrow_mut().insert(varname.clone(), varname_map_entry);
        }

        new_unique_varname
    }

    // if the function declaration has external linkage, then do not change the name but
    // keep the user-choosen name so that the linker can correlate the name in other libraries and object files
    pub fn new_function_declaration(&mut self, func_name: &String) -> String {

        // TODO: if there is a variable name that matches the function name and the variable has no linkage,
        // then it is not allowed to reuse that name, since it refers to a different object and a single name
        // is not enough to make the objects unique!
        
        let in_use = self.is_variable_name_defined(&func_name);
        if in_use {
            panic!("Variable Name \"{}\" is used already!", func_name);
        }

        // map var name to new temp name
        if let Some(varname_map) = self.varname_map_stack.last() {

            let varname_map_entry: VarnameMapEntry = VarnameMapEntry { 
                varname: func_name.clone(),
                is_new: true, // new define
                is_external_linkage: true, // a function is external linkage by default
            };
            varname_map.borrow_mut().insert(func_name.clone(), varname_map_entry);
        }

        func_name.clone()
    }
}