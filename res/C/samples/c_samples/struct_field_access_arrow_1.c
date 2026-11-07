struct listnode
{
    int a;
    int b;
    int c;
};

int main() {

    int b;

    struct listnode struct_1;

    struct listnode *struct_ptr;
    struct_ptr = &struct_1;

    struct_ptr->b = 8;

    b = struct_ptr->b;

    if (b == 8) {
        return 0;
    }

    return b;
}
