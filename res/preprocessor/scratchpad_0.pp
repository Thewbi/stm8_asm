struct listnode
{
    int a;
    int b;
    int c;
};

int main() {

    int b;

    struct listnode struct_1;
    struct_1.b = 177;

    b = struct_1.b;

    return b;
}
