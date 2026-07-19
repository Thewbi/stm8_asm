# Translating commands to assembly

## new_local_variable

### Example
```
new_local_variable("tmp.0")
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

## Binary Op

### Example
```
binary_op('*', "tmp.0", const(2), const(3)) // op, dest, lhs, rhs
binary_op('+', "tmp.0", const(2), const(3)) // op, dest, lhs, rhs
```

### Compiler
- retrieve "tmp.0" from the current scope / symbol table. (If "tmp.0" does not exist in the symbol table -> compiler error!)
- check type of const(2), const(3) and "tmp.0". Convert the types into each other!
- generate assembly using the location used for "tmp.0" stored in the symbol table


### Assembler
https://www.felixcloutier.com/x86/add

Assume that "tmp.0" was located to eax.

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
return 1 + 2 * 3;
```

### Compiler

The compiler sees *return &lt;expression&gt;;*

The compiler needs to first generate statements for *&lt;expression&gt;;*
It needs to know the location of where the result of *&lt;expression&gt;;* has been placed.
Let this location be *loc_expression*.

### Assembler

The return value needs to go into eax.
If loc_expression is not eax already, insert a mov to eax.

```
extern ExitProcess      ; need to include ExitProcess

... 

mov     eax, loc_expression

call    ExitProcess, eax     ; call ExitProcess
```



## Function Call

https://stackoverflow.com/questions/55773868/returning-a-value-in-x86-assembly-language

```
double: push ebp           ; establish...
        mov ebp, esp       ; ...stack frame

        mov eax, [ebp + 8] ; load argument from stack into eax
        add eax, eax       ; add it to itself

        leave              ; tear down the stack frame
        ret                ; return to the caller
```



# Higher Level Constructs

## Expression

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
new_local_variable("tmp.0")
binary_op('*', "tmp.0", const(2), const(3)) // op, dest, lhs, rhs
new_local_variable("tmp.1")
binary_op('+', "tmp.1", const(1), "tmp.0") // op, dest, lhs, rhs
return_op("tmp.1")
```





## For Loop

```
int result = 0;
for (int i = 0; i < 10; i++) {
    result = result + i;
}
```

```
define and assign result-variable               operation: new_local_variable("tmp.0"), assign_value("tmp.0", const(0))
initialize run-variable (i = 0)                 operation: new_local_variable("tmp.1"), assign_value("tmp.1", const(0))
start_label:                                    operation: new_label("start_label.0")
check predicate (i < 10)                        operation: execute the predicate/expression and evaluate it to a bool value
if predicate == false --> jump to end_label     operation: jump_if_lessthan("tmp.1", const(10), "end_label.0")
    {                                           operation: push_scope() // create a new scope for the compound statement
        <compound_statement> // result = result + i;
                                                operation: binary_op('+', "tmp.0", "tmp.0", "tmp.1") // op, dest, lhs, rhs
                                                operation: assign_value("tmp.0", "tmp.2")
    }                                           operation: pop_scope() // delete the scope
increment run-variable (i++)                    operation: inc("tmp.1")
jump to start_label                             operation: jump("start_label.0")
end_label:                                      operation: new_label("end_label.0")
```