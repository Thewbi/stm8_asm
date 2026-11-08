// struct_field_access_arrow_0.c

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

    if (b == 8) {
        return 0;
    }

    return b;
}
