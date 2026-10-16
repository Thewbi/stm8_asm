// the preprocessor cannot parse this since it thinks that
// (a + b) is the interface
// * c is the defintion
//
// This means the output of the macro TEST will be * c
// and the preprocessor will complain about missing actual parameters.
//
// Probably the only way to get this macro to work as expected is to
// use more brackets in order to disambiguate the input:
// #define TEST ((a + b) * c)

#define TEST (a + b) * c

Here is the macro: TEST
