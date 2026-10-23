// not allowed by grammar. Requires separation of struct and typedef
// typedef struct Point {
//     int x;
//     int y;
// } point_t;

// not allowed by grammar. Requires separation of struct and typedef
// typedef struct {
//     float x;
//     float y;
// } Vector2D;

// typedef unsigned char uint8_t;
// typedef signed char int8_t;
// typedef char int8_t;

// illegal: byte is not defined by the grammar
// typedef unsigned byte uint8_t;
// typedef byte uint8_t;

//typedef struct point point_t;

// int a;

// #include <stdint.h>
//
// //typedef int own_int;
//
struct point {
    unsigned int x;
    signed int y;
    int z;
};

typedef struct point point_t;
//
// int main()
// {
//     //point_t p0;
//     struct point p0;
//
//     return 0;
// }
