// res\C\samples\c_samples\fopen_2.c

int CreateFileA(char *lpFileName,
    int dwDesiredAccess,
    int dwShareMode,
    int lpSecurityAttributes,
    int dwCreationDisposition,
    int dwFlagsAndAttributes,
    int hTemplateFile);

int main() {

    // https://learn.microsoft.com/de-de/windows/win32/api/fileapi/nf-fileapi-createfilea
    //
    // HANDLE CreateFileA(
    //  [in]           LPCSTR                lpFileName,
    //  [in]           DWORD                 dwDesiredAccess,
    //  [in]           DWORD                 dwShareMode,
    //  [in, optional] LPSECURITY_ATTRIBUTES lpSecurityAttributes,
    //  [in]           DWORD                 dwCreationDisposition,
    //  [in]           DWORD                 dwFlagsAndAttributes,
    //  [in, optional] HANDLE                hTemplateFile
    // );

    CreateFileA(
        //"C:\\Users\\lapto\\source\\repos\\x64_test\\x64\\Debug\\test.txt", // lpFileName
        "C:\\Users\\lapto\\temp\\fopen_2_test.txt",
        0x10000000, // dwDesiredAccess
        0,          // dwShareMode
        0,          // lpSecurityAttributes
        0,          // dwCreationDisposition
        0x80,       // dwFlagsAndAttributes
        4           // hTemplateFile
    );

    return 0;
}