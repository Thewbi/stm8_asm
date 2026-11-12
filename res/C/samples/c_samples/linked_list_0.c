// res\C\samples\c_samples\linked_list_0.c

#include <stddef.h>

struct node {
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

    if (struct_2.val == 2) {
        return 0;
    }

    return struct_2.val;
}
