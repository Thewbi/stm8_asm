// res\C\samples\c_sample_2\gameboy_decoder.c

// Links
// https://gbdev.io/pandocs/The_Cartridge_Header.html
// https://archive.gbdev.io/salvage/decoding_gbz80_opcodes/Decoding%20Gamboy%20Z80%20Opcodes.html

// https://gbdev.io/rgbds-live/
// https://www.inspiredpython.com/course/game-boy-emulator/game-boy-emulator-writing-the-z80-disassembler

// The source code of the ROM is here:
// https://github.com/nitro2k01/bgbtest/blob/master/src/bgbtest.asm

// 00 C3 50 01
//
// 00 - 00 is nop
//
// C3 [50 01] - C3 is jump, the next two byte are a big-endian 2 byte address: 0x0150
//
// F3 31 FE FF F0 44 FE 90 20 FA AF E0 40 21 00 80
//
// Disassembly
//
// Offset	    Bytes	        Assembly	        Effect
// +00	        F3	            DI	                Disables interrupts.
// +01	        31 FE FF        LD SP, $FFFE	    Sets the stack pointer to $FFFE (top of HRAM).
// -- WAITVBL --------------------------------------------------------------------------------------
// +04	        F0 44	        LDH A, ($44)	    Reads LY ($FF44), the current LCD scanline.
// +06	        FE 90	        CP $90	            Compares A with $90 (144, the start of VBlank).
// +08	        20 FA           JR NZ, -$06	        Loops back to +04 (LDH A, ($44)) until LY == $90.
// -------------------------------------------------------------------------------------------------
// +0A	        AF	            XOR A	            Sets A to $00.
// +0B	        E0 40	        LDH ($40), A	    Writes $00 to LCDC ($FF40), turning off the LCD.
// +0D	        21 00 80        LD HL, $8000	    Loads $8000 (start of VRAM tile data) into HL.
//

#define BYTE_BUFFER_SIZE 10240

int CreateFileA(char *lpFileName,
    int dwDesiredAccess,
    int dwShareMode,
    int lpSecurityAttributes,
    int dwCreationDisposition,
    int dwFlagsAndAttributes,
    int hTemplateFile);

int WriteFile(
    int hFile,
    char *lpBuffer,
    int nNumberOfBytesToWrite,
    long lpNumberOfBytesWritten,
    long lpOverlapped);

int ReadFile(
    int hFile,
    char *lpBuffer,
    int nNumberOfBytesToRead,
    long lpNumberOfBytesRead,
    long lpOverlapped);

int main() {

    int file_handle = CreateFileA(
        "C:\\users\\lapto\\Downloads\\bgbw64\\bgbtest.gb",
        0x80000000,     // dwDesiredAccess (GENERIC_READ: 0x80000000)
        1,              // dwShareMode (FILE_SHARE_READ: 0x00000001)
        0,              // lpSecurityAttributes
        3,              // dwCreationDisposition (OPEN_EXISTING: 3)
        0x80,           // dwFlagsAndAttributes (FILE_ATTRIBUTE_READONLY: 1, FILE_ATTRIBUTE_NORMAL: 0x80)
        3               // hTemplateFile
    );

    char buffer[BYTE_BUFFER_SIZE];

    ReadFile(
        file_handle,    // hFile - file handle
        buffer,         // lpBuffer - the buffer to store read data into
        BYTE_BUFFER_SIZE,  // nNumberOfBytesToRead
        0,              // lpNumberOfBytesRead
        0               // lpOverlapped
    );

    return buffer[0x0150];
}
