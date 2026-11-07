struct listnode
{
    int a;
    int b;
    int c;
};

int main() {

    //int a;
    //a = 1;

    int *b;

    struct listnode struct_1;
    struct_1.b = 8;

    b = &struct_1.b;

    if (*b == 8) {
        return 0;
    }

    //return b;

    return 500;
}
