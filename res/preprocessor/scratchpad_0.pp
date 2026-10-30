struct listnode
{
    int a;
};

int main() {

    int b;

    struct listnode struct_1;

    struct listnode *struct_ptr;
    struct_ptr = &struct_1;

    b = struct_ptr->a;

    return 0;
}
