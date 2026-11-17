// res\C\samples\c_samples\if_else_if_0.c

int main() {
    int i = 0;
    if (i < 10) {
        i++;
    } else if (i < 20) {
        i++;
        i++;
    } else {
        i--;
    }

    if (i == 1) {
        return 0;
    }
    return i;
}
