int main() {
    int wochentag = 4;
    int a = 0;

    switch (wochentag) {
        case 1:
            {
                a = a + 1;
                a = a + 1;
            }
            break;
        case 2:
            a = a - 1;
            break;
        case 3:
            a = a + 6;
            break;
        default:
            a = a + 90;
            break;
    }

    if (a == 6) {
        return 0;
    }
    return a;
}
