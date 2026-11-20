// res\C\samples\c_samples\function_call_4.c

int test_function(char *param_1,
    int param_2,
    int param_3,
    int param_4,
    int param_5,
    int param_6,
    int param_7)
{
    return param_2;
}

int main() {

    int result = test_function(
        1,
        2,
        3,
        4,
        5,
        6,
        7
    );

    return result;
}