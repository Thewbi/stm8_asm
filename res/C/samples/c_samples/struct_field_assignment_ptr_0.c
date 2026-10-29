struct listnode
{
    int a;
};

int main() {

    struct listnode struct_1;
    // struct_1.a = 1;

    struct listnode *struct_ptr;
    struct_ptr = &struct_1;
    struct_ptr->a = 2;

    return 0;
}
