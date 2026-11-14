// res\C\samples\c_samples\char_2.c

struct node_struct
{
   char a;
};

int main() {
    struct node_struct node_1;
    node_1.a = 20;

    if (node_1.a == 20) {
        return 0;
    }
    return node_1.a;
}