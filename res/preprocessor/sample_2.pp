#define ADD(x, y) (x + y)
#define EXPR_A (ADD(7, 8) + 2 + 3)
#define EXPR_B (EXPR_A * 4)

#if EXPR_B == 80
    // This code WILL be included
#endif
