int foo(int a, int b) {
// int foo(int a, int b, int c, int d, int e, int f, int g) {
// int foo(int a, int b, int c, int d, int e, int f, int g, int h) {
    //return 1;

    return a + b;
    //return a - b;
}

int main(void) {
    
    int a;
    a = foo(6, 4);
    // a = foo(1, 2, 3, 4, 5, 6, 7);
    //a = foo(1, 2, 3, 4, 5, 6, 7, 8);
    return a;

    // int b;
    // b = 2 + 3;
    // return b;
}