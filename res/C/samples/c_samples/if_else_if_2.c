// res\C\samples\c_samples\if_else_if_2.c

int main() {

    int index = 1;
    int vv = 0;
    if ((index >= 0) && (index <= 9)) {
        vv = index + 48;
    } else if ((index >= 10) && (index <= 15)) {
        vv = index + 65 - 10;
    }

    return vv;
}