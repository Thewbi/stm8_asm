#include <stddef.h>

// https://nothingleaves.com/tools/memory-layout-visualizer/
//
// The struct has a total size of 16 byte since the four byte
// int val is padded so that the second field (pointer) can
// receive the desired alignment of 8. The second field is
// 8 byte (size of pointer on x64). So the total size is
// 4 byte (val) + 4 byte padding (for alignment) + 8 byte (*next) = 16 byte.
struct node {
    int val;
    struct node *next;
};

typedef struct node node_t;

int main() {

    node_t struct_1; // 16 byte
    node_t struct_2; // 16 byte

    struct_1.val = 1;
    struct_2.val = 2;

    struct_1.next = &struct_2;
    struct_2.next = NULL;

    return struct_1.next->val;
    //return 300;

    //return struct_1.val;
    //return struct_2.val;
}
