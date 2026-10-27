struct listnode
{
    int a;
    long b;
    int *c;
    struct listnode *next;
};

typedef struct listnode listnode_t;


//typedef struct listnode listnode_t;
//
//struct listnode struct_1;
//
//// syntax not supported
////struct listnode struct_1 = { .a = 100 };
//
//struct listnode struct_2 = { 100 };
