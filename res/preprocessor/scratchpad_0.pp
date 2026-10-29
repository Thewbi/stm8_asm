struct listnode
{
    int a;
    int b;
};

int main() {

    struct listnode struct_1;
    // struct_1.a = 1;

    struct listnode *struct_ptr;
    struct_ptr = &struct_1;
    struct_ptr->a = 2;
    struct_ptr->b = 3;

    return 0;
}
