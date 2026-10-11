// the preprocessor cannot parse this since it thinks that
// (a + b) is the interface
// * c is the defintion
//
// This means the output of the macro TEST will be * c
// and the preprocessor will complain about missing actual parameters

#define TEST (a + b) * c

Here is the macro: TEST