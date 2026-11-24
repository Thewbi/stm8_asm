// res\C\samples\c_samples\unsigned_char_division_0.c

// mov dword ptr [rbp+-4], 10      ; divisor
// mov eax, 100                    ; dividend
// cdq
// idiv dword ptr [rbp+-4]         ; parameter needs to be divisor
//                                 ; result is in eax

int main() {
    unsigned char data = 100;

    data = data / 10;

    return data;
}