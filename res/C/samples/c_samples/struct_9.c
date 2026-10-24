typedef char int8_t;
typedef unsigned char uint8_t;

struct listnode
{
   int data_int;
   int *data_int_ptr;

   long data_long;
   long *data_long_ptr;

   int8_t data_int8_t;

   uint8_t data_uint8_t;

   //struct listnode *next;
};

typedef struct listnode listnode_t;
