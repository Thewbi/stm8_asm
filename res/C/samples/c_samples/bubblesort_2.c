// #include <stdio.h>

// Funktion zum Sortieren eines Arrays mittels Bubble Sort
// void bubble_sort(int *arr, int n) {
//void bubble_sort(int arr[], int n) {
    // int i, j, temp;
    // int swapped;

    // for (i = 0; i < n - 1; i++) {
    //     swapped = 0; // Optimierung: Prüfen, ob ein Tausch stattgefunden hat

    //     for (j = 0; j < n - i - 1; j++) {
    //         if (arr[j] > arr[j + 1]) {
    //             // Elemente tauschen
    //             temp = arr[j];
    //             arr[j] = arr[j + 1];
    //             arr[j + 1] = temp;
    //             swapped = 1;
    //         }
    //     }

    //     // Wenn in diesem Durchlauf kein Tausch stattfand, ist das Array bereits sortiert
    //     if (swapped == 0) {
    //         break;
    //     }
    // }
// }

int main() {
    //int arr[7] = {64, 34, 25, 12, 22, 11, 90};
    int arr[] = {64, 34, 25, 12, 22, 11, 90};

    //int n = sizeof(arr) / sizeof(arr[0]);
    int n = 7;

    //int i, j, temp;

    int i;
    int j;
    int temp;

    int swapped;

    // printf("Unsortiertes Array:\n");
    // for (int i = 0; i < n; i++) {
    //     printf("%d ", arr[i]);
    // }
    // printf("\n");

    //bubble_sort(arr, n);

    // printf("Sortiertes Array:\n");
    // for (int i = 0; i < n; i++) {
    //     printf("%d ", arr[i]);
    // }
    // printf("\n");

    // for (i = 0; i < n - 1; i++) {
    for (i = 0; i < n; i++) {
        // Optimierung: Pruefen, ob ein Tausch stattgefunden hat
        swapped = arr[i];
    }

    return swapped;
}