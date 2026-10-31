#include <stdint.h>

//typedef int own_int;

struct point {
    int x;
    int y;
};

//typedef struct point point_t;

int main()
{
    //point_t p0;
    struct point p0;
    p0.x = 100;

    return p0.x;
}
