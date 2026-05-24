TOOLCHAIN = C:\aaa_se\sdk_toolchain\arm-none-eabi\bin

CC = ${TOOLCHAIN}\arm-none-eabi-gcc.exe
OD = ${TOOLCHAIN}\arm-none-eabi-objdump.exe
OC = ${TOOLCHAIN}\arm-none-eabi-objcopy

PARAMS = -O0 -nostdinc -nostdlib -ffreestanding -mthumb -mcpu=cortex-m4

.PHONY: clearscr clean ihex listing assemble all

all: clearscr assemble ihex listing

assemble: printing_strings_to_terminal.s
	${CC} ${PARAMS} printing_strings_to_terminal.s

ihex:
	${OD} -D a.out > output.lst

listing:
	${OC} -O ihex a.out a.hex

clean:
	rm -f a.out a.hex output.lst

clearscr:
	cls
