// sieve_of_eratosthenes.c

#include <math.h>

int main() {
    return 0;
}



/*
#include <stdio.h>
#include <stdbool.h>
#include <math.h>

void sieveOfEratosthenes(int n) {
    // Ein boolesches Array erstellen und mit true initialisieren
    // prime[i] wird true sein, wenn i eine Primzahl ist, sonst false.
    bool prime[n + 1];
    for (int i = 0; i <= n; i++) {
        prime[i] = true;
    }

    prime[0] = false;
    prime[1] = false;

    // Durchlaufe bis zur Quadratwurzel von n
    for (int p = 2; p <= sqrt(n); p++) {
        // Falls prime[p] nicht veraendert wurde, ist es eine Primzahl
        if (prime[p] == true) {
            // Alle Vielfachen von p ab p*p als false markieren
            for (int i = p * p; i <= n; i += p) {
                prime[i] = false;
            }
        }
    }

    // Alle Primzahlen ausgeben
    printf("Primzahlen bis %d:\n", n);
    for (int p = 2; p <= n; p++) {
        if (prime[p]) {
            printf("%d ", p);
        }
    }
    printf("\n");
}

int main() {
    int n = 50;
    sieveOfEratosthenes(n);
    return 0;
}
*/
