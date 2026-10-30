#include <stddef.h>

struct node {
    //void *val;
    int val;
    struct node *next;
};

typedef struct node node_t;

int main() {

    node_t struct_1;
    node_t struct_2;

    struct_1.val = 1;
    struct_2.val = 2;

    struct_1.next = &struct_2;
    struct_2.next = NULL;

    return 0;
}
