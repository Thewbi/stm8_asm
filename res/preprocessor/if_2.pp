#define EXPR_A (2 + 3)
#define EXPR_B (EXPR_A * 4)

#if EXPR_B == 20
    // This code WILL be included
#endif