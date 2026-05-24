/*
Übersicht mit KI
The ARM Thumb pseudo-instruction ADD r0, r1 adds the contents of register r1 to r0, storing the result in r0. 
It is commonly assembled into a 16-bit ADDS r0, r0, r1 (setting condition flags) when using low registers (r0 - r7) in Thumb mode to perform . 

Syntax: ADD Rd, Rm (equivalent to ADD Rd, Rd, Rm)
Action: r0 = r0 + r1

Thumb-1 (16-bit): Usually generates ADDS r0, r0, r1. This requires the registers to be low registers (r0 - r7).
Thumb-2 (32-bit): Can be encoded to use high registers (r8 - r15) if necessary, but 16-bit is preferred.
Flags: When translated to ADDS, the flags are updated. 

If you are using high registers, it may assemble to a different 32-bit instruction or fail, depending on the architecture version. 
*/

ADD r0, r1