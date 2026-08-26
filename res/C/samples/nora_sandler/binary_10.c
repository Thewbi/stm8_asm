// a function name is used as a variable in an expression!
// The TypeCheckingVisitor checker needs to catch this case!

int foo() {
    return 0;
}

int main(void) {

    int a = 3;
    int c = a + foo;

    return b;
}