int main() {

    int data[11] = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 0xaa};

    // This code makes no sense!
    int i = 0;
    while (data[i] != 0xaa) {
        i = i + data[i];
    }

    return 0;
}
