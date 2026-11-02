struct listnode
{
    int a;
};

int main() {

    int b;

    struct listnode struct_0;
    struct_0.a = 8;

    struct listnode *struct_ptr;
    struct_ptr = &struct_0;

    b = struct_ptr->a;

    return b;
}
