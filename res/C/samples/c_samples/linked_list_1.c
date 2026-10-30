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
    node_t struct_3;
    node_t struct_4;
    node_t struct_5;

    struct_1.val = 1;
    struct_2.val = 2;
    struct_3.val = 3;
    struct_4.val = 4;
    struct_5.val = 5;

    struct_1.next = &struct_2;
    struct_2.next = &struct_3;
    struct_3.next = &struct_4;
    struct_4.next = &struct_5;
    struct_5.next = NULL;

    return 0;
}
