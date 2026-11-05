int main() {
    int arr[3];

    arr[0] = 0x10;
    arr[1] = 0x20;
    arr[2] = 0x30;

    if (arr[2] == 0x30) {
        return 0;
    }
    return arr[2];
}
