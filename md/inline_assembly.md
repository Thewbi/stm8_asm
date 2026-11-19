# Introduction

It is possible to insert assembly code into a .c file. This is called inline assembly.

The compiler will forward this inline assembly to the output assembly file, so it
is processed by the assembler along with the rest of the generated assembly.

## Lexer

The lexer is extended to accept two new token called ```INLINE_ASM_A``` which is ```__asm__``` and ```INLINE_ASM_B``` which is ```__asm```.

Both token are supposed to affect the parser and make the parser process inline assembly wrapped in squiggly brackets following both token.

``` c_lexer.rs
// GCC
    // __asm__(
    //    "movl %edx, %eax\n\t"
    //    "addl $2, %eax\n\t"
    // );

    // MSVC
    // __asm {
    //    mov eax, edx
    //    add eax, 2
    // }

    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "__asm__", "INLINE_ASM_A", 132);
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "__asm", "INLINE_ASM_B", 133);
```

## Parser

The parser has been extended by a parse_mode and it starts out in ```ParseMode::Default```
which is where it parses the grammar out of which the LALR(1) parser has been generated.

```
parser_mode: ParserMode::Default,
```

### Entering and exiting ```ParserMode::InlineAssembly```

In ```pub fn Parser::consume()```, if the parser sees a terminal token of type ```INLINE_ASM_A``` or ```INLINE_ASM_B```, it will change mode to ```ParserMode::InlineAssembly```.

```
// change parse mode temporarily
self.parser_mode = ParserMode::InlineAssembly;
```

It will stay in ```ParserMode::InlineAssembly``` until it sees a terminal token of type ```CLOSING_CURLY_BRACKET``` which is when it switches back to ```ParseMode::Default``` which means it leaves inline assembly parsing. This means that no terminal token ```CLOSING_CURLY_BRACKET``` is allowed to occure inside the inline assembly code as this will confuse the parser!

### Building InlineAssembly AST Nodes

For every block of inline assembly (which currently is only allowed to occur inside function bodies), the parser will create a AST Node of type inline assembly. The assembly code is stored inside the inline assembly AST node.

An inline assembly AST node is created inside ```pub fn Parser::consume()``` after entering ```ParserMode::InlineAssembly```.

A second block of token processing has been inserted into the parser. Here the parser will insert incoming token into the current inline assembly AST node, if it's parse mode is ```ParserMode::InlineAssembly```. Search for the comment text: ```// Consume Inline Assembly code``` in the parser.rs file to find this block.

The data is loaded into the AST node in the extra block for terminals during ```ParserMode::InlineAssembly```

## InlineAssembly inside the Tacky Visitor

The AST is processed by the Tacky Visitor to convert it to the immediate language TACKY. The AstNodes of type ```AstNodeType::InlineAssembly``` need to be converted to a TACKY instruction.

The function ```pub fn TackyVisitor::visit()``` will handle the AstNodes in the AST. The match branch for ```AstNodeType::InlineAssembly``` will handle the InlineAssembly AstNodes. These nodes have the inline assembly in their ```string_val``` field.

```
AstNodeType::InlineAssembly => {

}
```

## InlineAssembly inside the TackyToIntermediateAsmConversionVisitor

The ```TackyToIntermediateAsmConversionVisitor``` will just forward the inline assembly to the
intermediate assembly.

## Inline Assembly inside the asm_ast_fixup_visitor

The fixup visitor performs no action for inline assembly

## InlineAssembly inside the AsmAstMasmEmitterVisitor

The AsmAstMasmEmitterVisitor outputs the inline assembly:

```
AsmAstInstructionType::InlineAssembly => {
    match &asm_ast_instruction.src.operand_type {
        AsmAstOperandType::ImmString(inline_assembly_as_string) => {
            self.string_buffer.push_str(inline_assembly_as_string);
            self.string_buffer.push_str("\n");
        }
        _ => {
            panic!();
        }
    }
}
```