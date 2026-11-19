# Introduction

It is possible to insert assembly code into a .c file. This is called inline assembly.

The compiler will forward this inline assembly to the output assembly file, so it
is processed by the assembler along with the rest of the generated assembly.

## Parser

The parser has been extended by a parse_mode and it starts out in ```ParseMode::Default```
which is where it parses the grammar out of which the LALR(1) parser has been generated.

```
parser_mode: ParserMode::Default,
```

## Entering and exiting ```ParserMode::InlineAssembly```

In ```pub fn Parser::consume()```, if the parser sees a terminal token of type ```INLINE_ASM_A``` or ```INLINE_ASM_B```, it will change mode to ```ParserMode::InlineAssembly```.

```
// change parse mode temporarily
self.parser_mode = ParserMode::InlineAssembly;
```

It will stay in ```ParserMode::InlineAssembly``` until it sees a terminal token of type ```CLOSING_CURLY_BRACKET``` which is when it switches back to ```ParseMode::Default``` which means it leaves inline assembly parsing. This means that no terminal token ```CLOSING_CURLY_BRACKET``` is allowed to occure inside the inline assembly code as this will confuse the parser!

## Building InlineAssembly AST Nodes

For every block of inline assembly (which currently is only allowed to occur inside function bodies), the parser will create a AST Node of type inline assembly. The assembly code is stored inside the inline assembly AST node.

An inline assembly AST node is created inside ```pub fn Parser::consume()``` after entering ```ParserMode::InlineAssembly```.

A second block of token processing has been inserted into the parser. Here the parser will insert incoming token into the current inline assembly AST node, if it's parse mode is ```ParserMode::InlineAssembly```. Search for the comment text: ```// Consume Inline Assembly code``` in the parser.rs file to find this block.

## Lexer