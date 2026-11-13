// res\C\samples\c_samples\struct_5.c

struct point {
    int x;
    int y;
};

struct tile {
    struct point a;
};

int main()
{
    // point_t p0 = { 3, 4 };
    // p0.x = 5;
    // p0.y = 6;

    struct tile t;
    t.a.x = 100;

    return 0;
}
