struct point {
    int x;
    int y;
};

typedef struct point point_t;

int main()
{
    struct point p1 = { 1, 2 };

    p1.x = 2;

    return p1.x;
}