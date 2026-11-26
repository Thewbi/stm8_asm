# The Intermediate Language Step - TACKY

An intermediate language allows to split the compiler into a frontend for each programming language and a backend for each target.architecture.

It is overkill for compilers that translate a single source programming language for a single target. In this case the idea is to translate the C programming language to several target architectures (x86, RISC-V, 6502 and STM-8).

In the book, three address code is introduced on page 35. TACKY is defined on page 36.

Each chapter defines new TACKY constructs and also contains a table mapping AST nodes to TACKY elements.

TACKY is then converted into an Assembly AST (!= the AST, but a new assembly AST!). The assembly AST is then converted to the target assembler program code. This is stated on page 39.

## A word about storing TACKY to files

One problem with Nora Sandler's TACKY is that it does not store type information. This means that you cannot persist the TACKY IR into a file and read it back from the file since the type information is not output to TACKY but lives in RAM and is forwarded to subsequent compiler stages along with the TACKY IR code at runtime!

This means TACKY is not really a language that can be output to files. Instead TACKY is designed to live in RAM at compile time! If you want to persist TACKY to disc, you also need to perists and then deserialize the type information later!

Another issue is with Arrays. TACKY has no way of storing the amount of elements in an array. The amount of elements is also part of the type information which TACKY does not make explicit. This means you cannot read TACKY back from a file and detect arrays! Arrays are normal variables but there is no information in the TACKY syntax that the variable is an array and how many elements the array has. (https://github.com/Thewbi/cpp_compiler/blob/main/src/test/resources/TACKY/array_1.tky therefor introduces custom TACKY constructs that are not defined in Nora Sandler's book.)

## Open Questions:
unions,
sizeof,



## Page 36 - Unary Operations

The TACKY definition is contained on page 36.

```
program = Program(function_definition)

function_definition = Function(identifier, instruction* body)

instruction =
    Return(val)
    | Unary(unary_operator, val src, val dst)

val = Constant(int)
    | Var(identifier)

unary_operator =
    Complement
    | Negate
```

The AST -> TACKY conversion table is contained on page 37.
The TACKY -> ASM_AST converstion table is contained on page 41.
The ASM_AST -> ASM conversion table is contained on page 43, 44.



## Page 58 - Binary Operations

The TACKY definition is contained on page 58.

```
program = Program(function_definition)

function_definition = Function(identifier, instruction* body)

instruction =
    Return(val)
    | Unary(unary_operator, val src, val dst)
    | Binary(binary_operator, val src1, val src2, val dst)

val = Constant(int)
    | Var(identifier)

unary_operator =
    Complement
    | Negate

binary_operator =
    Add
    | Subtract
    | Multiply
    | Divide
    | Remainder
```

The AST -> TACKY conversion table is contained on page 58.
The TACKY -> ASM_AST converstion table is contained on page 62.
The ASM_AST -> ASM conversion table is contained on page 63.



## Page 75 - Logical and Relational Operators

The TACKY definition is contained on page 75.

```
program = Program(function_definition)

function_definition = Function(identifier, instruction* body)

instruction =
    Return(val)
    | Unary(unary_operator, val src, val dst)
    | Binary(binary_operator, val src1, val src2, val dst)
    | Copy(val src, val dst)
    | Jump(identifier target)
    | JumpIfZero(val condition, identifier target)
    | JumpIfNotZero(val condition, identifier target)
    | Label(identifier)

val = Constant(int)
    | Var(identifier)

unary_operator =
    Complement
    | Negate
    | Not

binary_operator =
    Add
    | Subtract
    | Multiply
    | Divide
    | Remainder
    | Equal
    | NotEqual
    | LessThan
    | LessOrEqual
    | GreaterThan
    | GreaterOrEqual
```

The AST -> TACKY conversion table is contained on page 76.
The TACKY -> ASM_AST converstion table is contained on page 85, 87.
The ASM_AST -> ASM conversion table is contained on page 89.



## Page 109 - Local Variables

There are no new TACKY definitions.
The AST -> TACKY conversion table is contained on page 110.
The TACKY -> ASM_AST converstion table is contained on page ?.
The ASM_AST -> ASM conversion table is contained on page ?.



## Page 126 - If Statements and Conditional Expressions

The TACKY definition is contained on page -- No new elements are introduced.

The AST -> TACKY conversion table is contained on page 126.
Templates to show how to construct TACKY are added.

AST:
```
if '(' <condition> ')' then <statement>
```

TACKY-Template:
```
<instructions for condition>
c = <result of condition> // There is no TACKY definition for an assignment! How is this assignment defined?
JumpIfZero(c, end)
<instructions for statement>
Label(end)
```

AST:
```
if '(' <condition> ')' then <statement_1> else <statement_2>
```

TACKY-Template:
```
<instructions for condition>
c = <result of condition> // There is no TACKY definition for an assignment! How is this assignment defined?
JumpIfZero(c, else_label)
<instructions for statement_1>
Jump(end)
Label(else_label)
<instructions for statement_2>
Label(end)
```

AST:
```
<condition> ? <e_1> else <e_2>
```

TACKY-Template:
```
<instructions for condition>
c = <result of condition>   // There is no TACKY definition for an assignment! How is this assignment defined?
JumpIfZero(c, e2_label)
<instructions to calculate e_1>
v1 = <result of e1>         // There is no TACKY definition for an assignment! How is this assignment defined?
result = v1                 // There is no TACKY definition for an assignment! How is this assignment defined?
Jump(end)
Label(e2_label)
<instructions to calculate e_2>
v2 = <result of e2>         // There is no TACKY definition for an assignment! How is this assignment defined?
result = v2                 // There is no TACKY definition for an assignment! How is this assignment defined?
Label(end)
```

The TACKY -> ASM_AST converstion table is contained on page ?.
The ASM_AST -> ASM conversion table is contained on page ?.



## Page 140 - Compound Statements

The TACKY definition is contained on page ? -- No new elements are introduced.

The AST -> TACKY conversion table is contained on page ?.

The book says on page 140

```
To convert a compound statement to TACKY, just convert each block item inside it to TACKY.
```

The TACKY -> ASM_AST converstion table is contained on page ?.
The ASM_AST -> ASM conversion table is contained on page ?.



## Page 155 - Loops

The TACKY definition is contained on page ? -- No new elements are introduced.

The AST -> TACKY conversion table is contained on page 155.

Assumption: Loop Annotations from page 152 has been successfully performed.

AST:
```
do <body> while '(' <condition> ')'
```

TACKY-Template:
```
Label(start)
    -- body
    <instructions for body>
Label<continue_label>
    -- condition
    <instructions for condition>
    v = <result of condition>       // There is no TACKY definition for an assignment! How is this assignment defined?
    JumpIfNotZero(v, start)
Label<break_label>
```



AST:
```
while '(' <condition> ')' <body>
```

TACKY-Template:
```
Label<continue_label>

    -- condition
    <instructions for condition>
    v = <result of condition>       // There is no TACKY definition for an assignment! How is this assignment defined?
    JumpIfNotZero(v, break_label)

    -- body
    <instructions for body>
    Jump(continue_label)

Label<break_label>
```



AST:
```
for '(' <init> ; <condition> ; <post> ')' <body>
```

TACKY-Template:
```
    -- init
    <instructions for init>

Label(start)

    -- condition
    <instructions for condition>
    v = <result of condition>       // There is no TACKY definition for an assignment! How is this assignment defined?
    JumpIfNotZero(v, break_label)

    -- body
    <instructions for body>

Label<continue_label>

    -- post
    <instructions for post>

    Jump(start)

Label<break_label>
```



Switch on page 159
AST:
```
switch (test_val) {
    case 1:
        <body_1>
        break;

    case 2:
        <body_2>
        break;

    case 3:
    case 4:
        <body_3_4>
        break;

    default:
        <body_default>
        break;
}
```

TACKY-Template:
```
    -- condition case 1
    Binary(Equal, test_val, 1, v)
    JumpIfZero(v, label_body_1)

    -- condition case 2
    Binary(Equal, test_val, 2, v)
    JumpIfZero(v, label_body_2)

    -- condition case 3
    Binary(Equal, test_val, 3, v)
    JumpIfZero(v, label_body_3_4)

    -- condition case 4
    Binary(Equal, test_val, 4, v)
    JumpIfZero(v, label_body_3_4)

    Jump(label_default)

Label<label_body_1>
    <instructions for body_1>
    Jump(break_label)

Label<label_body_2>
    <instructions for body_2>
    Jump(break_label)

Label<label_body_3_4>
    <instructions for body_3_4>
    Jump(break_label)

Label<label_default>
    <instructions for body_default>
    Jump(break_label)

Label<break_label>
```

The TACKY -> ASM_AST converstion table is contained on page ?.
The ASM_AST -> ASM conversion table is contained on page ?.



## Page ? - Function Calls

The TACKY definition is contained on page 182.

```
program = Program(function_definition*)

function_definition = Function(identifier, identifier* params, instruction* body)

instruction =
    Return(val)
    | Unary(unary_operator, val src, val dst)
    | Binary(binary_operator, val src1, val src2, val dst)
    | Copy(val src, val dst)
    | Jump(identifier target)
    | JumpIfZero(val condition, identifier target)
    | JumpIfNotZero(val condition, identifier target)
    | Label(identifier)
    | FunCall(identifier fun_name, val* args, val dst)
```

The AST -> TACKY conversion table is contained on page 183.

First convert every function_definition with body found in the AST into TACKY function_definition nodes.
The book says:
```
To convert an entire program to TACKY, process the top-level function declarations one at a time, converting each function definition to a TACKY function_definition and discarding any declaration without a body.
```

To convert function calls to TACKY:
```
To convert a function call to TACKY, generate the instructions to evaluate each argument and construct a list of the resulting TACKY values.
...
Remember to add a Return(0) instruction to the end of every function.
```
In case some paths do not contain a return or the user forgot to add a return.
Also read page 261 - "Generating Extra Return Instructions" to read about why return(0) is okay even if the function does not return an integer.

```
<instructions for e1>
v1 = <result of e1>

<instructions for e2>
v2 = <result of e2>

result = FunCall(fun, [v1, v2, ...])
```

The TACKY -> ASM_AST converstion table is contained on page ?.
The ASM_AST -> ASM conversion table is contained on page ?.



## Page 234 - File Scope Variable Declarations and Storage-Class Specifiers

The TACKY definition is contained on page 234.

```
program = Program(top_level*)

top_level =
    Function(identifier, bool global, identifier* params, instruction* body)
    | StaticVariable(identifier, bool global, int init)
```

The AST -> TACKY conversion table is contained on page ?.
The TACKY -> ASM_AST converstion table is contained on page 235, 236.
The ASM_AST -> ASM conversion table is contained on page ?.



## Page ? - Long Integers

The TACKY definition is contained on page 258, 259.

```
StaticVariable(identifier, bool global, type t, static_init init)

instruction =
    Return(val)
    | SignExtend(val src, val dst)
    | Truncate(val src, val dst)
    | Unary(unary_operator, val src, val dst)
    | Binary(binary_operator, val src1, val src2, val dst)
    | Copy(val src, val dst)
    | Jump(identifier target)
    | JumpIfZero(val condition, identifier target)
    | JumpIfNotZero(val condition, identifier target)
    | Label(identifier)
    | FunCall(identifier fun_name, val* args, val dst)

val = Constant(const)
    | Var(identifier)
```

The AST -> TACKY conversion table is contained on page 259, 260.
The TACKY -> ASM_AST converstion table is contained on page 261 and page 264, 265.
The ASM_AST -> ASM conversion table is contained on page ?.



## Page ? - Unsigned Integers

The TACKY definition is contained on page 281 and page 289.

```
instruction =
    Return(val)
    | SignExtend(val src, val dst)
    | Truncate(val src, val dst)
    | ZeroExtend(val src, val dst)
    | Unary(unary_operator, val src, val dst)
    | Binary(binary_operator, val src1, val src2, val dst)
    | Copy(val src, val dst)
    | Jump(identifier target)
    | JumpIfZero(val condition, identifier target)
    | JumpIfNotZero(val condition, identifier target)
    | Label(identifier)
    | FunCall(identifier fun_name, val* args, val dst)

const = ConstInt(int)
    | ConstUInt(int)
    | ConstLong(int)
    | ConstULong(int)
```

The AST -> TACKY conversion table is contained on page ?.
The TACKY -> ASM_AST converstion table is contained on page ?.
The ASM_AST -> ASM conversion table is contained on page ?.



## Page ? - Floating Point Numbers

The TACKY definition is contained on page 309.

```
instruction =
    Return(val)
    | SignExtend(val src, val dst)
    | Truncate(val src, val dst)
    | ZeroExtend(val src, val dst)
    | DoubleToInt(val src, val dst)
    | DoubleToUInt(val src, val dst)
    | IntToDouble(val src, val dst)
    | UIntToDouble(val src, val dst)
    | Unary(unary_operator, val src, val dst)
    | Binary(binary_operator, val src1, val src2, val dst)
    | Copy(val src, val dst)
    | Jump(identifier target)
    | JumpIfZero(val condition, identifier target)
    | JumpIfNotZero(val condition, identifier target)
    | Label(identifier)
    | FunCall(identifier fun_name, val* args, val dst)
```

The AST -> TACKY conversion table is contained on page ?.
The TACKY -> ASM_AST converstion table is contained on page ?.
The ASM_AST -> ASM conversion table is contained on page ?.



## Page ? - Pointers

The TACKY definition is contained on page 371.

```
instruction =
    Return(val)
    | SignExtend(val src, val dst)
    | Truncate(val src, val dst)
    | ZeroExtend(val src, val dst)
    | DoubleToInt(val src, val dst)
    | DoubleToUInt(val src, val dst)
    | IntToDouble(val src, val dst)
    | UIntToDouble(val src, val dst)
    | Unary(unary_operator, val src, val dst)
    | Binary(binary_operator, val src1, val src2, val dst)
    | Copy(val src, val dst)
    | GetAddress(val src, val dst)
    | Load(val src_ptr, val dst)
    | Store(val src, val dst_ptr)
    | Jump(identifier target)
    | JumpIfZero(val condition, identifier target)
    | JumpIfNotZero(val condition, identifier target)
    | Label(identifier)
    | FunCall(identifier fun_name, val* args, val dst)
```

PlainOperand is defined on page 372.

```
exp_result = PlainOperand(val) | DereferencedPointer(ptr)
```

Used on page 373 in listing 14-23

QUESTION: The symbol exp_result, PlainOperand, DereferencedPointer
never appear in any overview of the TACKY definitions in the book!
It is as if they are defined but never really displayed in full context!
I do not know what is going on.

They might be part of the implementation code and not part of TACKY.
PlainOperand(val) is returned by emit_tacky() for non-pointers.
DereferencedPointer(ptr) is returned by emit_tacky() for pointers.
Based on the type that emit_tacky() returns, the pointer is either assign to, lvalue converted or it's address is retrieved
which are the three cases that Nora Sandler lists on page 371 "It's tricky to convert ..."

page 372:
emit_tacky() will return exp_result which represents an expression result that has not been lvalue converted.
emit_tacky_and_convert() will [internally] call emit_tacky(), lvalue convert the result (if it is an lvalue rather than a constant)
and return the converted result as TACKY.

The problem here is basically how the TACKY generator will handle
- unary expressions
- dereference expressions
- assignments

Because with pointers introduced, unary expression now comprise AddressOf() and Dereference().
Assignments now contain the case where the LHS is a dereferenced pointer.

### The AST -> TACKY conversion table/ruleset is contained on page 370.

The AddrOf AstNode is mapped to the GetAddress() Tacky element.

Page 384 - Objects and Values

values - are sequences of bits with a type.

objects - have an address and a value. Stored in a memory cell. If lvalue conversion is applied, the object's value is returned. (pointer variables are objects too with a type of pointer to xyz).
The & operator returns the object's address. The & operator does not produce objects but turns objects into values of pointer type. The value of pointer type being the object's address. So the & operator turns objects into values.
Similarly to the & operator, lvalue-conversion converts a object to it's value. lvalue-conversion has no explicit operator in C! lvalue-conversion is applied implicitly.
The dereference operator * turns an address into a value???

lvalue - is an expression. Can evaluate to an object or to a ???

lvalues conversion - turns an lvalue into it's value.
(Page 348) "If an lvalue appears as the left operand of an assignment expression or as the operand of the & operator, it doesn`t undergo lvalue conversion. If it appears anywhere else in an expression, it does."

x is an lvalue here (since assignment or & operator)

```
x = 3;
y = &x;
```

x is not an lvalue here but it's value is used through lvalue-conversion. (1. x's value is used as a param, 2. equality comparison, 3. x's value is used as an RHS value of an assignment)

```
foo(x);
x == y;
a = x;
```

An expression can evaluate to an object and then either the value can be used or the object can be assigned a new value.

- in which case it's value can be used

```
... x + 1; // x being the expression. Here the value is used. This is called lvalue conversion. An object is converted to a value.
```

In TACKY, this turns into a Binary() instruction. In assembly, this turns into an ADD instruction.

- in which case a value can be assigned to the object.

```
x = 1; // x being the expression. Here a value is assigned to the object x. No lvalue conversion.
```

In TACKY, this turns into a Copy() instruction. In assembly this turns into a MOV instruction.

#### Converting the & addressof operator to TACKY

```
y = &x;
GetAddress(src:Variable(x), dst:Variable(y))
```

#### RHS Dereference - Converting Dereference operator to TACKY

The Dereference Expression is lvalue converted, meaning it is turned into a value.

```
y = *x;
Load(src_ptr:Variable(x_ptr), dst:Variable(y))
```

#### LHS Dereference - Converting value assignments to pointer to TACKY

The Dereference Expression is NOT lvalue converted but it evaluates to an object and a value is assigned to the object.

```
x_ptr = &x;
*x_ptr = 5;

Store(src:Constant(5), dst_ptr(x_ptr))
```

#### AddressOf on a Dereference of a pointer

```
*(&x) is the same as just using x. Here the value of x is returned in both cases.

&(*x_ptr) is the same as x_ptr. Here the lvalue-conversion of x_ptr which is an address of an object is returned in both cases.
```

The C standard says that in the case of the combining inverse operations, neither is evaluated but they are just dropped.


### The TACKY -> ASM_AST converstion table is contained on page ?.
### The ASM_AST -> ASM conversion table is contained on page ?.



## Page ? - Arrays and Pointer

The TACKY definition is contained on page 406, 407.

```
instruction =
    ...
    | AddPtr(val ptr, val index, int scale, val dst)
    | CopyToOffset(val src, identifier dst, int offset)
    ...
```

The AST -> TACKY conversion table is contained on page ?.
The TACKY -> ASM_AST converstion table is contained on page 416.
The ASM_AST -> ASM conversion table is contained on page ?.






## Page ? - Chapter 16 - Characters and Strings

The TACKY definition is contained on page ?.

```

```

The AST -> TACKY conversion table is contained on page ?.
The TACKY -> ASM_AST converstion table is contained on page ?.
The ASM_AST -> ASM conversion table is contained on page ?.



## Page 480 - (Not a full chapter but a sidenote on SizeOf(exp) and SizeOfT(type))

Page 480

"We'll evaluate  sizeof expressions during TACKY generation and represent the result as unsigned long constants ..."

No new TACKY elements are introduced for SizeOf and SizeOfT instead, compute the size of the variable or type and then
emit a constant for the size:

```
| SizeOf(inner) ->
    type = get_type(inner)
    result = size(type)
    return PlainOperand(Constant(ConstULong(result)))

| SizeOfT(type) ->
    result = size(type)
    return PlainOperand(Constant(ConstULong(result)))
```

I do not know what PlainOperand() is! It is not defined in TACKY!
PlainOperand is defined on page 372.

```
exp_result = PlainOperand(val) | DereferencedPointer(val)
```




## Page 487 - Supporting Dynamic Memory Allocation

The TACKY definition is contained on page 481.

```
program = Program(top_level*)

top_level =
    Function(identifier, bool global, identifier* params, instruction* body)
    | StaticVariable(identifier, bool global, type t, static_init init)
    | StaticConstant(identifier, type t, static_init init)

instruction =
    Return(val?)
    | SignExtend(val src, val dst)
    | Truncate(val src, val dst)
    | ZeroExtend(val src, val dst)
    | DoubleToInt(val src, val dst)
    | DoubleToUInt(val src, val dst)
    | IntToDouble(val src, val dst)
    | UIntToDouble(val src, val dst)
    | Unary(unary_operator, val src, val dst)
    | Binary(binary_operator, val src1, val src2, val dst)
    | Copy(val src, val dst)
    | GetAddress(val src, val dst)
    | Load(val src_ptr, val dst)
    | Store(val src, val dst_ptr)
    | AddPtr(val ptr, val index, int scale, val dst)
    | CopyToOffset(val src, identifier dst, int offset)
    | Jump(identifier target)
    | JumpIfZero(val condition, identifier target)
    | JumpIfNotZero(val condition, identifier target)
    | Label(identifier)
    | FunCall(identifier fun_name, val* args, val? dst)

val = Constant(const)
    | Var(identifier)

unary_operator =
    Complement
    | Negate
    | Not

binary_operator =
    Add
    | Subtract
    | Multiply
    | Divide
    | Remainder
    | Equal
    | NotEqual
    | LessThan
    | LessOrEqual
    | GreaterThan
    | GreaterOrEqual
```

The AST -> TACKY conversion table is contained on page ?.
The TACKY -> ASM_AST converstion table is contained on page ?.
The ASM_AST -> ASM conversion table is contained on page ?.

## Page 485 - Structures

The TACKY definition is contained on page 513.

```
instruction =
    ...
    | CopyToOffset(val src, identifier dst, int offset)
    | CopyFromOffset(identifier src, int offset, val dst)
    ...
```

The AST -> TACKY conversion table is contained on page ?.
The TACKY -> ASM_AST converstion table is contained on page ?.
The ASM_AST -> ASM conversion table is contained on page ?.

## Page ?

The TACKY definition is contained on page ?.
The AST -> TACKY conversion table is contained on page ?.
The TACKY -> ASM_AST converstion table is contained on page ?.
The ASM_AST -> ASM conversion table is contained on page ?.

## Page ?

The TACKY definition is contained on page ?.
The AST -> TACKY conversion table is contained on page ?.
The TACKY -> ASM_AST converstion table is contained on page ?.
The ASM_AST -> ASM conversion table is contained on page ?.

## Page ?

The TACKY definition is contained on page ?.
The AST -> TACKY conversion table is contained on page ?.
The TACKY -> ASM_AST converstion table is contained on page ?.
The ASM_AST -> ASM conversion table is contained on page ?.



## Final Version of TACKY Definitions

### Hint
```
exp_result = PlainOperand(val) | DereferencedPointer(val)
```
are still missing from the overview somehow!

### Full Definitions

```
program = Program(top_level*)

top_level =
    Function(identifier name, bool global, identifier* params, instruction* body)
    | StaticVariable(identifier name, bool global, type t, static_init init)
    | StaticConstant(identifier name, type t, static_init init)

static_init =
    | Constant(const)

type =
    <take this value from the type information data structure> (char, bool, int, short, float, double, ...)

instruction =
    Return(val?)
    | Unary(unary_operator op, val src, val dst)
    | Binary(binary_operator op, val src1, val src2, val dst)
    | Jump(identifier target)
    | JumpIfZero(val condition, identifier target)
    | JumpIfNotZero(val condition, identifier target)
    | Copy(val src, val dst)
    | GetAddress(val src, val dst)
    | Load(val src_ptr, val dst)
    | Store(val src, val dst_ptr)
    | AddPtr(val ptr, val index, int scale, val dst)
    | CopyToOffset(val src, identifier dst, int offset)
    | CopyFromOffset(identifier src, int offset, val dst)
    | Label(identifier name)
    | FunCall(identifier function_name, val* args, val? dst)
    | ZeroExtend(val src, val dst)
    | SignExtend(val src, val dst)
    | Truncate(val src, val dst)
    | IntToDouble(val src, val dst)
    | DoubleToInt(val src, val dst)
    | UIntToDouble(val src, val dst)
    | DoubleToUInt(val src, val dst)

val = Constant(const value)
    | Var(identifier name)

unary_operator =
    Complement
    | Negate
    | Not

binary_operator =
    Add
    | Subtract
    | Multiply
    | Divide
    | Remainder
    | Equal
    | NotEqual
    | LessThan
    | LessOrEqual
    | GreaterThan
    | GreaterOrEqual
```



# TACKY Part II

## Page 371