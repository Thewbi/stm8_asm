






# Higher Level Constructs




## variable declaration w/o initialization

```
int lower;              operation:  new_local_variable("tmp.0", int32) 
                                    + insert it into the scope (= Add variable into current stack frame format)
                                    Add mapping into scope ("lower == tmp.0")
                                   
```





## variable declaration with initialization

```
int lower = 0;          operation:  new_local_variable("tmp.0", int32) 
                                    + insert it into the scope (= Add variable into current stack frame format)
                                    Add mapping into scope ("lower == tmp.0")
                        operation: assign_value(var("tmp.0"), const(0))
```




## if statement

```
void main() {
    int i = 0;
    if (i < 10) 
    {
        i++;
    }
}
```


```
void main()                             operation: push_scope() // create a new scope
{
    int i = 0;                          operation: new_local_variable("tmp.0", int32) + insert it into the scope 
                                                   Add mapping into scope ("i == tmp.0")
                                        operation: assign_value(var("tmp.0"), const(0))

                                        operation: new_local_variable("tmp.1", bool)
    if (i < 10)                         operation: binary_compare(LessThan, var("tmp.1"), var("tmp.0"), const(10)) 
                                                   // op, dest, lhs, rhs
                                        operation: jump_if_zero("tmp.1", end_label.0)
    {                                   operation: push_scope() // create a new scope

        i++;                            operation: look up variable name for symbol "i" to "tmp.0"
                                                   unary(inc, var("tmp.0"), var("tmp.0")) // op, dest, src

    }                                   operation: pop_scope() // delete the scope                       

                                        operation: new_label("end_label.0")

}                                       operation: pop_scope() // delete the scope
```




## else-if statement

```
void main() {
    int i = 0;
    if (i < 10) 
    {
        i++;
    } 
    else if (i < 20) 
    {
        i++;
    }
}
```


```
void main()                             operation: push_scope() // create a new scope
{
    int i = 0;                          operation: new_local_variable("tmp.0", int32) + insert it into the scope 
                                                   Add mapping into scope ("i == tmp.0")
                                        operation: assign_value(var("tmp.0"), const(0))

                                        operation: new_local_variable("tmp.1", bool)
    if (i < 10)                         operation: binary_op(LessThan, var("tmp.1"), var("tmp.0"), const(10)) 
                                                   // op, dest, lhs, rhs
                                        operation: jump_if_zero("tmp.1", end_label.0)
    {                                   operation: push_scope() // create a new scope

        i++;                            operation: look up variable name for symbol "i" to "tmp.0"
                                                   unary(inc, var("tmp.0"), var("tmp.0")) // op, dest, src

    }                                   
                                        operation: pop_scope() // delete the scope

    else if (i < 20)                    operation: new_local_variable("tmp.2", bool)
                                        operation: binary_op(LessThan, var("tmp.2"), var("tmp.0"), const(20)) 
                                                   // op, dest, lhs, rhs
                                        operation: jump_if_zero("tmp.2", end_label.0)
    {                                   operation: push_scope() // create a new scope
        
        i++;                            operation: look up variable name for symbol "i" to "tmp.0"
                                                   unary(inc, var("tmp.0"), var("tmp.0")) // op, dest, src
                                                   
    }                                   operation: pop_scope() // delete the scope

                                        operation: new_label("end_label.0")

}                                       operation: pop_scope() // delete the scope
```





## Expression using constants (res/C/samples/expression_1.c)

```
int main() {
    return 1 + 2 * 3;
}
```

```
--- + ---
|       |
1     --*--
      |   |
      2   3
```

```
new_local_variable("tmp.0", int32)
binary_op(MUL, var("tmp.0"), const(2), const(3)) // op, dest, lhs, rhs

new_local_variable("tmp.1", int32)
binary_op(ADD, var("tmp.1"), const(1), var("tmp.0")) // op, dest, lhs, rhs

return_op(var("tmp.1"))
```

### Notes/Remarks

* When the visitor enters the compound statement (function body), it will insert a new scope/symbol table entry.
* For every binary-operator (here: MUL, ADD) the binary operator first causes the creation of a new temp variable, which needs to be inserted into the current scope. The temp variable is needed to store the intermediate result of the expression-tree.
* The temporary variable id is also stored in the AST node of the binary operator, so the next binary operator is able to read the variable id from the subtree root node and is able to retrieve the location from the symbol table and to use that location in it's binary operation.
* The return statement will retrieve the temp variable id from the expression node, resolve the id to a location using the symbol table and return that location.
* On exiting the function, a stackframe needs to be constructed for all the temporary variables used in the function block. Nested blocks are ignored ???

### ParseTree

```
[Parser::consume()] REDUCING RULE: [250] type_specifier -> INT
[Parser::consume()] REDUCING RULE: [244] declaration_specifiers -> type_specifier
[Parser::consume()] REDUCING RULE: [366] direct_declarator -> IDENTIFIER
TerminalValue: 'main'
[Parser::consume()] REDUCING RULE: [1141] direct_declarator -> direct_declarator OPENING_BRACKET CLOSING_BRACKET
[Parser::consume()] REDUCING RULE: [354] declarator -> direct_declarator
[Parser::consume()] REDUCING RULE: [712] primary_expression -> NUMERIC
TerminalValue: '1'
[Parser::consume()] REDUCING RULE: [702] postfix_expression -> primary_expression
[Parser::consume()] REDUCING RULE: [687] unary_expression -> postfix_expression
[Parser::consume()] REDUCING RULE: [685] cast_expression -> unary_expression
[Parser::consume()] REDUCING RULE: [742] multiplicative_expression -> cast_expression
[Parser::consume()] REDUCING RULE: [712] primary_expression -> NUMERIC
TerminalValue: '2'
[Parser::consume()] REDUCING RULE: [702] postfix_expression -> primary_expression
[Parser::consume()] REDUCING RULE: [687] unary_expression -> postfix_expression
[Parser::consume()] REDUCING RULE: [1079] cast_expression -> unary_expression
[Parser::consume()] REDUCING RULE: [712] primary_expression -> NUMERIC
TerminalValue: '3'
[Parser::consume()] REDUCING RULE: [702] postfix_expression -> primary_expression
[Parser::consume()] REDUCING RULE: [687] unary_expression -> postfix_expression
[Parser::consume()] REDUCING RULE: [1079] cast_expression -> unary_expression
[Parser::consume()] REDUCING RULE: [742] multiplicative_expression -> cast_expression
[Parser::consume()] REDUCING RULE: [5711] multiplicative_expression -> cast_expression ASTERISK multiplicative_expression
[Parser::consume()] REDUCING RULE: [738] additive_expression -> multiplicative_expression
[Parser::consume()] REDUCING RULE: [5576] additive_expression -> multiplicative_expression PLUS additive_expression
[Parser::consume()] REDUCING RULE: [735] shift_expression -> additive_expression
[Parser::consume()] REDUCING RULE: [732] relational_expression -> shift_expression
[Parser::consume()] REDUCING RULE: [727] equality_expression -> relational_expression
[Parser::consume()] REDUCING RULE: [724] and_expression -> equality_expression
[Parser::consume()] REDUCING RULE: [722] exclusive_or_expression -> and_expression
[Parser::consume()] REDUCING RULE: [720] inclusive_or_expression -> exclusive_or_expression
[Parser::consume()] REDUCING RULE: [718] logical_and_expression -> inclusive_or_expression
[Parser::consume()] REDUCING RULE: [710] logical_or_expression -> logical_and_expression
[Parser::consume()] REDUCING RULE: [701] conditional_expression -> logical_or_expression
[Parser::consume()] REDUCING RULE: [686] assignment_expression -> conditional_expression
[Parser::consume()] REDUCING RULE: [914] expression -> assignment_expression
[Parser::consume()] REDUCING RULE: [6834] jump_statement -> RETURN expression SEMICOLON
[Parser::consume()] REDUCING RULE: [875] statement -> jump_statement
[Parser::consume()] REDUCING RULE: [866] declaration_or_statement_list -> statement
[Parser::consume()] REDUCING RULE: [2053] compound_statement -> OPENING_SQUIGGLY_BRACKET declaration_or_statement_list CLOSING_SQUIGGLY_BRACKET
[Parser::consume()] REDUCING RULE: [478] function_definition -> declaration_specifiers declarator compound_statement
[Parser::consume()] REDUCING RULE: [237] external_declaration -> function_definition
[Parser::consume()] REDUCING RULE: [236] translation_unit -> external_declaration
[Parser::consume] ACCEPT !!!!
```

## Expression using variables

```
int main() {
    int a = 3;
    int b = 5;
    int c = 10;

    return a + b * c;
}
```

```
--- + ---
|       |
a     --*--
      |   |
      b   c
```

```
new_local_variable("tmp.0", int32)
assign_value(var("tmp.0"), const(3))

new_local_variable("tmp.1", int32)
assign_value(var("tmp.1"), const(5))

new_local_variable("tmp.2", int32)
assign_value(var("tmp.2"), const(10))

new_local_variable("tmp.3", int32)
binary_op(MUL, var("tmp.3"), var("tmp.1"), var("tmp.2")) // op, dest, lhs, rhs

new_local_variable("tmp.4", int32)
binary_op(ADD, var("tmp.4"), var("tmp.3"), var("tmp.0")) // op, dest, lhs, rhs

return_op(var("tmp.4"))
```

## For Loop

```
int main() {

    int result = 0;
    int i = 0;
    for (i = 3; i < 10; i++) {
        result = result + i;
    }

}
```


```
        <instructions for init>
Label(start)
        <instructions for condition>
        v = <result of condition>
        JumpIfZero(v, end_label)
        <instructions for body>
Label(continue_label)
        <instructions for post>
        Jump(start)
Label(end_label)
```

Notes: 
* continue jumps to Label(continue_label)
* break jumps to Label(end_label)


```
int main() {                                        operation: push_scope() // create a new scope

    define and assign result-variable               operation: new_local_variable("tmp.0", int32) + insert it into the scope 
                                                               Add mapping into scope ("result == tmp.0")
                                                    operation: assign_value(var("tmp.0"), const(0))

    define and assign run-variable (i = 0)          operation: new_local_variable("tmp.1", int32) + insert it into the scope
                                                               Add mapping into scope ("i == tmp.1")
                                                    operation: assign_value(var("tmp.1"), const(0))

    initialize run-variable (i = 3)                 operation: assign_value(var("tmp.1"), const(3))
                                                               look up variable name for symbol "i" to "tmp.1"
start_label:                                        operation: new_label("start_label.0")
    check predicate (i < 10)                        operation: execute the predicate/expression and evaluate it to a bool value
    if predicate == false --> jump to end_label     operation: jump_if_lessthan(var("tmp.1"), const(10), "end_label.0")
        {                                           operation: push_scope() // create a new scope for the compound statement
            <compound_statement> // result = result + i;
                                                    operation: binary_op(ADD, var("tmp.0"), var("tmp.0"), var("tmp.1")) // op, dest, lhs, rhs
                                                    operation: assign_value(var("tmp.0"), var("tmp.2"))
        }                                           operation: pop_scope() // delete the scope

continue_label:                                     operation: new_label("continue_label.0")
    increment run-variable (i++)                    operation: unary(inc, var("tmp.1"), var("tmp.1")) // op, dest, src
    jump to start_label                             operation: jump("start_label.0")
end_label:                                          operation: new_label("end_label.0")

}                                                   operation: pop_scope() // delete the scope
```








## While Loop

```
Label(start_label)
        <instructions for condition>
        v = <result of condition>
        JumpIfZero(v, end_label)
        <instructions for body>
        Jump(start_label)
Label(end_label)
```

Notes: 
* continue jumps to Label(start_label)
* break jumps to Label(end_label)









# Translating commands to assembly

This section is about how to translate the atomic statements into assembly.





## new_local_variable

### Example
```
new_local_variable("tmp.0", int32)
new_local_variable("tmp.0", bool)
```

### Compiler
- select a free register, if no register is free select a free memory address on the stack
- insert tmp.0 into current scope which acts as the symbol table. 
    - as type use int64
    - as location either use the free register or the memory address on the stack
    - prioritize callee saved registers (saved by the called function) 
       over caller saved register because they will be trashed by a called function
       if not saved.

### Assembler
    - N/A






## Unary op

unary_operator = Complement | Negate | Not | Inc | Dec

```
Unary(Complement, Constant(1), Var("tmp"))
Unary(Negate, Constant(1), Var("tmp"))
unary(Not, var("tmp.1"), var("tmp.1")) // op, dest, src
unary(Inc, var("tmp.1"), var("tmp.1")) // op, dest, src
unary(Dec, var("tmp.1"), var("tmp.1")) // op, dest, src
```





## Binary Op

### Example
```
binary_op(MUL, var("tmp.0"), const(2), const(3)) // op, dest, lhs, rhs
binary_op(ADD, var("tmp.0"), const(2), const(3)) // op, dest, lhs, rhs
```

### Compiler
- retrieve "tmp.0" from the current scope / symbol table. (If "tmp.0" does not exist in the symbol table -> compiler error!)
- check type of const(2), const(3) and "tmp.0". Convert the types into each other!
- generate assembly using the location used for "tmp.0" stored in the symbol table


### Assembler
https://www.felixcloutier.com/x86/add

Assume that var("tmp.0") was located to eax.

```
mov eax, 2
add eax, 3
```

```
mov eax, 2
mul eax, 3
```





## Return Op

### Example

```
return_op(var("tmp.0"))
```

### Compiler

The compiler sees *return &lt;expression&gt;;*

The compiler needs to first generate statements for *&lt;expression&gt;;*
It needs to know the location of where the result of *&lt;expression&gt;;* has been placed.
Let this location be *loc_expression*. The temporary variable is called "tmp.0".

### Assembler

The return value needs to go into eax.
If loc_expression is not eax already, insert a mov to eax.

```
extern ExitProcess              ; need to include ExitProcess

... 

mov     eax, loc_expression

call    ExitProcess, eax        ; call ExitProcess
```




## Function Call

### Example (res\C\samples\c_samples\function_call_0.c)

```
int add(int x, int y);

void main() {
    int i = 0;
    i = add(1, 2);
}

int add(int x, int y) { 
    return x + y;
}
```

To call a function, the functions stack frame format has to be available.
The stack frame format says, which parameter goes into which address of the stack frame
and also which local variable of the function's scope goes into which address of the stack frame.

To call the function, 

1. the functions stack frame format is retrieved from some datastructure that maps the functions unique name to the stack frame format. 
1. A stack frame is pushed on to the stack
1. Then the actual parameter values are pushed onto the current stack frame at the right addresses. 
1. Then the functions body compound statement is executed.
1. Once the body is done, the stack frame is removed from the stack.


https://stackoverflow.com/questions/55773868/returning-a-value-in-x86-assembly-language

```
double: push ebp           ; establish...
        mov ebp, esp       ; ...stack frame

        mov eax, [ebp + 8] ; load argument from stack into eax
        add eax, eax       ; add it to itself

        leave              ; tear down the stack frame
        ret                ; return to the caller
```
