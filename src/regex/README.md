# Generation

The lexer is not hand-crafted but generated from regular expressions.

The regular expressions form the input to the generation.
They are formulated

Since generating the Lexer takes a couple of seconds, and this can be annoying on every compilation cycle, the lexer can be stored to a file and loaded back from the file instead of being generated on the fly. Loading from file is almost unnoticable fast so this is the standard. To generate the lexer, update main.rs.

```
// let generate_lexer = true;
let generate_lexer = false;
if generate_lexer {
    dfa = produce_c_lexer();
    // store into file
    enfa_serialize(&mut dfa, "enfa.txt");
} else {
    // load from file
    enfa_deserialize(&mut dfa, "enfa.txt");
}
```

The function produce_c_lexer(); generates the lexer. It is contained in src\example_lexers\c_lexer.rs

The function produce_c_lexer() describes all regular expressions in the form of rust-code. There is no nice lexer file format for now.

produce_c_lexer() makes heavy use of add_token_definition() which inserts new regexes to the large resulting NFA automaton.

Once all regexes are added to the NFA, the NFA is converted to a DFA.
The DFA is deterministic and fast enough (sacrificing resources) for parsing the input.

The basic steps are:

1. Convert the input regex_infix from infix to postfix notation using the converter.
2. The postfix notation is used as a guidance rule for building fragments on the
   fragment stack. The fragment stack eventually contains a single fragment that
   points to the start and end state of an automaton for the input regex.
   See README.md in the base folder for a detailed explanation of the algorithms used.
3. Finalize the resulting fragment. State id and token name are inserted.
4. Insert the new fragment to all prior regexes stored in the combined_fragment.
5. Make the new fragment accessible from the old start state using an epsilon transition
6. Create a DFA from the NFA