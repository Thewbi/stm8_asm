int main(void) {

    int data[11] = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 0xaa};

    //int i = 0;
    int j = 0;
    while (data[j] != 0xaa) {
        //i = i + data[i];
        j = j + 1;
    }

    __asm__("halt\n");
    __asm__("wfi\n");

    return 0;
}
