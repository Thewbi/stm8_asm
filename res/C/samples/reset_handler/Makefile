# Makefile for compiling ARM Cortex-M0 assembly projects.
TARGET = main

# Define the linker script location and chip architecture.
LD_SCRIPT = STM32F031K6T6.ld
MCU_SPEC = cortex-m4

# Toolchain definitions (ARM bare metal defaults)
TOOLCHAIN = C:\aaa_se\sdk_toolchain\arm-none-eabi
CC = $(TOOLCHAIN)\bin\arm-none-eabi-gcc
AS = $(TOOLCHAIN)\bin\arm-none-eabi-as
LD = $(TOOLCHAIN)\bin\arm-none-eabi-ld
OC = $(TOOLCHAIN)\bin\arm-none-eabi-objcopy
OD = $(TOOLCHAIN)\bin\arm-none-eabi-objdump
OS = $(TOOLCHAIN)\bin\arm-none-eabi-size
NM = $(TOOLCHAIN)\bin\arm-none-eabi-gcc-nm

# Assembly directives.
ASFLAGS += -c
ASFLAGS += -O0
ASFLAGS += -mcpu=$(MCU_SPEC)
ASFLAGS += -mthumb
ASFLAGS += -Wall
# (Set error messages to appear on a single line.)
ASFLAGS += -fmessage-length=0

# Linker directives.
LSCRIPT = ./$(LD_SCRIPT)
LFLAGS += -mcpu=$(MCU_SPEC)
LFLAGS += -mthumb
LFLAGS += -Wall
LFLAGS += --specs=nosys.specs
LFLAGS += -nostdlib
LFLAGS += -lgcc
#LFLAGS += -nostartfiles
#LFLAGS += -static
LFLAGS += -T$(LSCRIPT)

AS_SRC += ./core.S

OBJS =  $(AS_SRC:.S=.o)

.PHONY: all
all: $(TARGET).elf $(TARGET).hex $(TARGET).bin $(TARGET).nm $(TARGET).lst

# compile all .S files into object files
%.o: %.S
	$(CC) -x assembler-with-cpp $(ASFLAGS) $< -o $@

# compile all object files into the final .elf output file
.PHONY: elf
elf: $(TARGET).elf
$(TARGET).elf: $(OBJS)
	$(CC) $^ $(LFLAGS) -o $@
	$(OS) $<

# create .hex output file	
.PHONY: hex
hex: $(TARGET).hex
$(TARGET).hex: $(TARGET).elf
	$(OC) -O ihex $(TARGET).elf $(TARGET).hex
#	$(OS) $<

# create .bin output file
# The bin file contains the raw bytes to place at 0x08000000 (STM flash) or 0x00000000 (ARM Cortex-M4 start of code memory)
# STM maps 0x08000000 to 0x00000000
.PHONY: bin
$(TARGET).bin: $(TARGET).elf
	$(OC) -S -O binary $< $@
#	$(OS) $<

# nm will dump the final addresses for all functions and symbols as text
# This text is the written into a .nm file which is really a plain text file
.PHONY: nm
nm: $(TARGET).nm
$(TARGET).nm: core.o
	$(NM) core.o > $(TARGET).nm
#	$(OS) $<

.PHONY: lst
lst: $(TARGET).lst
$(TARGET).lst: main.elf
	$(OD) -D main.elf > $(TARGET).lst

.PHONY: clean
clean:
	rm -f $(OBJS)
	rm -f $(TARGET).elf
	rm -f $(TARGET).hex
	rm -f $(TARGET).bin
	rm -f $(TARGET).nm
	rm -f $(TARGET).lst