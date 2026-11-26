# Division

For x64, there is a div and an idiv instruction.

div covers unsigned integer division.
idiv covers signed integer division.

Both integer division operators both compute the
quotient and the remainder at the same time!

The key to using the instructions is to know where
to place the parameters and where to expect the
quotient and the remainder!

## div - unsigned integer division

| Divisor-Größe  | Dividend (Input)   | Quotient (Output)   | Rest (Output)    |
| -------------- | ------------------ | ------------------- | ---------------- |
| 8 Bit (r/m8)	 | AX (16 Bit)	      | AL	                | AH               |
| 16 Bit (r/m16) | DX:AX (32 Bit)	  | AX	                | DX               |
| 32 Bit (r/m32) | EDX:EAX (64 Bit)	  | EAX	                | EDX              |
| 64 Bit (r/m64) | RDX:RAX (128 Bit)  | RAX	                | RDX              |

NOTE:
There is a irregulaty in the outputs! For 8 bit, quotient and remainder both go into the A-register!
For all other widths, the A and D register is used!


## idiv - signed integer division

| Divisor Size	 | Dividend	(Input)   | Quotient (Output)   | Remainder	(Output) | Sign-Extension Prep |
| -------------- | ------------------ | ------------------- | ------------------ | ------------------- |
| 8-bit (r/m8)	 | AX	              | AL	                | AH	             | CBW (AL → AX)       |
| 16-bit (r/m16) | DX:AX	          | AX	                | DX	             | CWD (AX → DX:AX)    |
| 32-bit (r/m32) | EDX:EAX	          | EAX	                | EDX	             | CDQ (EAX → EDX:EAX) |
| 64-bit (r/m64) | RDX:RAX	          | RAX	                | RDX	             | CQO (RAX → RDX:RAX) |

NOTE:
There is a irregulaty in the outputs! For 8 bit, quotient and remainder both go into the A-register!
For all other widths, the A and D register is used!

The sign-extension commands will convert between two types and while doing so, keep the sign.

The commands are:

* CBW (Convert Byte to Word)
* CWD (Convert Word to DoubleWord)
* CDQ (Convert DoubleWord to QuadWord)
* CQO (Convert QuadWord to Octaword)

they all perform sign extension.