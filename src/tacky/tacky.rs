// program = Program(top_level*)

// top_level =
//     Function(identifier name, bool global, identifier* params, instruction* body)
//     | StaticVariable(identifier name, bool global, type t, static_init init)
//     | StaticConstant(identifier name, type t, static_init init)

// static_init =
//     | Constant(const)

// type =
//     <take this value from the type information data structure> (char, bool, int, short, float, double, ...)

// instruction =
//     Return(val?)
//     | Unary(unary_operator op, val src, val dst)
//     | Binary(binary_operator op, val src1, val src2, val dst)
//     | Jump(identifier target)
//     | JumpIfZero(val condition, identifier target)
//     | JumpIfNotZero(val condition, identifier target)
//     | Copy(val src, val dst)
//     | Load(val src_ptr, val dst)
//     | Store(val src, val dst_ptr)
//     | GetAddress(val src, val dst)
//     | AddPtr(val ptr, val index, int scale, val dst)
//     | CopyToOffset(val src, identifier dst, int offset)
//     | CopyFromOffset(identifier src, int offset, val dst)
//     | Label(identifier name)
//     | FunCall(identifier function_name, val* args, val? dst)
//     | ZeroExtend(val src, val dst)
//     | SignExtend(val src, val dst)
//     | Truncate(val src, val dst)
//     | IntToDouble(val src, val dst)
//     | DoubleToInt(val src, val dst)
//     | UIntToDouble(val src, val dst)
//     | DoubleToUInt(val src, val dst)

// val = Constant(const value)
//     | Var(identifier name)

// unary_operator =
//     Complement
//     | Negate
//     | Not

// binary_operator =
//     Add
//     | Subtract
//     | Multiply
//     | Divide
//     | Remainder
//     | Equal
//     | NotEqual
//     | LessThan
//     | LessOrEqual
//     | GreaterThan
//     | GreaterOrEqual

use std::fs::File;

use std::io::BufWriter;
use std::io::Write;

use std::fmt;
use std::fmt::Display;

use std::option::Option::None;

use crate::common::data_type::DataType;

pub struct Program {
    pub name: String,
    pub top_level: Vec<Box<TopLevel>>,
}

impl Program {
    pub fn new() -> Program {
        Program {
            name: String::from(""),
            top_level: Vec::<Box<TopLevel>>::new(),
        }
    }
}

#[derive(Debug)]
pub struct Argument {
    pub name: String,
    pub data_type: DataType,
}

impl Argument {
    pub fn new() -> Argument {
        Argument {
            name: String::from(""),
            data_type: DataType::DataTypeUnknown,
        }
    }
}

pub struct TopLevel {
    pub name: String,
    pub top_level_type: TopLevelType,
    pub global: bool,
    pub type_id: String,
    pub init: String,
    pub arguments: Vec::<Box<Argument>>,
    pub body: Vec<Box<Instruction>>,
    pub return_type: Option<DataType>,
}

impl TopLevel {
    pub fn new() -> TopLevel {
        TopLevel {
            name: String::from(""),
            top_level_type: TopLevelType::Function,
            global: false,
            type_id: String::from("function"),
            init: String::from("init"),
            arguments: Vec::<Box<Argument>>::new(),
            body: Vec::<Box<Instruction>>::new(),
            return_type: Option::None,
        }
    }
}

pub enum TopLevelType {
    Function,
    StaticVariable,
    StaticConstant,
}

#[derive(Debug)]
pub struct Instruction {
    pub instruction_type: InstructionType,
    pub src: ValueElement,
    pub src_2: ValueElement,
    pub dst: ValueElement,
    pub unary_operator: UnaryOperator,
    pub binary_operator: BinaryOperator,
    pub label: String,
    pub data_type: String,
    pub function_name: String,
    pub parameters: Vec::<Box<ValueElement>>,
    pub offset: i32,
    pub index: ValueElement,
    pub scale: u32,
}

impl Instruction {
    pub fn new() -> Instruction {
        Instruction {
            instruction_type: InstructionType::Return,
            src: ValueElement::None,
            src_2: ValueElement::None,
            dst: ValueElement::None,
            unary_operator: UnaryOperator::Not,
            binary_operator: BinaryOperator::Add,
            label: String::from(""),
            data_type: String::from(""),
            function_name: String::from(""),
            parameters: Vec::<Box<ValueElement>>::new(),
            offset: 0i32,
            index: ValueElement::None,
            scale: 0u32,
        }
    }
}

#[derive(Debug)]
pub enum InstructionType {
    Return,
    Unary,
    Binary,
    Jump,
    JumpIfZero,
    JumpIfNotZero,
    Copy,
    Load, // load from memory into variable
    Store, // store from variable into memory
    GetAddress,
    AddPtr,
    CopyToOffset,
    CopyFromOffset,
    Label,
    FunCall,
    ZeroExtend,
    SignExtend,
    Truncate,
    IntToDouble,
    DoubleToInt,
    UIntToDouble,
    DoubleToUInt,
    VariableDeclaration,

    Comment, // artifically added
}

#[derive(Debug, Clone)]
pub enum UnaryOperator {
    Complement,
    Negate,
    Not,
    Increment,
    Dereference,
    AddrOf,
}

#[derive(Debug, Clone)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Division,
    Remainder,
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,

    And,
    Or,
    Xor,
    LeftShift,
    RightShift,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum ValueElement {
    Constant(String),
    Variable(String),
    None,
}

impl fmt::Debug for ValueElement {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        match self {

            ValueElement::Constant(value) => {
                write!(f, "Constant({})", &value).expect("Write failed!");
            }
            ValueElement::Variable(value) => {
                write!(f, "Variable({})", &value).expect("Write failed!");
            }

            _ => {
                write!(f, "None").expect("Write failed!");
            }
        }

        Ok(())
    }
}

pub fn print_tacky_instruction(instruction: &Instruction, string_buffer: &mut String, indent: usize) {

    // indent
    let indent_string = std::iter::repeat(" ").take(indent * 2).collect::<String>();
    string_buffer.push_str(&indent_string);

    match &instruction.instruction_type {

        InstructionType::Comment => {
            string_buffer.push_str(format!("// {}\n", instruction.label).as_str());
        }

        InstructionType::Return => {
            string_buffer.push_str(format!("Return({:?})\n", instruction.src).as_str());
        }

        InstructionType::Unary => {
            string_buffer.push_str(format!("Unary({:?}, src:{:?}, dst:{:?})\n", instruction.unary_operator, instruction.src, instruction.dst).as_str());
        }

        InstructionType::Binary => {
            string_buffer.push_str(format!("Binary({:?}, src_1:{:?}, src_2:{:?}, dst:{:?})\n", instruction.binary_operator, instruction.src, instruction.src_2, instruction.dst).as_str());
        }

        InstructionType::Jump => {
            string_buffer.push_str(format!("Jump({})\n", instruction.label).as_str());
        }

        InstructionType::JumpIfZero => {
            string_buffer.push_str(format!("JumpIfZero({:?}, {})\n", instruction.src, instruction.label).as_str());
        }

        InstructionType::JumpIfNotZero => {
            string_buffer.push_str(format!("JumpIfNotZero({:?}, {})\n", instruction.src, instruction.label).as_str());
        }

        InstructionType::Copy => {
            string_buffer.push_str(format!("Copy(src:{:?}, dst:{:?})\n", instruction.src, instruction.dst).as_str());
        }

        InstructionType::Load => {
            string_buffer.push_str(format!("Load(src_ptr:{:?}, dst:{:?})\n", instruction.src, instruction.dst).as_str());
        }

        InstructionType::Store => {
            string_buffer.push_str(format!("Store(src:{:?}, dst_ptr:{:?})\n", instruction.src, instruction.dst).as_str());
        }

        InstructionType::GetAddress => {
            string_buffer.push_str(format!("GetAddress(src:{:?}, dst:{:?})\n", instruction.src, instruction.dst).as_str());
        }

        InstructionType::AddPtr => {
            // val ptr, val index, int scale, val dst
            string_buffer.push_str(format!("AddPtr(ptr:{:?}, index:{:?}, scale:{:?}, dst:{:?})\n", instruction.src, instruction.index, instruction.scale, instruction.dst).as_str());
        }

        InstructionType::CopyToOffset => {
            string_buffer.push_str(format!("CopyToOffset(src:{:?}, dst:{:?}, offset:{})\n", instruction.src, instruction.dst, instruction.offset).as_str());
        }

        InstructionType::CopyFromOffset => {
            string_buffer.push_str(format!("CopyFromOffset(src:{:?}, offset:{}, dst:{:?})\n", instruction.label, instruction.offset, instruction.dst).as_str());
        }

        InstructionType::Label => {
            string_buffer.push_str(format!("Label(\"{}\")\n", instruction.label).as_str());
        }

        InstructionType::FunCall => {
            string_buffer.push_str(format!("FunCall({:?}", instruction.function_name).as_str());

            for i in 0..instruction.parameters.len() {
                string_buffer.push_str(format!(", src:{:?}", instruction.parameters[i]).as_str());
            }

            if instruction.dst != ValueElement::None {
                string_buffer.push_str(format!(", dst:{:?}", instruction.dst).as_str());
            }

            string_buffer.push_str(format!(")\n").as_str());
        }

        InstructionType::ZeroExtend => {
            string_buffer.push_str(format!("ZeroExtend(src:{:?}, dst:{:?})\n", instruction.src, instruction.dst).as_str());
        }

        InstructionType::SignExtend => {
            string_buffer.push_str(format!("SignExtend(src:{:?}, dst:{:?})\n", instruction.src, instruction.dst).as_str());
        }

        InstructionType::Truncate => {
            string_buffer.push_str(format!("Truncate(src:{:?}, dst:{:?})\n", instruction.src, instruction.dst).as_str());
        }

        InstructionType::IntToDouble => {
            string_buffer.push_str(format!("IntToDouble(src:{:?}, dst:{:?})\n", instruction.src, instruction.dst).as_str());
        }

        InstructionType::DoubleToInt => {
            string_buffer.push_str(format!("DoubleToInt(src:{:?}, dst:{:?})\n", instruction.src, instruction.dst).as_str());
        }

        InstructionType::UIntToDouble => {
            string_buffer.push_str(format!("UIntToDouble(src:{:?}, dst:{:?})\n", instruction.src, instruction.dst).as_str());
        }

        InstructionType::DoubleToUInt => {
            string_buffer.push_str(format!("DoubleToUInt(src:{:?}, dst:{:?})\n", instruction.src, instruction.dst).as_str());
        }

        InstructionType::VariableDeclaration => {
            string_buffer.push_str(format!("VariableDeclaration(label:{:?}, data_type:{:?})\n", instruction.label, instruction.data_type).as_str());
        }

        // _ => {
        //     panic!("{}", format!("unknown instruction! {:?}", instruction.instruction_type));
        // }
    }
}

pub fn print_tacky_function(function: &TopLevel, string_buffer: &mut String, indent: usize) {

    // indent
    let indent_string = std::iter::repeat(" ").take(indent * 2).collect::<String>();
    string_buffer.push_str(&indent_string);

    // add function name and function global
    let mut function_string_buffer = String::new();
    function_string_buffer.push_str(format!("Function(\"{}\", {}", function.name, function.global).as_str());

    // add all format arguments (name and data type)
    for argument in function.arguments.iter() {
        function_string_buffer.push_str(format!(", Arg(\"{}\", {:?})", argument.name, argument.data_type).as_str());
    }

    function_string_buffer.push_str(")\n");

    string_buffer.push_str(&function_string_buffer);

    // print all statements in the function body
    for i in 0..function.body.len() {
        print_tacky_instruction(&function.body[i], string_buffer, indent + 1);
    }
}

pub fn print_tacky_top_level(top_level: &TopLevel, string_buffer: &mut String, indent: usize) {

    // indent
    let indent_string = std::iter::repeat(" ").take(indent * 2).collect::<String>();

    match top_level.top_level_type {

        TopLevelType::Function => {
            print_tacky_function(top_level, string_buffer, indent);
        }

        TopLevelType::StaticVariable => {
            string_buffer.push_str(&indent_string);
            string_buffer.push_str(format!("StaticVariable(\"{}\", {}, {})\n", top_level.name, top_level.global, top_level.init).as_str());
        }

        TopLevelType::StaticConstant => {
            string_buffer.push_str(&indent_string);
            string_buffer.push_str(format!("StaticConstant(\"{}\", {}, {})\n", top_level.name, top_level.type_id, top_level.init).as_str());
        }
    }
}

pub fn print_tacky_program(program: &Program, string_buffer: &mut String, indent: usize) {

    // indent
    let indent_string = std::iter::repeat(" ").take(indent * 2).collect::<String>();
    string_buffer.push_str(&indent_string);

    // add program
    string_buffer.push_str(format!("Program(\"{}\")\n", program.name).as_str());

    // print top-level
    for i in 0..program.top_level.len() {
        print_tacky_top_level(&program.top_level[i], string_buffer, indent + 1);
    }
}

// pub fn add(a: i32, b: i32) -> i32 {
//     a + b
// }

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    // #[test]
    // fn test_add() {
    //     assert_eq!(add(1, 2), 3);
    // }

    #[test]
    fn test_print() {

        // TopLevel Element - Static Variable
        let mut top_level_var: TopLevel = TopLevel::new();
        top_level_var.name = String::from("global_variable_1");
        top_level_var.top_level_type = TopLevelType::StaticVariable;
        top_level_var.global = true;
        top_level_var.init = String::from("0");

        // ZeroExtend(val src, val dst)
        let mut zero_extend_instruction: Instruction = Instruction::new();
        zero_extend_instruction.instruction_type = InstructionType::ZeroExtend;
        zero_extend_instruction.src = ValueElement::Variable(String::from("var.0"));
        zero_extend_instruction.dst = ValueElement::Variable(String::from("var.1"));

        // SignExtend(val src, val dst)
        let mut sign_extend_instruction: Instruction = Instruction::new();
        sign_extend_instruction.instruction_type = InstructionType::SignExtend;
        sign_extend_instruction.src = ValueElement::Variable(String::from("var.0"));
        sign_extend_instruction.dst = ValueElement::Variable(String::from("var.1"));

        // Truncate(val src, val dst)
        let mut truncate_instruction: Instruction = Instruction::new();
        truncate_instruction.instruction_type = InstructionType::Truncate;
        truncate_instruction.src = ValueElement::Variable(String::from("var.0"));
        truncate_instruction.dst = ValueElement::Variable(String::from("var.1"));

        // IntToDouble(val src, val dst)
        let mut int_to_double_instruction: Instruction = Instruction::new();
        int_to_double_instruction.instruction_type = InstructionType::IntToDouble;
        int_to_double_instruction.src = ValueElement::Variable(String::from("var.0"));
        int_to_double_instruction.dst = ValueElement::Variable(String::from("var.1"));

        // DoubleToInt(val src, val dst)
        let mut double_to_int_instruction: Instruction = Instruction::new();
        double_to_int_instruction.instruction_type = InstructionType::DoubleToInt;
        double_to_int_instruction.src = ValueElement::Variable(String::from("var.0"));
        double_to_int_instruction.dst = ValueElement::Variable(String::from("var.1"));

        // UIntToDouble(val src, val dst)
        let mut uint_to_double_instruction: Instruction = Instruction::new();
        uint_to_double_instruction.instruction_type = InstructionType::UIntToDouble;
        uint_to_double_instruction.src = ValueElement::Variable(String::from("var.0"));
        uint_to_double_instruction.dst = ValueElement::Variable(String::from("var.1"));

        // DoubleToUInt(val src, val dst)
        let mut double_to_uint_instruction: Instruction = Instruction::new();
        double_to_uint_instruction.instruction_type = InstructionType::DoubleToUInt;
        double_to_uint_instruction.src = ValueElement::Variable(String::from("var.0"));
        double_to_uint_instruction.dst = ValueElement::Variable(String::from("var.1"));

        // CopyToOffset
        let mut copy_to_offset_instruction: Instruction = Instruction::new();
        copy_to_offset_instruction.instruction_type = InstructionType::CopyToOffset;
        copy_to_offset_instruction.label = String::from("new_label");
        copy_to_offset_instruction.offset = 0x1234;
        copy_to_offset_instruction.dst = ValueElement::Variable(String::from("var.1"));

        // CopyFromOffset
        let mut copy_from_offset_instruction: Instruction = Instruction::new();
        copy_from_offset_instruction.instruction_type = InstructionType::CopyFromOffset;
        copy_from_offset_instruction.label = String::from("new_label");
        copy_from_offset_instruction.offset = 0x1234;
        copy_from_offset_instruction.dst = ValueElement::Variable(String::from("var.1"));

        // Label
        let mut label_instruction: Instruction = Instruction::new();
        label_instruction.instruction_type = InstructionType::Label;
        label_instruction.label = String::from("new_label");

        // FunCall - Function Call (!= Function Declaration)
        let mut function_call_instruction: Instruction = Instruction::new();
        function_call_instruction.instruction_type = InstructionType::FunCall;
        function_call_instruction.function_name = String::from("test_func");
        function_call_instruction.parameters.push(Box::new(ValueElement::Constant(String::from("var.0"))));
        function_call_instruction.parameters.push(Box::new(ValueElement::Constant(String::from("var.1"))));
        function_call_instruction.dst = ValueElement::Variable(String::from("var.1"));

        // GetAddress
        let mut get_address_instruction: Instruction = Instruction::new();
        get_address_instruction.instruction_type = InstructionType::GetAddress;
        get_address_instruction.src = ValueElement::Constant(String::from("1000"));
        get_address_instruction.dst = ValueElement::Variable(String::from("var.1"));

        // val ptr, val index, int scale, val dst
        let mut add_ptr_instruction: Instruction = Instruction::new();
        add_ptr_instruction.instruction_type = InstructionType::AddPtr;
        add_ptr_instruction.src = ValueElement::Variable(String::from("var.0")); // src field is used as ptr field
        add_ptr_instruction.index = ValueElement::Constant(String::from("0"));
        add_ptr_instruction.scale = 4u32;
        add_ptr_instruction.dst = ValueElement::Variable(String::from("var.1"));

        // Store
        let mut store_instruction: Instruction = Instruction::new();
        store_instruction.instruction_type = InstructionType::Store;
        store_instruction.src = ValueElement::Constant(String::from("var.0"));
        store_instruction.dst = ValueElement::Variable(String::from("var.1"));

        // Load
        let mut load_instruction: Instruction = Instruction::new();
        load_instruction.instruction_type = InstructionType::Load;
        load_instruction.src = ValueElement::Constant(String::from("var.0"));
        load_instruction.dst = ValueElement::Variable(String::from("var.1"));

        // Copy
        let mut copy_instruction: Instruction = Instruction::new();
        copy_instruction.instruction_type = InstructionType::Copy;
        copy_instruction.src = ValueElement::Constant(String::from("var.0"));
        copy_instruction.dst = ValueElement::Variable(String::from("var.1"));

        // JumpIfNotZero
        let mut jump_if_not_zero_instruction: Instruction = Instruction::new();
        jump_if_not_zero_instruction.instruction_type = InstructionType::JumpIfNotZero;
        jump_if_not_zero_instruction.src = ValueElement::Constant(String::from("1"));
        jump_if_not_zero_instruction.label = String::from("continue_label");

        // JumpIfZero
        let mut jump_if_zero_instruction: Instruction = Instruction::new();
        jump_if_zero_instruction.instruction_type = InstructionType::JumpIfZero;
        jump_if_zero_instruction.src = ValueElement::Constant(String::from("0"));
        jump_if_zero_instruction.label = String::from("continue_label");

        // Jump
        let mut jump_instruction: Instruction = Instruction::new();
        jump_instruction.instruction_type = InstructionType::Jump;
        jump_instruction.label = String::from("continue_label");

        // Binary operator
        let mut binary_instruction: Instruction = Instruction::new();
        binary_instruction.instruction_type = InstructionType::Binary;
        binary_instruction.binary_operator = BinaryOperator::Add;
        binary_instruction.src = ValueElement::Constant(String::from("0"));
        binary_instruction.src_2 = ValueElement::Constant(String::from("1"));
        binary_instruction.dst = ValueElement::Variable(String::from("var.2"));

        // Unary
        let mut unary_instruction: Instruction = Instruction::new();
        unary_instruction.instruction_type = InstructionType::Unary;
        unary_instruction.unary_operator = UnaryOperator::Complement;
        unary_instruction.src = ValueElement::Constant(String::from("0"));
        unary_instruction.dst = ValueElement::Variable(String::from("var.1"));

        // Return
        let mut return_instruction: Instruction = Instruction::new();
        return_instruction.instruction_type = InstructionType::Return;
        return_instruction.src = ValueElement::Variable(String::from("var.1"));

        // Function Declaration (!= Function Call) - TopLevel Element
        let mut top_level_function: TopLevel = TopLevel::new();
        top_level_function.name = String::from("function_1");
        top_level_function.top_level_type = TopLevelType::Function;
        top_level_function.global = true;

        // VariableDeclaration (Custom, Not define by Nora Sandler)
        let mut var_declaration: Instruction = Instruction::new();
        var_declaration.instruction_type = InstructionType::VariableDeclaration;
        var_declaration.label = String::from("var_name");
        var_declaration.data_type = String::from("int");





        top_level_function.body.push(Box::new(return_instruction));
        top_level_function.body.push(Box::new(unary_instruction));
        top_level_function.body.push(Box::new(binary_instruction));
        top_level_function.body.push(Box::new(jump_instruction));
        top_level_function.body.push(Box::new(jump_if_zero_instruction));
        top_level_function.body.push(Box::new(jump_if_not_zero_instruction));
        top_level_function.body.push(Box::new(copy_instruction));
        top_level_function.body.push(Box::new(load_instruction));
        top_level_function.body.push(Box::new(store_instruction));
        top_level_function.body.push(Box::new(get_address_instruction));
        top_level_function.body.push(Box::new(add_ptr_instruction));
        top_level_function.body.push(Box::new(copy_to_offset_instruction));
        top_level_function.body.push(Box::new(copy_from_offset_instruction));
        top_level_function.body.push(Box::new(label_instruction));
        top_level_function.body.push(Box::new(function_call_instruction));
        top_level_function.body.push(Box::new(zero_extend_instruction));
        top_level_function.body.push(Box::new(sign_extend_instruction));
        top_level_function.body.push(Box::new(truncate_instruction));
        top_level_function.body.push(Box::new(int_to_double_instruction));
        top_level_function.body.push(Box::new(double_to_int_instruction));
        top_level_function.body.push(Box::new(uint_to_double_instruction));
        top_level_function.body.push(Box::new(double_to_uint_instruction));
        top_level_function.body.push(Box::new(var_declaration));

        let mut program: Program = Program::new();
        program.name = String::from("program_1");
        program.top_level.push(Box::new(top_level_var));
        program.top_level.push(Box::new(top_level_function));

        let mut string_buffer = String::from("");
        let indent = 0usize;

        print_tacky_program(&program, &mut string_buffer, indent);

        // 1. Create or overwrite the file
        let file = File::create("tacky.tky").expect("Create file failed!");

        // 2. Wrap the file in a BufWriter
        let mut writer = BufWriter::new(file);

        // 3. Write data
        write!(writer, "{}", string_buffer);

        // 4. Explicitly flush the remaining data to disk
        writer.flush().expect("flush failed!");
    }
}