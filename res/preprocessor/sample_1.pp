#define EXPR_A (2 + 3)
#define EXPR_B (EXPR_A * 4)

#if EXPR_B == 20
    // This code WILL be included
#endif



#define VAL 1+1

int main() {

#if defined VAL
    printf("Hello World! %s %s %d", "a"/*a*/, "b"/*b*/, 5/*c*/);
#endif

    return 0;
}
