int main() {
    //int arr[3]; // FIX: here, the compiler does not reserve space on the stack for the three elements!
    int arr[3] = { 10, 20, 30 };

    arr[2] = 123;

    //int result = arr[2];

    // THIS WORKS
    int result;
    // result = arr[2];
    result = arr[0];

    return result;
    //return 0;
}