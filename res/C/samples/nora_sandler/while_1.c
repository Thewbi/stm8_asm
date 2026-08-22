int foo() {
    int a = 0;
    int b = 0;

    while (a < 10) {
        a = a + 2;
        b = b + 200;
    }

    // add a ret automatically in the compiler!
    return b;
}

int main() {

    int a = foo();

    return a;
}