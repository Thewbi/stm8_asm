// res\C\samples\c_samples\putchar_0.c
//
// includelib kernel32.lib
// includelib User32.lib
// includelib legacy_stdio_definitions.lib
//
// extern putchar: proc
//
// cd C:\Users\lapto\dev\rust\stm8_asm
// "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvarsall.bat" x64
//
// target\debug\rust_pest_asm.exe -i res\C\samples\c_samples\putchar_0.c
//
// ml64.exe /Fo build\main.obj /c build\main.asm
//
// link.exe /SUBSYSTEM:console /LARGEADDRESSAWARE:NO /ENTRY:main /OUT:build\main.exe /MACHINE:X64 legacy_stdio_definitions.lib kernel32.lib user32.lib gdi32.lib winspool.lib comdlg32.lib advapi32.lib shell32.lib ole32.lib oleaut32.lib uuid.lib odbc32.lib odbccp32.lib build\main.obj
// "C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Tools\MSVC\14.44.35207\bin\HostX64\x64\link.exe" /NOLOGO /ENTRY:main /OUT:build\main.exe /SUBSYSTEM:CONSOLE kernel32.lib user32.lib gdi32.lib winspool.lib comdlg32.lib advapi32.lib shell32.lib ole32.lib oleaut32.lib uuid.lib odbc32.lib odbccp32.lib ucrt.lib msvcrt.lib build\main.obj
// This works:
// "C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Tools\MSVC\14.44.35207\bin\HostX64\x64\link.exe" /NOLOGO /ENTRY:main /OUT:build\main.exe /SUBSYSTEM:CONSOLE kernel32.lib user32.lib gdi32.lib winspool.lib comdlg32.lib advapi32.lib shell32.lib ole32.lib oleaut32.lib uuid.lib odbc32.lib odbccp32.lib ucrt.lib build\main.obj
//
// Executing:
// build\main.exe
//
// TODO:
// output: extern putchar: proc

//#include <stdio.h>

//int putchar(int c);

//int strlen(char *s)
//{
//    int len = 0;
//    char val = 0;
//    do {
//        val = *s;           // retrieve current char
//        s = s + 1;          // increment pointer for next loop
//        if (val != 0) {     // increment string length
//            len++;
//        }
//    } while (val != 0);
//
//    return len;
//}

int strlen(char *s, char *b)
//int strlen(char *s)
{
    //int i = 0;
    //while (s[i] != 0) {
    //    i++;
    //}
    //return i;

    int i = 0;
    while (b[i] != 0) {
        i++;
    }
    return i;
}

int main() {
    int result = strlen("abc def", "Hello World!"); // 0x61 0x62 0x63 0x20 0x64 0x65 0x66
    //int result = strlen("abc def");
    return result;
}
