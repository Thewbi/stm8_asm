// a variable is declared with the same name as a function that was declared earlier
// This is ok! The variable name is replaced with a new temp variable name

int foo() {
    return 0;
}

int main() {
    int foo;
    return foo;
}