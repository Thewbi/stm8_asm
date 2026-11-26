use std::array;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::asm_ast::asm_ast::AstAstAssemblyType;
use crate::asm_ast::asm_ast::retrieve_base_register;
use crate::common::data_type::DataType;
use crate::common::symbol_table::SymbolTable;
use crate::common::symbol_table::SymbolTableEntry;
use crate::tacky::tacky::ValueElement;

use crate::AsmAstProgram;

use crate::asm_ast::asm_ast::AsmAstFunction;
use crate::asm_ast::asm_ast::AsmAstInstruction;
use crate::asm_ast::asm_ast::AsmAstInstructionType;
use crate::asm_ast::asm_ast::AsmAstOperand;
use crate::asm_ast::asm_ast::AsmAstOperandType;
use crate::asm_ast::asm_ast::AsmAstBinaryOperator;
use crate::asm_ast::asm_ast::AsmAstReg;

//
// Fixes up the intermediate Asm AST before the intermediate AST
// goes into the Assembly Generation (AsmAstMasmEmitterVisitor, GASEmitter... )
//
// Before this struct runs, there is naming replacement, type checking, TACKY generation.
//
// The intermediate ASM is fixed by:
//
// - replacing pseudo variables/operands with stack addresses
// - replacing mov instructions which move from memory to memory
//   without using a intermediate temp register (not possible in x86)
//
// 1. src\c_ast\identifier_resolution_visitor.rs - checks for duplicate or undeclared variable names
// 2. src\c_ast\type_checking_visitor.rs - performs type checking. The rules that are checked are documented in the file itself
// 3. src\tacky\tacky_visitor.rs - Generate TACKY (from AST)
// 4. asm_ast/TackyToIntermediateAsmConversionVisitor - Converts the AST into a ASM AST for assembly with a precursory form of mnenomics
// 5. asm_ast/AsmAstFixupVisitor - In the intermediate ASM language, replacing pseudo operands/variables with stack addresses
// 6. asm_ast/AsmAstMasmEmitterVisitor / asm_ast/AsmAstASEmitterVisitor / ...
//

pub struct AsmAstFixupVisitor {
    pub asm_ast_program: AsmAstProgram,
    pub replace_pseudo: bool,
    stack_offset: i32,
    pub stack_offset_map: HashMap::<String, i32>,
    pub stack_offset_to_symbol_map: HashMap::<String, SymbolTableEntry>,
    // chapter 15 - talks about a new backend symbol table that is create from the symbol table after TACKY generation
    pub backend_symbol_table: Rc<RefCell<SymbolTable>>, // https://www.youtube.com/watch?v=8O0Nt9qY_vo
    pub type_system: Rc<RefCell<HashMap::<String, DataType>>>,
    pub debug: bool,
}

impl AsmAstFixupVisitor {

    pub fn new(backend_symbol_table_param: Rc<RefCell<SymbolTable>>,
        type_system_param: Rc<RefCell<HashMap::<String, DataType>>>) -> AsmAstFixupVisitor
    {
        AsmAstFixupVisitor {
            asm_ast_program: AsmAstProgram::new(),
            replace_pseudo: false,
            stack_offset: 0,
            stack_offset_map: HashMap::<String, i32>::new(),
            stack_offset_to_symbol_map: HashMap::<String, SymbolTableEntry>::new(),
            backend_symbol_table: backend_symbol_table_param,
            type_system: type_system_param,
            debug: false,
        }
    }

    //
    // Helper/Util Functions
    //

    #[allow(unreachable_code)] // still under development, so enums will be extended and the match should catch unhandled options so the catch-all case needs to stay even if it throws warnings
    #[allow(unreachable_patterns)] // still under development, so enums will be extended and the match should catch unhandled options so the catch-all case needs to stay even if it throws warnings
    pub fn replace_pseudo_operand(&mut self,
        asm_ast_operand: &AsmAstOperand) -> AsmAstOperand
    {
        // DESCRIPTION:
        //
        // This is where Nora Sandler, page 267 wants to check the backend symbol
        // table for the size of each pseudo variable in order to correctly enlarge
        // the stack frame with the size consumed by the pseudo variable.
        //
        // Special Case: a function declaration defines pseudo variables for the
        // parameters in it's prototype which do not neet to reserve space on the stack!
        //
        // int - 4 Byte
        // long - 8 Byte
        // pointer to anyhting - 8 Byte

        match &asm_ast_operand.operand_type {

            AsmAstOperandType::Imm(immediate_value) => {
                // DEBUG
                if self.debug {
                    println!("Imm");
                }
            }

            AsmAstOperandType::UnsignedImm(immediate_value) => {
                // DEBUG
                if self.debug {
                    println!("UnsignedImm");
                }
            }

            AsmAstOperandType::Reg(register_name) => {
                // DEBUG
                if self.debug {
                    println!("Reg, register_name:{:?}", register_name);
                }
            }

            AsmAstOperandType::DerefReg(register_name) => {
                // DEBUG
                if self.debug {
                    println!("DerefReg, register_name:{:?}", register_name);
                }
            }

            AsmAstOperandType::BaseRegWithOffsetAndSize(base_reg, offset_reg, data_type_size) => {
                // DEBUG
                if self.debug {
                    println!("BaseRegWithOffsetAndSize, base_reg:{:?}, offset_reg:{:?}, data_type_size:{:?}", base_reg, offset_reg, data_type_size);
                }
            }

            AsmAstOperandType::Pseudo(pseudo_name) => {

                // // DEBUG
                // if pseudo_name == "x" {
                //     panic!();
                // }
                // if pseudo_name == "y" {
                //     panic!();
                // }

                // DEBUG
                if self.debug {
                    println!("[ASM Fixup Visitor::replace_pseudo_operand] Pseudo: Pseudo-Name: '{}'", pseudo_name);
                }

                assert!(pseudo_name.len() != 0,
                    "[ERR] [asm_ast_fixup_visitor] PseudoVariable has no name! Need a name to replace!");

                let mut stack_offset_value = 0i32;

                if self.replace_pseudo {

                    if self.stack_offset_map.contains_key(pseudo_name) {

                        // variable with 'pseudo_name' has already been resolved.
                        // Reuse existing value.
                        stack_offset_value = *self.stack_offset_map.get(pseudo_name).unwrap();

                    } else {

                        // retrieve type
                        if self.backend_symbol_table.borrow().contains(&pseudo_name) {

                            // first, determine the data type. This is needed in order to determine
                            // the correct amount of bytes to increment the stack pointer with

                            let st = self.backend_symbol_table.borrow();
                            let symbol_table_entry = st.retrieve(&pseudo_name);

                            // pointer or not a pointer?
                            if symbol_table_entry.is_pointer {

                                // if the variable is identified to be a pointer (to any type),
                                // then the pointer will always use the same amount of bytes on
                                // the stack.
                                // On 64-bit platform, a pointer/address is 8 Byte
                                // On 32-bit platform, a pointer/address is 4 Byte

                                self.stack_offset = self.stack_offset - 8; // hard coded assume 64-Bit right now!

                            } else {

                                // if the type is a normal data type, use the data types size
                                // to store it on the stack

                                let data_type = symbol_table_entry.data_type.clone();

                                let data_type_size = data_type.get_size(&self.type_system.borrow()) as i32;

                                if self.debug {
                                    println!("[AsmFixupVisitor] variable-pseudo_name: '{}', data_type: {}, data_type_size: {}",
                                        pseudo_name, data_type, data_type_size);
                                }

                                // convert the data type into the offset in bytes
                                self.stack_offset = self.stack_offset - data_type_size;

                            }

                            // DEBUG
                            if self.debug {
                                println!("self.stack_offset: {}", self.stack_offset);
                            }

                            self.stack_offset_map.insert(pseudo_name.to_string(), self.stack_offset);
                            self.stack_offset_to_symbol_map.insert(pseudo_name.to_string(), symbol_table_entry.clone());

                            // update stack offset_value
                            stack_offset_value = self.stack_offset;

                        } else {
                            println!("[AsmAstFixupVisitor][ERR] Cannot find symbol '{}' in backend_symbol_table", &pseudo_name);
                            println!("[AsmAstFixupVisitor][ERR] Printing symbol table for debugging ...");
                            // DEBUG
                            if self.debug {
                                self.backend_symbol_table.borrow().print_symbol_table();
                            }
                            panic!("Cannot find symbol '{}' in backend_symbol_table", &pseudo_name);
                        }
                    }
                }

                // DEBUG
                if self.debug {
                    println!("[ASM Fixup Visitor::replace_pseudo_operand] Pseudo: Pseudo-Name: '{}' -> stack_offset_value: '{}'", pseudo_name, stack_offset_value);
                }

                return AsmAstOperand {
                    operand_type: AsmAstOperandType::Memory(AsmAstReg::RBP, stack_offset_value)
                };
            }

            AsmAstOperandType::PseudoMem(pseudo_name, pseudo_offset) => {
                // DEBUG
                if self.debug {
                    println!("[ASM_AST_FIXUP_VISITOR::replace_pseudo_operand] PseudoMem: pseudo_name: '{}'", pseudo_name.clone());
                }

                if self.replace_pseudo {

                    // let mut stack_offset_value = 0i32;

                    //
                    // Here, replace the PseudoMem(pseudo_name, pseudo_offset) by a stack address
                    // that is large enough to store the array with all it's elements
                    //

                    if self.stack_offset_map.contains_key(pseudo_name) {

                        // variable with 'pseudo_name' has already been resolved.
                        // Reuse existing value.
                        // stack_offset_value = *self.stack_offset_map.get(pseudo_name).unwrap();

                    } else {

                        // retrieve type
                        if self.backend_symbol_table.borrow().contains(&pseudo_name) {

                            // first, determine the data type. This is needed in order to determine
                            // the correct amount of bytes to increment the stack pointer with
                            let symbol_table_entry =
                                self.backend_symbol_table.borrow().retrieve(&pseudo_name);

                            // DEBUG
                            if self.debug {
                                println!("[AsmAstFixupVisitor::PseudoMem()] '{:?}'", symbol_table_entry);
                            }

                            if symbol_table_entry.is_pointer {
                                panic!("pointer!");
                            }

                            let mut array_element_count = symbol_table_entry.array_element_count;
                            if symbol_table_entry.is_array {
                                // DEBUG
                                if self.debug {
                                    println!("is_array. ElementCount: {}", symbol_table_entry.array_element_count);
                                }
                                assert_ne!(symbol_table_entry.array_element_count, 0, "An array type variable should not have a length of zero!");
                            } else {
                                array_element_count = 1;
                            }

                            assert_ne!(array_element_count, 0);

                            let data_type = symbol_table_entry.data_type.clone();

                            // convert the data type into the offset in bytes
                            let data_type_size = data_type.get_size(&self.type_system.borrow()) as i32;

                            // DEBUG
                            if self.debug {
                                println!("[ASM_AST_FIXUP_VISITOR::replace_pseudo_operand] pseudo_name: '{}', DataType: {:?}, DataTypeSize: {}",
                                    pseudo_name.to_string(), data_type, data_type_size);
                            }

                            // DEBUG
                            if self.debug {
                                println!("data_type_size: {}", data_type_size);
                                println!("array_element_count: {}", array_element_count);
                            }

                            // update stack offset (for normal variables and for arrays)

                            //TODO: This is a problem with determining the size of a DataType
                            //The data type of an array datatype !?!? is it the size of the array
                            // Meaning type * element_count or only the inner element type?

                            //self.stack_offset = self.stack_offset - data_type_size * array_element_count;
                            self.stack_offset = self.stack_offset - data_type_size;

                            self.stack_offset_map.insert(pseudo_name.to_string(), self.stack_offset);
                            self.stack_offset_to_symbol_map.insert(pseudo_name.to_string(), symbol_table_entry.clone());

                            // remember new stack offset
                            // stack_offset_value = self.stack_offset;

                        } else {
                            panic!("Cannot find symbol '{}' in backend_symbol_table", &pseudo_name);
                        }
                    }
                }
            }

            AsmAstOperandType::Memory(register, stack_offset) => {
                // DEBUG
                if self.debug {
                    println!("Stack, offset:{}", stack_offset);
                }

                return asm_ast_operand.clone();
            }

            AsmAstOperandType::DirectGetAddress(symbol_name) => {
                // nothing
            }

            _ => {
                println!("[ASM Fixup Visitor] asm_ast_operand: {:?}", asm_ast_operand);

                panic!("{}", format!("ASM Fixup Visitor] Unhandled operand_type {:?}!\n", asm_ast_operand.operand_type).as_str());

                return asm_ast_operand.clone();
            }
        }

        asm_ast_operand.clone()
    }

    fn untangle_mem_mem(&mut self,
        asm_ast_instruction: &mut AsmAstInstruction,
        new_body: &mut Vec::<Box<AsmAstInstruction>>,
        intermediate_register: AsmAstReg)
    {
        let mut fix: bool = false;

        let mut new_asm_ast_instruction = asm_ast_instruction.clone();

        // x86 mov cannot move from memory (stack or other memory) to memory directly!
        if matches!(asm_ast_instruction.src_2.operand_type, AsmAstOperandType::Memory(_, _)) {
            if matches!(asm_ast_instruction.dst.operand_type, AsmAstOperandType::Memory(_, _)) {
                fix = true;
            }
        }

        if fix {

            // mov dword ptr [ebp-8], R10
            let mut mov_1 = new_asm_ast_instruction.clone();
            mov_1.comment.push_str(" ; [AsmAstFixUpVisitor::untangle_mem_mem] - clone 2");
            mov_1.instruction_type = AsmAstInstructionType::Mov;
            mov_1.src = new_asm_ast_instruction.src_2.clone();
            mov_1.dst = AsmAstOperand {
                operand_type: AsmAstOperandType::Reg(intermediate_register.clone()) };
            // insert instruction into the output
            new_body.push(Box::new(mov_1));

            // add
            // https://www.felixcloutier.com/x86/add
            // Adds the destination operand (first operand) and the source operand (second operand)
            // and then stores the result in the destination operand.
            new_asm_ast_instruction.src_2 = AsmAstOperand {
                operand_type: AsmAstOperandType::Reg(intermediate_register.clone()) };

            // // DEBUG
            // if self.debug {
            //     println!("{}", new_asm_ast_instruction);
            // }

            // insert instruction into the output
            new_body.push(Box::new(new_asm_ast_instruction));

        } else {

            new_body.push(Box::new(new_asm_ast_instruction));

        }
    }

    //
    // Member/Impl Functions
    //

    pub fn visit_asm_ast_program(&mut self,
        asm_ast_program: &mut AsmAstProgram)
    {
        // DEBUG
        if self.debug {
            println!("[FixupAsmAstVisitor::visit_asm_ast_program()]");
            println!("  name = {}", asm_ast_program.name);
        }

        for i in 0..asm_ast_program.functions.len() {

            // DEBUG
            if self.debug {
                let instr = &asm_ast_program.functions[i];
                println!("instr: {:?}", instr);
            }

            self.visit_asm_ast_function(&mut asm_ast_program.functions[i]);
        }
    }

    // will iterate over all instructions in the function's body and visit them
    pub fn visit_asm_ast_function(&mut self,
        asm_ast_function: &mut AsmAstFunction)
    {
        // DEBUG
        if self.debug {
            println!("[FixupAsmAstVisitor::visit_asm_ast_function()]");
            println!("  name = {}", asm_ast_function.name);
            println!("  body.len() = {}", asm_ast_function.body.len());
        }

        // reset stack offset to get rid of the stale value from the last function definition
        self.stack_offset = 0;

        // self.stack_offset_map.clear();
        // self.stack_offset_to_symbol_map.clear();

        // iterate over all instructions in the body and visit each.
        //
        // Each visited element will potentially insert more statements to the statements in the body.
        // To manage this feature of adding more nodes, a vector called new_body is inserted into the
        // recursion and instead of mutating the original body vector, the statements along with the
        // newly created statements are inserted in this 'new_body' statement.

        // During iteration over all statements in the body, each statement is visited.
        //
        // If a visit encounters the use of a pseudo-variable, this pseudo-variable needs to be
        // replaced by a stack address. When using more memory on the stack, the visited element
        // needs to update the self.stack_offset member variable to extend the stack frame.
        //
        // To replace a pseudo variable, each visited element will call the function:
        // replace_pseudo_operand(). Check for the usage of replace_pseudo_operand() in this
        // visitor to see where pseudo variables are replaced.
        let mut new_body = Vec::<Box<AsmAstInstruction>>::new();
        for i in 0..asm_ast_function.body.len() {

            // DEBUG
            if self.debug {
                let instruction = &asm_ast_function.body[i].as_ref().clone();
                println!("\n\n instruction: {:?}", instruction);
            }

            let instruction = asm_ast_function.body[i].as_ref().clone();
            self.visit_asm_ast_instruction(&mut new_body, instruction);
        }

        // if this is not the call which is used to print but the call which is used to compute
        if self.replace_pseudo {

            let mut vv: usize = (self.stack_offset * -1) as usize;
            vv = vv.next_multiple_of(16);

            let val = vv as i32;

            // patch allocate stack because right now,
            // the required stack size is readily available

            // first instruction is AllocateStack
            let asm_ast_allocate_stack = &mut new_body[0];

            // replace the src parameter
            asm_ast_allocate_stack.src = AsmAstOperand {
                operand_type: AsmAstOperandType::Imm(val)
            };
            asm_ast_allocate_stack.comment.push_str("\n    ; Updated in AsmAstFixupVisitor::visit_asm_ast_function()");

            // update stack frame size
            asm_ast_function.stack_frame_size = val;
        }

        // DEBUG - print the body
        if self.debug {
            let print_body = false;
            if print_body {
                println!("-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-");
                for i in 0..new_body.len() {
                    println!("{}", new_body[i]);
                }
                println!("-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-");
            }
        }

        // DEBUG - output the stack frame layout
        if self.debug {
            println!("[ASM AST FIXUP Visitor] stack_offset_map for function: '{}' --------------------------------------------", &asm_ast_function.name);
            let mut index = 0;
            for (key, value) in self.stack_offset_map.clone().into_iter() {
                //println!("{}) {} / {:?} ", index, key, value);
                // println!("{} / {:?}", key, value.data_type);
                let symbol_table_entry: &SymbolTableEntry = self.stack_offset_to_symbol_map.get(&key).unwrap();
                println!("{}) name:'{}', offset: {:?}, size: {}",
                    index,
                    symbol_table_entry.initial_name,
                    value,
                    symbol_table_entry.data_type.get_size(&self.type_system.borrow_mut()));
                index = index + 1;
            }
            println!("---------------------------------------------------------------------------------");
        }

        asm_ast_function.body = new_body;
    }

    #[allow(unreachable_code)] // still under development, so enums will be extended and the match should catch unhandled options so the catch-all case needs to stay even if it throws warnings
    #[allow(unreachable_patterns)] // still under development, so enums will be extended and the match should catch unhandled options so the catch-all case needs to stay even if it throws warnings
    pub fn visit_asm_ast_instruction(&mut self,
        new_body: &mut Vec::<Box<AsmAstInstruction>>,
        mut asm_ast_instruction: AsmAstInstruction)
    {
        // // DEBUG
        // if asm_ast_instruction.id == 28 {
        //     println!("asm_ast_instruction: {:?}", asm_ast_instruction);
        // }

        match asm_ast_instruction.instruction_type {

            AsmAstInstructionType::Break => {
                new_body.push(Box::new(asm_ast_instruction));
            }

            AsmAstInstructionType::Ret => {
                // DEBUG
                if self.debug {
                    println!("Ret");
                    println!("{:?}", asm_ast_instruction);
                }

                new_body.push(Box::new(asm_ast_instruction));
            }

            AsmAstInstructionType::Mov => {
                // DEBUG
                if self.debug {
                    println!("Mov {:?} {:?}", asm_ast_instruction.src, asm_ast_instruction.dst);
                }

                // // DEBUG
                // if asm_ast_instruction.id == 28 {
                //     println!("[ASM_AST_FIXUP_VISITOR] {:?}", asm_ast_instruction);
                // }

                // replace pseudo operand by relative address on stack
                asm_ast_instruction.src = self.replace_pseudo_operand(&mut asm_ast_instruction.src);
                asm_ast_instruction.dst = self.replace_pseudo_operand(&mut asm_ast_instruction.dst);

                if self.replace_pseudo {

                    // clear the destination register of stale data if parts of the register are
                    // not overwritten by the move (which is the case for all types != quadword)
                    //
                    // only clear the destination register, if it is not part of the pointer
                    // expression!

                    if asm_ast_instruction.assembly_type != AstAstAssemblyType::Quadword
                        && asm_ast_instruction.assembly_type != AstAstAssemblyType::UnsignedQuadword {

                        match &asm_ast_instruction.dst.operand_type {

                            AsmAstOperandType::Reg(register_name) => {

                                let mut xor_asm_ast_instruction: AsmAstInstruction = AsmAstInstruction::new();
                                xor_asm_ast_instruction.instruction_type = AsmAstInstructionType::Xor;
                                xor_asm_ast_instruction.assembly_type = AstAstAssemblyType::Quadword;
                                xor_asm_ast_instruction.comment = String::from(format!("abc id = {}", xor_asm_ast_instruction.id).as_str());
                                xor_asm_ast_instruction.src.operand_type = AsmAstOperandType::Reg(register_name.clone());
                                xor_asm_ast_instruction.dst.operand_type = AsmAstOperandType::Reg(register_name.clone());

                                if xor_asm_ast_instruction.id == 69 {
                                    println!("test: {:?}", asm_ast_instruction);
                                }

                                let mut insert_clear_register: bool = true;
                                match &asm_ast_instruction.offset.operand_type {
                                    AsmAstOperandType::Reg(offset_register_name) => {
                                        if retrieve_base_register(register_name) == retrieve_base_register(offset_register_name) {
                                            insert_clear_register = false;
                                        }
                                    }
                                    _ => {
                                    }
                                }

                                if insert_clear_register {
                                    new_body.push(Box::new(xor_asm_ast_instruction));
                                }
                            }

                            _ => {
                                // panic!();
                            }
                        }
                    }

                    // does this instruction require a fix?
                    let mut fix: bool = false;

                    // x86 mov cannot move from memory (stack or other memory) to memory directly!
                    if matches!(asm_ast_instruction.src.operand_type, AsmAstOperandType::Memory(_, _)) || matches!(asm_ast_instruction.src.operand_type, AsmAstOperandType::PseudoMem(_, _)) {
                        if matches!(asm_ast_instruction.dst.operand_type, AsmAstOperandType::Memory(_, _)) || matches!(asm_ast_instruction.dst.operand_type, AsmAstOperandType::PseudoMem(_, _)) {
                            fix = true;
                        }
                    }

                    // x86 cmp cannot compare immediate to immediate directly!
                    if matches!(asm_ast_instruction.src_2.operand_type, AsmAstOperandType::Imm(_)) {
                        if matches!(asm_ast_instruction.dst.operand_type, AsmAstOperandType::Imm(_)) {
                            fix = true;
                        }
                    }

                    // if the operands won't assemble/compile, change them!
                    // fix = false;
                    if fix {

                        let mut mov_1 = asm_ast_instruction.clone();
                        mov_1.instruction_type = AsmAstInstructionType::Mov;
                        mov_1.comment.push_str(" ; Added by FixupVistor - clone 1");
                        mov_1.dst = AsmAstOperand { operand_type: AsmAstOperandType::Reg(AsmAstReg::R10) };
                        // println!("{:?}", &mov_1);
                        new_body.push(Box::new(mov_1));
                        asm_ast_instruction.src = AsmAstOperand {
                            operand_type: AsmAstOperandType::Reg(AsmAstReg::R10) };
                        new_body.push(Box::new(asm_ast_instruction));

                    } else {

                        new_body.push(Box::new(asm_ast_instruction));

                    }

                } else {

                    new_body.push(Box::new(asm_ast_instruction));

                }
            }

            AsmAstInstructionType::Movsx => {
                new_body.push(Box::new(asm_ast_instruction));
             }

            AsmAstInstructionType::Unary => {
                // DEBUG
                // if self.debug {
                //    println!("Unary: {:?} {:?}", asm_ast_instruction.unary_operator, asm_ast_instruction.dst);
                // }

                // println!("[FixupVisitor] {:?}", asm_ast_instruction);

                let mut fix = false;
                if matches!(asm_ast_instruction.src.operand_type, AsmAstOperandType::Memory(_, _)) {
                    if matches!(asm_ast_instruction.dst.operand_type, AsmAstOperandType::Memory(_, _)) {
                        fix = true;
                    }
                }
                if matches!(asm_ast_instruction.src.operand_type, AsmAstOperandType::Pseudo(_)) {
                    if matches!(asm_ast_instruction.dst.operand_type, AsmAstOperandType::Pseudo(_)) {
                        fix = true;
                    }
                }

                // fix = false;
                if fix {

                    // mov dword ptr [ebp-8], R10
                    let mut mov_1 = asm_ast_instruction.clone();
                    mov_1.comment.push_str(" ; FIXUP Visitor");
                    mov_1.instruction_type = AsmAstInstructionType::Mov;
                    mov_1.src = asm_ast_instruction.src.clone();
                    mov_1.dst = AsmAstOperand { operand_type: AsmAstOperandType::Reg(AsmAstReg::R10) };

                    //mov_1.dst = self.replace_pseudo_operand(&mut asm_ast_instruction.dst);
                    mov_1.src = self.replace_pseudo_operand(&mut asm_ast_instruction.src);

                    // println!("{:?}", &mov_1);

                    new_body.push(Box::new(mov_1));

                    // add
                    // https://www.felixcloutier.com/x86/add
                    // Adds the destination operand (first operand) and the source operand (second operand)
                    // and then stores the result in the destination operand.
                    asm_ast_instruction.src = AsmAstOperand { operand_type: AsmAstOperandType::Reg(AsmAstReg::R10) };
                    asm_ast_instruction.dst = self.replace_pseudo_operand(&mut asm_ast_instruction.dst);

                    // DEBUG
                    if self.debug {
                        println!("{}", asm_ast_instruction.clone());
                    }

                    // DEBUG
                    println!("[AsmFixupVisitor] asm_ast_instruction: {:?}", &asm_ast_instruction);

                    new_body.push(Box::new(asm_ast_instruction));

                } else {

                    // TODO: Honolulu
                    asm_ast_instruction.dst = self.replace_pseudo_operand(&mut asm_ast_instruction.dst);

                    if asm_ast_instruction.src.operand_type != AsmAstOperandType::Unknown {
                        asm_ast_instruction.src = self.replace_pseudo_operand(&mut asm_ast_instruction.src);
                    }

                    // DEBUG
                    println!("[AsmFixupVisitor] asm_ast_instruction: {:?}", &asm_ast_instruction);

                    new_body.push(Box::new(asm_ast_instruction));
                }
            }

            AsmAstInstructionType::Binary => {
                // DEBUG
                if self.debug {
                    println!("[FixupVisitor] Binary {:?} {:?} {:?}",
                        asm_ast_instruction.binary_operator, asm_ast_instruction.src_2, asm_ast_instruction.dst);
                }

                asm_ast_instruction.src_2 = self.replace_pseudo_operand(&mut asm_ast_instruction.src_2);
                asm_ast_instruction.dst = self.replace_pseudo_operand(&mut asm_ast_instruction.dst);

                match asm_ast_instruction.binary_operator {

                    AsmAstBinaryOperator::Add | AsmAstBinaryOperator::Subtract => {
                        if self.replace_pseudo {
                            self.untangle_mem_mem(&mut asm_ast_instruction, new_body, AsmAstReg::R10);
                        } else {
                            new_body.push(Box::new(asm_ast_instruction));
                        }
                    }

                    AsmAstBinaryOperator::Multiply => {
                        panic!();
                    }

                    AsmAstBinaryOperator::LeftShift => {
                        if self.replace_pseudo {
                            // if the shift amount is not an immediate value,
                            // shift left is only able to use the CL register!
                            // None of the other registers are allowed!
                            self.untangle_mem_mem(&mut asm_ast_instruction, new_body, AsmAstReg::CX);
                        } else {
                            new_body.push(Box::new(asm_ast_instruction));
                        }
                    }

                    AsmAstBinaryOperator::RightShift => {
                        // just forward, do not make any changes
                        new_body.push(Box::new(asm_ast_instruction));
                    }

                    _ => {
                        panic!("Unhandled Instruction: {:?}", asm_ast_instruction);
                    }
                }
            }

            AsmAstInstructionType::Cdq => {
                // DEBUG
                if self.debug {
                    println!("Cdq");
                }

                new_body.push(Box::new(asm_ast_instruction));
            }

            AsmAstInstructionType::Div => {
                // DEBUG
                if self.debug {
                    println!("Div {:?}", asm_ast_instruction.dst);
                }

                asm_ast_instruction.dst = self.replace_pseudo_operand(&mut asm_ast_instruction.dst);

                new_body.push(Box::new(asm_ast_instruction));
            }

            AsmAstInstructionType::Mod => {
                // DEBUG
                if self.debug {
                    println!("Mod {:?}", asm_ast_instruction.dst);
                }

                asm_ast_instruction.dst = self.replace_pseudo_operand(&mut asm_ast_instruction.dst);

                new_body.push(Box::new(asm_ast_instruction));
            }

            AsmAstInstructionType::Mul => {
                // DEBUG
                if self.debug {
                    println!("Mul {:?}", asm_ast_instruction.dst);
                }

                //asm_ast_instruction.dst = self.replace_pseudo_operand(&mut asm_ast_instruction.dst);
                asm_ast_instruction.src = self.replace_pseudo_operand(&mut asm_ast_instruction.src);

                new_body.push(Box::new(asm_ast_instruction));
            }

            AsmAstInstructionType::Cmp => {
                // DEBUG
                if self.debug {
                    println!("Cmp {:?} {:?} {:?}",
                        asm_ast_instruction.src,
                        asm_ast_instruction.src_2,
                        asm_ast_instruction.dst);
                }

                asm_ast_instruction.src_2 = self.replace_pseudo_operand(&mut asm_ast_instruction.src_2);
                asm_ast_instruction.dst = self.replace_pseudo_operand(&mut asm_ast_instruction.dst);

                // does this instruction require a fix?
                // let mut fix: bool = false;

                // x86 cmp cannot compare memory (stack or other memory) to memory directly!
                if matches!(asm_ast_instruction.src_2.operand_type, AsmAstOperandType::Memory(_, _)) {
                    if matches!(asm_ast_instruction.dst.operand_type, AsmAstOperandType::Memory(_, _)) {

                        // insert a mov command to mitigate the situation

                        // clone the compare instruction into a move instruction
                        // The move, moves src_2 parameter into R10
                        let mut mov_1 = asm_ast_instruction.clone();
                        mov_1.comment.push_str(" - move for compare");
                        mov_1.instruction_type = AsmAstInstructionType::Mov;
                        mov_1.src = mov_1.src_2.clone();
                        mov_1.dst = AsmAstOperand { operand_type: AsmAstOperandType::Reg(AsmAstReg::R10) };
                        new_body.push(Box::new(mov_1));

                        // in the original cmp, parameter 2 becomes the register R10
                        asm_ast_instruction.src_2 = AsmAstOperand {
                            operand_type: AsmAstOperandType::Reg(AsmAstReg::R10) };
                        new_body.push(Box::new(asm_ast_instruction));
                    } else {
                        new_body.push(Box::new(asm_ast_instruction));
                    }
                } else if matches!(asm_ast_instruction.src_2.operand_type, AsmAstOperandType::Imm(_)) {
                    if matches!(asm_ast_instruction.dst.operand_type, AsmAstOperandType::Memory(_, _)) {
                        let temp = asm_ast_instruction.src_2;
                        asm_ast_instruction.src_2 = asm_ast_instruction.dst;
                        asm_ast_instruction.dst = temp;

                        new_body.push(Box::new(asm_ast_instruction));
                    } else {
                        new_body.push(Box::new(asm_ast_instruction));
                    }
                } else {
                    new_body.push(Box::new(asm_ast_instruction));
                }
            }

            AsmAstInstructionType::Jmp => {
                // DEBUG
                if self.debug {
                    println!("Jmp {:?} {:?} {:?}", asm_ast_instruction.src, asm_ast_instruction.src_2, asm_ast_instruction.dst);
                }

                new_body.push(Box::new(asm_ast_instruction));
            }

            AsmAstInstructionType::JmpCC => {
                // DEBUG
                if self.debug {
                    println!("JmpCC {:?} {:?}", asm_ast_instruction.src, asm_ast_instruction.src_2);
                }

                new_body.push(Box::new(asm_ast_instruction));
            }

            AsmAstInstructionType::Label => {
                // DEBUG
                if self.debug {
                    println!("Label {:?}", asm_ast_instruction.src);
                }

                new_body.push(Box::new(asm_ast_instruction));
            }

            AsmAstInstructionType::SetCC => {
                // DEBUG
                if self.debug {
                    println!("SetCC {:?} {:?}", asm_ast_instruction.src, asm_ast_instruction.dst);
                }

                asm_ast_instruction.dst = self.replace_pseudo_operand(&mut asm_ast_instruction.dst);

                new_body.push(Box::new(asm_ast_instruction));
            }

            AsmAstInstructionType::Lea => {
                // DEBUG
                if self.debug {
                    println!("Lea {:?} {:?}", asm_ast_instruction.src, asm_ast_instruction.dst);
                }

                asm_ast_instruction.src = self.replace_pseudo_operand(&mut asm_ast_instruction.src);
                asm_ast_instruction.dst = self.replace_pseudo_operand(&mut asm_ast_instruction.dst);

                // asm_ast_instruction.dst = self.replace_pseudo_operand(&mut asm_ast_instruction.dst);
                new_body.push(Box::new(asm_ast_instruction));
            }

            AsmAstInstructionType::Xor => {
                // DEBUG
                if self.debug {
                    println!("Xor {:?} {:?}", asm_ast_instruction.src, asm_ast_instruction.dst);
                }

                asm_ast_instruction.src = self.replace_pseudo_operand(&mut asm_ast_instruction.src);
                asm_ast_instruction.dst = self.replace_pseudo_operand(&mut asm_ast_instruction.dst);

                // asm_ast_instruction.dst = self.replace_pseudo_operand(&mut asm_ast_instruction.dst);
                new_body.push(Box::new(asm_ast_instruction));
            }

            AsmAstInstructionType::VariableDeclaration => {

                let variable_name = asm_ast_instruction.identifier;

                if self.stack_offset_map.contains_key(&variable_name) {

                } else {


                }
            }

            AsmAstInstructionType::AllocateStack => {
                // DEBUG
                if self.debug {
                    println!("AllocateStack({:?})", asm_ast_instruction.src);
                    println!("{:?}", asm_ast_instruction);
                }

                // // debug, prevent zero local variables!
                // match asm_ast_instruction.src.operand_type {
                //     AsmAstOperandType::Imm(value) => {
                //         assert_ne!(value, 0)
                //     }
                //     _ => {
                //         panic!();
                //     }
                // }

                new_body.push(Box::new(asm_ast_instruction));
            }

            AsmAstInstructionType::DeallocateStack => {
                // DEBUG
                if self.debug {
                    println!("DeallocateStack({:?})", asm_ast_instruction.src);
                    println!("{:?}", asm_ast_instruction);
                }

                new_body.push(Box::new(asm_ast_instruction));
            }

            AsmAstInstructionType::Push => {
                // DEBUG
                if self.debug {
                    println!("Push {:?}", asm_ast_instruction.dst);
                    println!("{:?}", asm_ast_instruction);
                }
                new_body.push(Box::new(asm_ast_instruction));
            }

            AsmAstInstructionType::FunctionCall => {
                // DEBUG
                if self.debug {
                    println!("FunctionCall {:?}", asm_ast_instruction.dst);
                    println!("{:?}", asm_ast_instruction);
                }
                new_body.push(Box::new(asm_ast_instruction));
            }

            AsmAstInstructionType::ShiftRightArithmetic => {
                // // DEBUG
                // if self.debug {
                //     println!("ShiftRightArithmetic {:?}", asm_ast_instruction.dst);
                //     println!("{:?}", asm_ast_instruction);
                // }
                new_body.push(Box::new(asm_ast_instruction));
            }

            AsmAstInstructionType::GlobalConstantDeclaration => {
                new_body.push(Box::new(asm_ast_instruction));
            }

            AsmAstInstructionType::InlineAssembly => {
                new_body.push(Box::new(asm_ast_instruction));
            }

        }
    }
}