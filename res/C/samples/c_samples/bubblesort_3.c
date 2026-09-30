int main() {
    //int arr[7] = { 64, 34, 25, 12, 22, 11, 90 };
    int arr[] = { 64, 34, 25, 12, 22, 11, 90 };

    int n = 6;

    // int i;
    int j;
    // int temp;

    int swapped;
    swapped = 1;

    int a;
    // int a = 0;
    int b;
    // int b = 0;

    int next_idx;
    next_idx = 0;

    while (swapped) {

         swapped = 0;

         for (j = 0; j < n; j++) {

            //arr[j] = 123;

             a = arr[j];
             next_idx = j + 1;
             b = arr[next_idx];

    //         // FIX
    //         //b = arr[j+1];

    //         // FIX
             if (a > b)
             {
    //             // FIX
    //             //arr[j] = 1;
                 arr[j] = b;
                 //arr[j] = 123;

    //             // FIX
                 arr[next_idx] = a;

    //             // OK
                 swapped = 1;
             }
         }
    }


    if (arr[0] != 11) {
        return 611;
    }
    if (arr[1] != 12) {
        return 612;
    }
    if (arr[2] != 22) {
        return 622;
    }
    if (arr[3] != 25) {
        return 625;
    }
    if (arr[4] != 34) {
        return 634;
    }
    if (arr[5] != 64) {
        return 664;
    }
    if (arr[6] != 90) {
        return 690;
    }


    // int result;
    // result = arr[0];

    // return result;

    return 0;
}