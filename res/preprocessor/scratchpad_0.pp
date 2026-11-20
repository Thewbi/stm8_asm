// res\C\samples\c_samples\function_call_5.c

#define ARRAY_LENGTH 100

int array_sum(char *lpBuffer, int length) {

    int result = 0;
    int i = 0;
    for (i = 0; i < length; i++) {
        //result += lpBuffer[i];
        result = result + lpBuffer[i];
    }

    //return lpBuffer[15];
    //return length;
    return result;
}

int main() {

    char buffer[ARRAY_LENGTH];

    //buffer[10] = 10;

    int i = 0;
    for (i = 0; i < ARRAY_LENGTH; i++) {
        buffer[i] = i;
    }

    int result = array_sum(buffer, ARRAY_LENGTH);
    //int result = array_sum("test");

    return result;
}