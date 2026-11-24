// res\C\samples\c_samples\malloc_0.c

#define BYTE_BUFFER_SIZE 65535

//void* malloc(int amount_of_bytes);
int malloc(int amount_of_bytes);
//int malloc(int);

int main() {

    //malloc(100);
    int buffer = malloc(BYTE_BUFFER_SIZE);
    return 0;
}
