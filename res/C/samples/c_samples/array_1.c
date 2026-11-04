int main() {
    //int arr[3]; // FIX: here, the compiler does not reserve space on the stack for the three elements!
    int arr[3] = { 10, 20, 30 };

    //int arr[3];
//
    //arr[0] = 1;
    //arr[1] = 2;
    //arr[2] = 3;
//
    //arr[2] = 123;

    // THIS WORKS
    int result;
    result = arr[2];

    if (result == 30) {
        return 0;
    }
    return 1;
}
