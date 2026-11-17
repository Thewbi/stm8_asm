// res\C\samples\c_samples\inline_assembly_0.c

int main() {

    __asm {
       mov eax, edx
       add eax, 2
    }

    return 0;
}
