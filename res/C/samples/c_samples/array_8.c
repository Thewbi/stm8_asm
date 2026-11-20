// res\C\samples\c_samples\array_8.c

int main() {
    int arr[100];

    //arr[0] = 0;
    //arr[1] = 1;

    int i = 0;
    for (i = 0; i < 100; i++) {
        arr[i] = i;
    }

    if (arr[3] == 3) {
        return 0;
    }

    return arr[3];
}
