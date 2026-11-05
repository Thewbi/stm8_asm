int main() {
    int wochentag = 1;
    int a = 0;

    switch (wochentag) {
        case 1: {
            a = a + 1;
        }
            break;
        case 2:
            break;
        case 3:
            break;
        default:
            break;
    }

    if (a == 1) {
        return 0;
    }
    return a;
}
