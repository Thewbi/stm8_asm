struct point {
    int x;
    int y;
};

typedef struct point point_t;

int main()
{
    point_t p0 = { 3, 4 };
    p0.x = 5;
    p0.y = 6;

    struct point p1 = { 1, 2 };
    p1.x = 2;

    return p1.x;
}
