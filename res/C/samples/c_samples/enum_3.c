// res\C\samples\c_samples\enum_3.c

const int MAX_SPEED = 120;

enum speed {
    LOW_SPEED = 0,
    MID_SPEED = 1,
    MAX_SPEED = 2 // this should cause an error since the constant above uses the same name
};

int main() {

    enum speed var_1 = MAX_SPEED;

    return 0;
}
