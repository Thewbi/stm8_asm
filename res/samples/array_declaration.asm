// source: https://godbolt.org/

// int main() {
//	
//	int temp_array[3] = { 1, 2, 3 };	
//	temp_array[1] = 17;
//	
//	return 0;
//}

.LC0:
        .word   1
        .word   2
        .word   3
main:
        str     fp, [sp, #-4]!
        add     fp, sp, #0
        sub     sp, sp, #20
        ldr     r2, .L3
        sub     r3, fp, #16
        ldm     r2, {r0, r1, r2}
        stm     r3, {r0, r1, r2}
        mov     r3, #17
        str     r3, [fp, #-12]
        mov     r3, #0
        mov     r0, r3
        add     sp, fp, #0
        ldr     fp, [sp], #4
        bx      lr
.L3:
        .word   .LC0