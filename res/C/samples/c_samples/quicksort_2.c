#include <stdio.h>

// Funktion zum Tauschen zweier Elemente
void swap(int* a, int* b) {
    int t = *a;
    *a = *b;
    *b = t;
}

// Diese Funktion nimmt das letzte Element als Pivot, platziert
// es an der richtigen Position und schiebt kleinere Elemente nach links
int partition(int arr[], int low, int high) {
    int pivot = arr[high]; // Pivot-Element
    int i = (low - 1);     // Index des kleineren Elements

    for (int j = low; j <= high - 1; j++) {
        // Wenn das aktuelle Element kleiner oder gleich dem Pivot ist
        if (arr[j] <= pivot) {
            i++; // Index erhöhen
            swap(&arr[i], &arr[j]);
        }
    }
    // Platziere das Pivot-Element direkt nach den kleineren Elementen
    swap(&arr[i + 1], &arr[high]);
    return (i + 1);
}

// Die Hauptfunktion, die Quicksort implementiert
void quickSort(int arr[], int low, int high) {
    if (low < high) {
        // pi ist der Partitionierungs-Index
        int pi = partition(arr, low, high);

        // Teilarreays vor und nach der Partitionierung separat sortieren
        quickSort(arr, low, pi - 1);
        quickSort(arr, pi + 1, high);
    }
}

// Hilfsfunktion zum Drucken des Arrays
void printArray(int arr[], int size) {
    for (int i = 0; i < size; i++)
        printf("%d ", arr[i]);
    printf("\n");
}

int main() {
    int arr[] = {10, 7, 8, 9, 1, 5};
    int n = sizeof(arr) / sizeof(arr[0]);
    
    printf("Unsortiertes Array: \n");
    printArray(arr, n);
    
    quickSort(arr, 0, n - 1);
    
    printf("Sortiertes Array: \n");
    printArray(arr, n);
    return 0;
}
