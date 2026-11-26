// res\C\samples\c_sample_2\gameboy_decoder_1.c

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

//#include <stdio.h>

//#define BYTE_BUFFER_SIZE 10240
//#define BYTE_BUFFER_SIZE 0x7FFF
//#define BYTE_BUFFER_SIZE 32767
//#define BYTE_BUFFER_SIZE 65535
#define BYTE_BUFFER_SIZE 65536

// res\C\samples\c_samples\print_string_0.c

int putchar(int c);
int malloc(int amount_of_bytes);

int print_string(char* s)
{
    int len = 0;
    int val = 0;
    do {
        val = *s;           // retrieve current char
        s = s + 1;          // increment pointer for next loop
        if (val != 0) {     // increment string length
        //if ((val != 0) && (val != '\')) {     // increment string length
            int result = putchar(val);
            len++;
        }
    } while (val != 0);

    putchar(13); // \r (Carriage Return, ASCII 13 / 0x0D)
    putchar(10); // \n (Line Feed / Newline, ASCII 10 / 0x0A)

    return len;
}

int putchar(int c);

void print_unsigned_char_numeric_decimal(unsigned char data) {
    unsigned char buffer[3];
    int text_len = 0;
    while (data > 0) {

        //putchar('a');

        //data = data - 1;

        buffer[text_len] = data % 10;

        //text_len++;
        text_len = text_len + 1;

        data = data / 10;

        if (data == 0) {
            break;
        }
    }

    //int vv = text_len + 48;
    //putchar(vv);

    //putchar('x');

    //int i = text_len-1;
    //for (; i >= 0; i--) {
    //    putchar(buffer[i] + '0');
    //}

    // FIX:
    // xor rax, rax
    // mov al, byte ptr [rbp+-1]
    // cdp
    //
    // mov byte ptr [rbp-16+rax*1], cl

    //int i = 0;
    //for (i = 0; i < text_len; i++) {

    //int i = text_len - 1;
    text_len = text_len - 1;
    int i = 0;
    //for (i = 1; i >= 0; i--) {
    for (i = text_len; i >= 0; i--) {
         //putchar('c');
         //putchar(buffer[i] + '0');

         int temp = buffer[i];
         int vv = temp + 48;
         putchar(vv);
    }

    putchar(13); // \r (Carriage Return, ASCII 13 / 0x0D)
    putchar(10); // \n (Line Feed / Newline, ASCII 10 / 0x0A)
}

int CreateFileA(char* lpFileName,
    int dwDesiredAccess,
    int dwShareMode,
    int lpSecurityAttributes,
    int dwCreationDisposition,
    int dwFlagsAndAttributes,
    int hTemplateFile);

int WriteFile(
    int hFile,
    char* lpBuffer,
    int nNumberOfBytesToWrite,
    long lpNumberOfBytesWritten,
    long lpOverlapped);

int ReadFile(
    int hFile,
    char* lpBuffer,
    int nNumberOfBytesToRead,
    long lpNumberOfBytesRead,
    long lpOverlapped);

int main() {

    int file_handle = 0;

    file_handle = CreateFileA(
        "C:\\users\\lapto\\Downloads\\bgbw64\\bgbtest.gb",
        0x80000000,     // dwDesiredAccess (GENERIC_READ: 0x80000000)
        1,              // dwShareMode (FILE_SHARE_READ: 0x00000001)
        0,              // lpSecurityAttributes
        3,              // dwCreationDisposition (OPEN_EXISTING: 3)
        0x80,           // dwFlagsAndAttributes (FILE_ATTRIBUTE_READONLY: 1, FILE_ATTRIBUTE_NORMAL: 0x80)
        3               // hTemplateFile
    );

    //unsigned char buffer[BYTE_BUFFER_SIZE];
    char buffer[BYTE_BUFFER_SIZE];

    //unsigned char buffer = malloc(BYTE_BUFFER_SIZE);
    //char buffer = malloc(BYTE_BUFFER_SIZE);

    ReadFile(
        file_handle,    // hFile - file handle
        buffer,         // lpBuffer - the buffer to store read data into
        BYTE_BUFFER_SIZE,  // nNumberOfBytesToRead
        //1000,
        0,              // lpNumberOfBytesRead
        0               // lpOverlapped
    );

    /*
        int test = 0;
        int i = 0;
        for (i = 0; i < BYTE_BUFFER_SIZE; i++) {
            test = buffer[i];
        }
    */

    /**/
    int pc = 0x0100;

    unsigned char op_code = 0;
    int temp_addr = 0;
    int temp_compute = 0;
    int i = 0;

    int a_idx = 0;
    int b_idx = 0;

    // accumulator A, 8-bit register
    char A = 0; // initially uninitialized, but here it is set to zero

    // stack pointer register, 16-bit register
    short SP = 0; // initially uninitialized, but here it is set to zero

    int z_flag = 1; // initially true

    for (i = 0; i < 20; i++) {

        op_code = buffer[pc];

        print_unsigned_char_numeric_decimal(op_code);

        // https://meganesu.github.io/generate-gb-opcodes/
        switch (op_code) {

        case 0x00:
            //print_string("[NOP]");

            pc = pc + 1;
            break;

        case 0x20:
            //print_string("[JR]");

            // JR NZ, s8

            a_idx = pc + 1;
            char jump_offset = buffer[a_idx];

            pc = pc + 2;

            if (z_flag == 0) {
                pc = pc + jump_offset;
            }

            //if (z_flag == 0) {
            //    pc = pc + jump_offset;
            //} else {
            //    pc = pc + 2;
            //}
            break;

        case 0x31: // 49dec
            //print_string("[LD] 7.3 16-bit load instructions\n");

            // 7.3 16-bit load instructions
            // LD rr, nn // general form
            // LD SP, nn // this specific form (register rr is hardset to SP. SP is the stack pointer register)
            // the two bytes following the opcode are nn. nn is a 16 bit value. nn is stored MSB
            // Load to the 16-bit register rr, the immediate 16-bit data nn.
            // Here, the register

            a_idx = pc + 1;
            b_idx = pc + 2;
            temp_addr = 0;
            temp_addr = (buffer[b_idx] << 8) + buffer[a_idx];

            SP = temp_addr;

            // TODO: load temp into stack pointer register SP

            pc = pc + 3;
            break;

        case 0xAF:
            print_string("Wurst");
            break;

        case 0xC3: // 195dec
            //print_string("[JP] JP");

            temp_addr = 0;

            //temp = (buffer[pc+2] << 8) + buffer[pc+1];
            //pc = temp;

            //if (temp == 0x0150) {
            //    print_string("Korrekt");
            //} else {
            //    print_string("Inkorrekt");
            //}

            a_idx = pc + 1;
            b_idx = pc + 2;
            temp_addr = 0;
            temp_addr = (buffer[b_idx] << 8) + buffer[a_idx];

            //if (temp == 0x0150) {
            //    print_string("Korrekt");
            //} else {
            //    print_string("Inkorrekt");
            //}

            pc = temp_addr;
            break;

        case 0xF0:
            //print_string("[LDH] LDH A, (n): Load accumulator (direct 0xFF00+n)");

            //
            // reads one byte (n) from memory
            // extends the byte to 0xFF00+n
            // treats the extended immediate 16-bit value as an address
            // loads a byte from said address and transfers the byte into the 8-bit register A (Accumulator)
            //

            a_idx = pc + 1;
            temp_addr = 0;
            temp_addr = (0xFF << 8) + buffer[a_idx];

            //printf("temp_addr %d\n", temp_addr);

            // load byte from memory at address temp
            A = buffer[temp_addr];

            if (A == 0x00) {
                print_string("Korrekt A");
            } else {
                print_string("Inkorrekt A");
                //print_unsigned_char_numeric_decimal(A);
            }

            pc = pc + 2;
            break;

        case 0xF3: // 243dec
            //print_string("[HALT] Halt system clock");

            pc = pc + 1;
            break;

        case 0xFE: // 254dec
            //print_string("[CP] Compare (immediate)");

            // CP n: Compare (immediate)

            // Subtracts from the 8-bit A register, the immediate data n,
            // and updates flags based on the result.
            //
            // This instruction is basically identical to SUB n, but does not update the A register.

            a_idx = pc + 1;
            temp_addr = 0;
            temp_compute = buffer[a_idx];

            temp_compute = A - temp_compute;

            pc = pc + 2;
            break;

        default:
            print_string("Unknown");
            break;
        }
    }

    print_string("end");
    return 0;
}
