struct listnode
{
    int a;
};

int main() {

    int b;

    struct listnode struct_1;
    struct_1.a = 123;
    b = struct_1.a;

    return b;
}
