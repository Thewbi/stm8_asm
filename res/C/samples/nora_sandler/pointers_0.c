int main(void) {

    int x = 0;
    int *ptr = &x;
    *ptr = 4;

    return *ptr;
}