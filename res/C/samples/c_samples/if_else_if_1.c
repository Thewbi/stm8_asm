// res\C\samples\c_samples\if_else_if_1.c

int main() {
    int i = 15;
    if (i < 10) {
        i++;
    } else if (i < 20) {
        i++;
        i++;
    } else {
        i--;
    }

    if (i == 17) {
        return 0;
    }
    return i;
}
