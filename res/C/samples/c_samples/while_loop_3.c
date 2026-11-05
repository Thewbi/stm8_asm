int main() {

    int i = 0;
    int j = 0;
    while (i < 2) {
        j = j + 1;
        i = i + 1;
    }

    if (j == 2) {
        return 0;
    }
    return j;
}
