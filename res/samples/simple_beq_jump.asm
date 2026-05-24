; This a simple example in arm thumb!

.text ; Start of .text section. This is where the code will be placed.
  mov r0, #5
  mov r1, #5
;  add r0, r1
  add r0, r0, r1
  cmp r0, #10
  beq stop

  mov r0, #0
  mov r1, #0

stop:  wfi # This makes the CPU sleep until an interrupt arrives (Which will never arrive ...)