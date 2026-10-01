# The Preprocessor

Many tests:

https://github.com/Thewbi/cpp_compiler/tree/main/src/test/resources/preprocessor

??? Backslash-Newline sequences are deleted, no matter where.

```
#define
#elif
#else
#endif
#error
#if
#ifdef
#ifndef
#import
#include
#line
#pragma
#undef
#using
```

https://github.com/Thewbi/cpp_compiler/blob/main/src/main/java/preprocessor/SimpleFileStackFrame.java

The input is a single compilation unit which is a .c file which draws in many .h files which in turn
may also draw in more .h files by including them. The output is one large.c file with all the values
from the .c and all included .h files. If two or more .h files for a loop by cyclicly including themselves,
the loop is broken by including each .h file at most once.

.h files are identified by their absolute or relative path. Relative paths need one or more base paths
to be resolved into absolute paths. Based on the type of #include symbol the use of paths change.
For chevron includes (#include <stdio.h>) the base paths are toolchain system paths and -I command line inputs.
For quote includes (#include "test.h") the include paths are resolved relative to the currently
process file (.c or .h).

A SimpleFileStackFrame object stands for either the main.c or any of the included files which.
Every included .h file is represented by it's own SimpleFileStackFrame.

SimpleFileStackFrame will use a lexer only to process the file. The preprocessor knows nothing
about the C grammar, it is a text processor.

The preprocessor iterates over the token it scans from the input file of the SimpleFileStackFrame.
There is a large if-else statement for each type of token.

As the preprocessor (implemented inside SimpleFileStackFrame) scans through the input it enters
different modes based on the items it has seen last. For example if an expression is detected,
it will enter ParserMode.EXPRESSION. It uses a hand-crafted expression parser to build trees from expressions.

Every if case in the large it-statement is created for a specific token-type. Within the branches,
there is a second if-else statement for the different modes that the preprocessor can be in.

As the preprocessor goes, it will build up different #define objects and store them into a datastructure
for later application in normal text. There is only one #define object data store for the entire
translation unit. This means #define object from included header files are also used when processing
the base .c file which has performed the include and also when processing all subsequent .h files.

If a token is normal text (no preprocessor token), then the normal text is compared to the #define objects
in the database. If the normal text matches one of the #define objects, the #define object is applied.
This means that the formal parameters need to be replaced by actual parameters, and the normal text is
replaced by the processed #define object and the newly replaced text is output by the preprocessor into
the large resulting StringBuffer or text file.

The functions filterByPreprocessorValues() and evaluatePreprocessorTreeNode() are used to perform these
replacement steps.

As an example, take a look at the SQUARE(x) #define object:

```
#define SQUARE(x) ((x) * (x))
```

When SQUARE is processed, it's expression x * x is parsed and inserted into the database. Whenever SQUARE
is encountered in normal text, it's formal parameter x is replaced and the replaced expression is output.

```
int a = SQUARE(5);
```

is replaced by

```
int a  = ((5) * (5));
```

```
/**
 * This function contains the basic idea of the algorithm. The purpose of the
 * algorithm is to parse expressions without a parser, using a lexer and token
 * only.
 * The expression is represented using a binary tree.
 * To build the tree, all token types are identified and if the token is a C/C++
 * operator, the precedence of that operator is used as a weight. The heavier
 * the node (higher precedence) the deeper the token will sink into the tree.
 * Literals have the highest weight and will sink down and act as the leafs of
 * the tree (Leaf == no children).
 */
```




# Comments

Needs to remove comments by a single space.

Removes comments (single line // and multiline comments are removed /* */
and replaced by a single space character).

Comments are not part of the C-grammar! This means the C-compiler cannot process comments.
This is the reason why the preprocessor needs to remove comments!

## Single line comment

```
// single line comment on it's own

int a = 1; // single line comment after a instruction
```

## Multiline Comment (Standard C comment)

```
/* test */

/* test
*/

/*
 * Extended shape
 */

printf("Hello World! %s %s %d", "a"/*a*/, "b"/*b*/, 5/*c*/); // interleaved multiline
```