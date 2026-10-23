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

    return 0;
}
