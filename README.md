# Proxy

Edit .cargo/config.toml to activate or deactivate the proxy.

# General Information

https://keleshev.com/compiling-to-assembly-from-scratch/07-arm-assembly-programming

# Tools

## Thumb Online Encoder / Decoder

https://shell-storm.org/online/Online-Assembler-and-Disassembler/
https://armconverter.com/

## Thumb Online Simulators

https://bkhmsi.github.io/ARMThumb_Sim/#/
https://freddyjs.github.io/wthumb/
https://wunkolo.github.io/OakSim/

# Adding a new Mnemonic

1. Update C:\aaa_se\rust\rust_pest_asm\src\asm.pest and add the new mnemonic to the grammar
1. Udpate C:\aaa_se\rust\rust_pest_asm\src\ast\instruction.rs (3 changes required)
1. Update C:\aaa_se\rust\rust_pest_asm\src\encoder\thumb\thumb_decoder.rs (Update the decoding for the new instruction here)
1. Update C:\aaa_se\rust\rust_pest_asm\src\cpu\cortex_m4.rs. Add the mnemonic execution here.

# The Boot Process

https://embetronicx.com/tutorials/microcontrollers/stm32/reset-sequence-in-arm-cortex-m4/
https://vivonomicon.com/2018/04/02/bare-metal-stm32-programming-part-1-hello-arm/

https://www.reddit.com/r/arm/comments/aifq0e/load_view_vs_execution_view_linker_script_and/
https://medium.com/@ragagr116/what-happens-before-main-understanding-the-startup-file-on-arm-cortex-m-46c9f55e1a6b

https://www.st.com/resource/en/product_training/STM32F7_Memory_Flash.pdf

The VTOR register contains the address of the vector table. 
The vector table contains the address of the reset handler, which is the code that the Cortex-M4 will jump to.

The VTOR register is not persisted! Instead it is reset! The VTOR register does not survive power cycles!
On reset (= power cycle. power down, power up) the VTOR register is reset to 0x00000000.
Therefore the vector table will always be at address 0x00000000 after a power cycle. 

PM0214 Programming manual, STM32 Cortex®-M4 MCUs and MPUs programming manual
Page 40:

> On system reset, the vector table is fixed at address 0x00000000. Privileged software can write to the VTOR to relocate the vector table start address to a different memory location, in the range 0x00000080 to 0x3FFFFF80. For further information see Vector table offsetregister (VTOR) on page 227.

For some reason, looking at programming tutorials for STM chips, the vector table is always placed at 0x08000000 instead of at 0x00000000.
The reason might be:

On STM32 Cortex-M4 devices, the vector table is physically located at 0x08000000 (beginning of Flash) but is aliased or mapped to 0x00000000 at reset.

ARM creates the Cortex-M4 which is only a specification but not a real chip. STM builds real chips that implement ARM's Cortex-M4. So STM has to
adhere to the ARM specification yet still create a real chip with real memory. STM use a flash memory chip and maps that flash memory to 0x08000000
and part of that flash chip also to 0x00000000 of the address space.

This means that most linker files for STM are organized so that the code is located to the flash only, meaning the address 0x08000000 is used for the vector table.
But the Cortex-M4 still searches the vector table at 0x00000000. The address 0x00000000 cannot be flashed by a .hex file since the flash memory is
located at 0x08000000. Therefore the vector table in flash is mapped to 0x00000000 by processor logic. (I DO NOT KNOW HOW MUCH IS MAPPED, so what is the end 
address of the mapped area?)

The emulator / simulator also has to map part of flash from 0x08000000 to 0x00000000. How large is that part? The question is which area is mapped!
When the simulated Cortex-M4 accesses 0x00000000 it has to be rerouted to 0x08000000.

INCORRECT: Also on reset (power cycle), the vector table will get it's default content! 
CORRECT: The vector table is defined by the .hex file that is flashed onto the microcontroller!
This means there is no default vector table as defined by the microcontroller's logic, instead the vector table's content is application specific and
defined by the user! This is the reason why the manual does only list the structure and offsets of the vector table but there is no default values 
defined for the entries anywhere!

The default reset-handler address is 0x????????, whichever is defined by the application binary that is flashed onto the chip.
This means, the microcontroller will jump to the reset handler address defined in the application binary to execute the reset handler code from there.

While the system is powered (no power cycle), the VTOR register can be manipulated by software to change the address of the vector table.
A completely new vector table can be written to the new address. This process is called, relocating the vector table!
Also another option is to change the values inside the default vector table at the default VTOR address (0x00000000) during runtime!

If no power-cycle happens but the system remains powered and a soft-reset
is executed, the microcontroller will look at the VTOR register and jump to the modified address in the VTOR register
and then execute the reset handler in the modified vector table.

The code inside the reset handler is user-defined by the application binary.

Why would you ever change the contents of VTOR or the content of the vector table if it is not persistent and reset after a power-cycle?
The answer is that the microcontroller can be reset without a hard power-cycle. Such a reset is called soft-reset or warm reset in the programmers manual.
Although the VTOR register and the vector table are not persisted, they will survive a soft-reset!

The use case might be a bootloader. After a hard-reset, VTOR is updated and the vector table is relocated away from 0x00000000.
The bootloader takes the position of the default vector table at 0x00000000. Then, when it is time to transfer new firmware to the system, 
the system is first soft-reset so that the bootloader is still present and the vector table is still relocated. 

After soft-reset, the microcontroller finds the vector table via VTOR and executes the reset_handler. 
The reset_handler jumps to the bootloader. The bootloader will start. It will wait some time if there is an uploader application trying to 
push new firmware into the bootloader. If there is none, the bootloader will timeout and start the normal main() function of the old firmware.
If the boot loader detects an uploader, it will talk to the uploader instead of jumping to the old firmware and it will accept the new firmware 
via UART or some other media from the uploader. Once uplaoded, the bootloader executes the main() function of the new firmware.

The steps during boot (after soft- or hard-reset) are:

1. (Hardwired Logic) Microntroller will look at the VTOR register and retrieve the address of the vector table
2. (Hardwired Logic) The first element of the vector table contains the stackpointer address which is loaded into the stack pointer register
3. The next word in the vector table contains the reset-handler
4. (Hardwired Logic) The microcontroller will jump to the reset handler's address and execute normally often with the code from the startup.s file.
5. startup.s sets up all clocks and peripherals and copies Copy the Initialized global variable, and static variable (.data) to SRAM, Copy the Un-initialized data (.bss) to SRAM and initialize it to 0. 
6. After setup, startup.s calls main()
7. main() executes application code


# Bootloader

What happens when you have a bootloader?
When your project has a bootloader, then you will be having two binaries. One is a Bootloader image and another one is an application. The bootloader will be placed in the 0x00000000 with its Vector Table. The application will be placed in another area of the flash memory with its vector table.

1. When you press the reset button, it will start from the bootloader. So, it will copy the Stack pointer to MSP (Main Stack Pointer) from the location of 0x00000000.
2. Then it moves to the bootloader’s reset handler.
3. The bootloader will do some operations based on your need like check for firmware version, upgrade the firmware, etc.
4. Once it has done with its operation, it will jump to the application’s vector table.
5. Then it will initialize the MSP (Main Stack Pointer) again.
6. Now we have to tell the controller to use the application’s vector table instead of using the bootloader’s vector table. That will be done using the Vector Table Offset Register (VTOR).
7. Then it goes to the application’s reset handler. There it will copy and initialize the data segments to the RAM (Global, Static variables).
8. Finally, it moves to the main function of the application.

If you want to learn about the bootloader, then we have provided a separate bootloader series for you. Please check it out.
https://embetronicx.com/bootloader-tutorials/

# Example Startup Test Application

HINT: In order to look at the .elf file in elf-viewer, switch to THUMB on the disassembly tab.

Q: The reset handler address inside the vector table will be 0x08000009 although it should be 0x08000008.
The code for the reset handler will correctly be generated at 0x08000008. 
Why is the address in the vector table for the reset ahndler seemingly misaligend and points one byte after the correct address?

A: https://stackoverflow.com/questions/40841852/arm-vector-table-pointing-one-byte-after
The address is different because ARM uses it to switch to THUMB mode.
> Answer: ARM uses it for switching to THUMB mode.

To switch to Thumb mode in an ARM reset handler, you must set the Least Significant Bit (LSB) of 
the address in the reset vector table to 1, or use the BX (Branch and Exchange) instruction to 
branch to a target address with the LSB set, such as BX r0 where r0 is odd.

0x0800000[8] = 00000000 00001000 00000000 00000000 00000000 00000000 00000000 00001000
0x0800000[9] = 00000000 00001000 00000000 00000000 00000000 00000000 00000000 00001001 <--- THUMB SWITCH BIT IS SET

The test application is taken from the following page:
https://vivonomicon.com/2018/04/02/bare-metal-stm32-programming-part-1-hello-arm/
https://vivonomicon.com/2018/04/20/bare-metal-stm32-programming-part-2-making-it-to-main/

It is contained in the folder: res/C/samples/reset_handler

In the Makefile, update the path to the installation of the arm-none-eabi toolchain.
The Makefile will create a .bin file amongst other files.

1. Load that bin file to 0x08000000 and mirror 0x08000000 to 0x00000000.
2. Let the Cortex-M4 emulator run. It will execute the reset_vector from 0x00000004.
3. Inspect the reset handler's address inside the vector table. If the lowest bit is set, switch to ARM and toggle the lowest bit. 
Then jump to that modified reset_handler address.
4. Execute the reset_handler in THUMB mode if activated. There is no setup code and no jump to main(). The entire application is located inside the reset_handler.

# Encoding

https://armconverter.com/

https://developer.arm.com/documentation/ddi0406/c/Application-Level-Architecture/Thumb-Instruction-Set-Encoding/32-bit-Thumb-instruction-encoding
https://developer.arm.com/documentation/ddi0406/c/Application-Level-Architecture/Thumb-Instruction-Set-Encoding/16-bit-Thumb-instruction-encoding?lang=en

Upper six bit is the opcode, lower 10 bit depends on operand.

02 48 - 000000 1001001000

48 02 - 000100 1001000000


https://developer.arm.com/documentation/107829/0201/What-is-assembly-language-/How-assembly-code-works

https://upload.wikimedia.org/wikiversity/en/7/74/ARM.2ASM.Thumb.20231223.pdf


## Example (46 08, mov r0, r1)

```
d = UInt(D:Rd);  m = UInt(Rm);  setflags = FALSE;
if d == 15 && InITBlock() && !LastInITBlock() then UNPREDICTABLE;
```

1. The Rd register is formed from the D bit concatenated with the Rd bits both unsigned.
2. The Rm register is formed from the unsigned Rm bits.
3. No flags are changed by mov.
4. If <Rd> is the PC, must be outside or last in IT block.

```
                ----------------- D
                |
                |  -------------- Rm -- r1
                |  |
                |  |  ----------- Rd -- r0
                |  |  |
                -****+++
46 08 - 0100011000001000]
        ------****++++++
           |    |    |
           |    |    ------------- (cref. [3]) - Encoding T1 - MOV<c> <Rd>, <Rm>
           |    |
           |    ------------------ sub-opcode 1000 within opcode 010001 - Move Low Registers - (cref. [2])
           |
           ----------------------- opcode (010001 - Special data instructions and branch and exchange) - (cref. [1])
```

[1] https://developer.arm.com/documentation/ddi0406/c/Application-Level-Architecture/Thumb-Instruction-Set-Encoding/16-bit-Thumb-instruction-encoding?lang=en
[2] https://developer.arm.com/documentation/ddi0406/c/Application-Level-Architecture/Thumb-Instruction-Set-Encoding/16-bit-Thumb-instruction-encoding/Special-data-instructions-and-branch-and-exchange?lang=en
[3] https://developer.arm.com/documentation/ddi0406/c/Application-Level-Architecture/Instruction-Details/Alphabetical-list-of-instructions/MOV--register--Thumb-?lang=en



When D is set to 1, 46 88 - mov r8, r1



# Decoding

Copy the bytes from the .bin file (res\C\samples\reset_handler\main.bin)

00 10 00 20 09 00 00 08 02 48 85 46 02 4F 00 20 01 30 FD E7 00 10 00 20 EF BE AD DE

Then use a disassembler (https://armconverter.com/?disasm) or objectdump the .elf file.

```
-- vector table
0x8000000: 00 10 00 20      -- 0x20001000, Address to load into the Main Stack Pointer 
                            -- (See res\C\samples\reset_handler\STM32F031K6T6.ld, line 6)
0x8000004: 09 00 00 08      -- 0x08000009, Reset Handler address with THUMB bit set, 
                            -- real address would be 08 00 00 08 which is eight byte after the Main Stack Pointer 
                            -- value which was inserted at 0x08000000.

-- Usually the vector table contains several more entries, but they are all unused in this sample.
-- Instead of other addresses for excption/interrupt handlers, here is the code for the reset_handler, 
-- which contains the entire application

-- After jumping to the reset handler, PC is set to the address of the first instruction in the reset handler which is 0x8000008

// Set the stack pointer to the end of the stack.
// The '_estack' value is defined in our linker script.
8000008: 02 48              -- ldr r0, [pc, #8]     -- [pc, #8] is a PC relative address. 
                                                    -- This means PC+8. PC is 0x8000008 right now. 0x8000008 + 8 = 
800000a: 85 46              -- mov sp, r0          

// Set some dummy values. When we see these values
// in our debugger, we'll know that our program
// is loaded on the chip and working.
800000c: 02 4F              -- ldr r7, [pc, #8]     -- [pc, #8] is a PC relative address. PC is 0x8000000C + 8 = 0x8000014
800000e: 00 20              -- movs r0, #0          

8000010: 01 30              -- adds r0, #1

// Jump back to the absolute address 0x08
8000012: FD E7              -- b [pc, #-2]                 -- [pc, #-2] is PC-relative

-- Starting from here, this is not real source code any more!
-- I do not know what these values are! The disassemblers will decode these bytes incorrectly!
-- The bytes ressemble the main stack pointer from line 1 exactly! I wonder if this is the main stack pointer.
8000014: 00 10 00 20        -- address of stack begin -- This is an address (DO NOT INTERPRET AS CODE!)
8000018: EF BE AD DE        -- data (0xDEADBEEF)      -- This is data (DO NOT INTERPRET AS CODE!)
```

Here is the objectdump of the .elf file:

```
Disassembly of section .text:

08000000 <vtable>:
 8000000:	20001000 	andcs	r1, r0, r0
 8000004:	08000009 	stmdaeq	r0, {r0, r3}

08000008 <reset_handler>:
 8000008:	4802      	ldr	r0, [pc, #8]	; (8000014 <main_loop+0x4>)
 800000a:	4685      	mov	sp, r0
 800000c:	4f02      	ldr	r7, [pc, #8]	; (8000018 <main_loop+0x8>)
 800000e:	2000      	movs	r0, #0

08000010 <main_loop>:
 8000010:	3001      	adds	r0, #1
 8000012:	e7fd      	b.n	8000010 <main_loop>
 8000014:	20001000 	andcs	r1, r0, r0
 8000018:	deadbeef 	cdple	14, 10, cr11, cr13, cr15, {7}
```

# GDB

https://medium.com/virtuslab/integrating-gdb-support-in-an-emulator-ef41ff13f301

# Instructions

The STM32 microcontroller is internally an ARM Cortex-M4 microcontroller with all STM32 peripherals added.
Therefore the instructions are not defined primarily through the STM documentation but rather via the ARM documentation: https://developer.arm.com/documentation/ddi0596/2020-12/Base-Instructions

But there is a STM document describing the programmers model: https://www.st.com/resource/en/programming_manual/pm0214-stm32-cortexm4-mcus-and-mpus-programming-manual-stmicroelectronics.pdf




# Implementation of a Simulator

Read as big endian (MSB is stored last and needs to be placed first when building the number in RAM).

1. Load a .hex file into several byte arrays, one array per Extended Linear Address Record (Type 04).
2. Load 4 byte value from address 0x80000000 into the main stack pointer
3. Load 4 byte value from address 0x80000004 into u32 variable.
4. If the LSB bit is set => activate THUMB mode. Else NotImplementedException.
5. Remove the LSB from the u32 variable.
6. Load u32 variable into PC
7. Start executing (Read 2 byte thumb instruction from PC and decode and execute.)




# Decoding a Real Application

Die Hex-Datei an Adresse 0x08000000 enthält den interrupt vector.
Die Adresse 0x08000000 in der .hex-Datei findet man zunächste über den Extended Linear Address Record (Type 04) für das High Word (0800)

:02000004[0800]F2

```
:020000040800F2
:10|0000|00[F8E501208DD3030897D30308B5D30308]7F
:10|0010|00[D3D30308F1D303080FD4030800000000]72
:10|0020|00[000000000000000000000000F1300008]A7
```



Hinweis: Die Adressen lassen sich mit Hilfe der .map Datei prüfen.

1. Lies F4650120 als big endian (MSB is stored last and needs to be placed first when building the number) -> 200165F4. Dieser Wert ist der Wert des Main-Stackpointer.

2. Lies 6D420308 als big endian -> 0803426D. Entferne das ARM THUMB bit. 0803426D -> 0803426C. 
Das THUMB Bit wird gesetzt um den Chip in den THUMB-Modus zu schalten, damit er den THUMB code verarbeitet, den der Compiler generiert hat.
Das ist die Adresse des ResetHandler. Springe zum Reset Handler.
8DD30308 -> 0803D38D --> [0803][D38D]
Entferne das THUMB-Activation bit: 0803D38D -> 0803D38C
D38C als 16 Bit-Grenze: 0xD38C -> 110100111000[1100] -> 110100111000[0000] -> 0xD380

In Zeile: 15677:

```
:10[D380]00|F8D90120 F8E50120 88ED00E0 [08B5][FFF7]|A5
```


Suche in der Hex Datei die Adresse des ResetHandler 0x0803426c.

1. Suche den Extended Linear Address Record (04) für das High-Word der Adresse 0x0803
:020000040803EF in Zeile 12292.

```
:020000040803EF

:    	- Marker
02   	- Länge der Payload
0000 	- Load Offset
04   	- Typ (Extended Linear Address Record)
0803 	- Payload
EF   	- CHKSUM
```

Innerhalb der folgenden Datenzeilen (:10) finde das Low-Word der Adresse 0x426c.

Da 0x426c keine 16 byte aligned Adresse ist, muss man die am nahe liegendste 16 Byte Grenze suchen.
Diese Grenze ist 0x4260

```
:10426000F4590120F465012088ED00E008B5FFF75E

:		- Marker
10		- Länge der Payload (0x10 == 16 byte)
4260	- Load Offset
00		- Typ (Data Record)
F4590120F465012088ED00E008B5FFF7 	- Payload
5E		- CHKSUM
```

Innerhalb der Payload befindet sich der Offset 0x0C am 12 byte

F4590120 F4650120 88ED00E0 [08B5][FFF7]

Das heißt der Reset Handler beginnt mit der THUMB Anweisung 08B5.
Dies wird Big-Endian interpretiert: 08B5 -> B508

Um genau zu wissen, was die Bytes bedeuten, wird die .elf Datei mit Objektdump dekodiert.

```
cd C:/aaa_se/wdp/build_DEBUG_fse_122_rtos_md/src
C:\aaa_se\sdk_toolchain\arm-none-eabi\bin\arm-none-eabi-objdump -D output.elf > output.lst
```

Quellcode für den Reset-Handler (C:\aaa_se\wdp\subprojects\sdk-core\src\targets\stm\stm32l4\stm32l451re\compiler\toolchain_gcc\startup.c)

```
void Reset_Handler(void)
{
    // STL startup code
#if(SDK_CONF_STL_PRESENT > 0U)
    #if(SDK_CONF_RTOS_PRESENT > 0U)
        OSLowLevelInit();
    #endif /* (SDK_CONF_RTOS_PRESENT > 0U) */
    SystemInit(); /* CMSIS System Initialization */
    STL_StartUp();
#endif // end (SDK_CONF_STL_PRESENT > 0)

    SystemInit(); /* CMSIS System Initialization */
    _start(); /* Enter PreMain (C library entry point) */
}
```

Hier ist der dekodierte Reset-Handler:

```
0803426c <Reset_Handler>:
 803426c:	b508      	push	{r3, lr}
 803426e:	f7ff fe09 	bl	8033e84 <SystemInit>
 8034272:	f7ff ffbd 	bl	80341f0 <_start>
```



## The push Instruction

push {r3, lr} means that a set of registers ({} notation) is pushed to the stack.
Here the R3 and the LR register are pushed to the stack.
I do not know why r3, lr are pushed.

push and pop instructions have a speciality:

https://stackoverflow.com/questions/13686353/thumb-push-pop-instructions

> "There is one caveat, arguments to push/pop/ldm/stm are in ascending register order, not in the order specified. 
So if you do push{r0,r1} and then pop{r1,r0} intending to swap them, this will fail because pop{r1,r0} is identical to pop{r0,r1}"

This means push and pop will always reorder their arguments and not operate as the programmer has specified in all cases!




## The BL instructions

https://developer.arm.com/documentation/dui0489/i/arm-and-thumb-instructions/bl

> "The BL instruction causes a branch to label, and copies the address of the next instruction into LR (R14, the link register)."

Using the BL instruction, the code will jump to / call / execute the <SystemInit> function followed by the <_start> function.

According to [MARTIN2016131], the SystemInit function initializes PLLs and the clock tree inside the Microcontroller.




# System Init Function

C:\aaa_se\wdp\subprojects\sdk-core\src\targets\stm\stm32l4\stm32l451re\fse_122\source\TGT_Clk.c

```
/**
  * @brief  Setup the microcontroller system
  *         Initialize the FPU setting, vector table location and External memory
  *         configuration.
  * @param  None
  * @retval None
  */
void SystemInit(void)
{
    /* FPU settings ------------------------------------------------------------*/
#if (__FPU_PRESENT == 1) && (__FPU_USED == 1)
    SCB->CPACR |= ((3UL << 10 * 2) | (3UL << 11 * 2)); /* set CP10 and CP11 Full Access */
#endif
    /* Reset the RCC clock configuration to the default reset state ------------*/
    /* Set HSION bit */
    RCC->CR |= (uint32_t)0x00000001;

    /* Reset CFGR register */
    RCC->CFGR = 0x00000000;

    /* Reset HSEON, CSSON and PLLON bits */
    RCC->CR &= (uint32_t)0xFEF6FFFF;

    /* Reset PLLCFGR register */
    RCC->PLLCFGR = 0x24003010;

    /* Reset HSEBYP bit */
    RCC->CR &= (uint32_t)0xFFFBFFFF;

    /* Disable all interrupts */
    RCC->CICR = 0x00000000;

#if defined (DATA_IN_ExtSRAM) || defined (DATA_IN_ExtSDRAM)
    SystemInit_ExtMemCtl();
#endif /* DATA_IN_ExtSRAM || DATA_IN_ExtSDRAM */

}
```

This is the assembly code for SystemInit()

```
08033e84 <SystemInit>:
8033e84:	4a0d      	ldr	r2, [pc, #52]	; (8033ebc <SystemInit+0x38>)
8033e86:	f8d2 3088 	ldr.w	r3, [r2, #136]	; 0x88
8033e8a:	f443 0370 	orr.w	r3, r3, #15728640	; 0xf00000
8033e8e:	f8c2 3088 	str.w	r3, [r2, #136]	; 0x88
8033e92:	4b0b      	ldr	r3, [pc, #44]	; (8033ec0 <SystemInit+0x3c>)
8033e94:	681a      	ldr	r2, [r3, #0]
8033e96:	f042 0201 	orr.w	r2, r2, #1
8033e9a:	601a      	str	r2, [r3, #0]
8033e9c:	2100      	movs	r1, #0
8033e9e:	6099      	str	r1, [r3, #8]
8033ea0:	681a      	ldr	r2, [r3, #0]
8033ea2:	f022 7284 	bic.w	r2, r2, #17301504	; 0x1080000
8033ea6:	f422 3280 	bic.w	r2, r2, #65536	; 0x10000
8033eaa:	601a      	str	r2, [r3, #0]
8033eac:	4a05      	ldr	r2, [pc, #20]	; (8033ec4 <SystemInit+0x40>)
8033eae:	60da      	str	r2, [r3, #12]
8033eb0:	681a      	ldr	r2, [r3, #0]
8033eb2:	f422 2280 	bic.w	r2, r2, #262144	; 0x40000
8033eb6:	601a      	str	r2, [r3, #0]
8033eb8:	6219      	str	r1, [r3, #32]
8033eba:	4770      	bx	lr
8033ebc:	e000ed00 	and	lr, r0, r0, lsl #26
8033ec0:	40021000 	andmi	r1, r2, r0
8033ec4:	24003010 	strcs	r3, [r0], #-16
```

## FPU Settings

```
/* FPU settings ------------------------------------------------------------*/
#if (__FPU_PRESENT == 1) && (__FPU_USED == 1)
    SCB->CPACR |= ((3UL << 10 * 2) | (3UL << 11 * 2)); /* set CP10 and CP11 Full Access */
#endif
```

The FPU is documented in the "Programming Manual" [PM0214], page 252, section "4.6 Floating point unit (FPU)"
CPACR is the Coprocessor access control register (CPACR). It is documented on page 253.

The SCB (system control block) has the address 0xE000ED00.
The CPACR has a SCB-relative offset of 0x88 and the absolute address: 0xE000ED00 + 0x88 = 0xE000ED88

Using the bits CP10 and CP11, full access to the co-processor is granted.

In the assembly code, the address of the SCB (0xE000ED00) is stored as a constant at the end of the function.

```
8033ebc:	e000ed00 	and	lr, r0, r0, lsl #26
```

Although objectdump provides a instruction for this value, the value is an address (data) and not code.
Just ignore the and instruction, it will not be executed.

First, this hardcoded value is transferred into the r2 register using a pc relative offset that points
to the hardcoded address data.

```
8033e84:	4a0d      	ldr	r2, [pc, #52]	; (8033ebc <SystemInit+0x38>)
```

Next, the current value of the SCB->CPACR register, which is stored in memory, is loaded into the r2 register by
using a ldr.w instruction.

```
8033e86:	f8d2 3088 	ldr.w	r3, [r2, #136]	; 0x88
````

This instruction uses the offset for the CPACR register, which is 0x88, in the second
argument to the ldr.w instruction. This means that the offset 0x88 is first applied to the
value stored in r2. r2 contains teh SCB address and after the offset is applied the resulting
address points to SCB->CPACR. The word-value at this memory-cell is transferred into register r3.

```
8033e86:	f8d2 3088 	ldr.w	r3, [r2, #136]	; 0x88
```

r3 is OR-ed with the constant 0xf00000. This is how the new values for the CP10 and CP11 are set.

```
8033e8a:	f443 0370 	orr.w	r3, r3, #15728640	; 0xf00000
```

The temporary value in r3 is then written back into the SCB->CPACR register using a str.w instruction.

```
8033e8e:	f8c2 3088 	str.w	r3, [r2, #136]	; 0x88
```




## Clock Configuration

```
/* Reset the RCC clock configuration to the default reset state ------------*/
/* Set HSION bit */
RCC->CR |= (uint32_t)0x00000001;
```

RCC is the "Reset and Clock Control" peripheral. It is part of STM32 chips (No documentation in ARM).
It is documented in the [RM0394] on page 179.

page 197 of [RM0394] is where the description of the memory-mapped registers for the RCC start.
The RCC base address is 0x40021000.

```
#define PERIPH_BASE           (0x40000000UL) /*!< Peripheral base address */
#define AHB1PERIPH_BASE       (PERIPH_BASE + 0x00020000UL)
#define RCC_BASE              (AHB1PERIPH_BASE + 0x1000UL)
```

RCC_CR is the Clock control register (RCC_CR). 
It has an RCC-relative offset of 0x00. 0x40021000 + 0x00 = 0x40021000.

RCC has also a typedef for the C programming language:

```
typedef struct
{
  __IO uint32_t CR;          /*!< RCC clock control register,                                              Address offset: 0x00 */
  __IO uint32_t ICSCR;       /*!< RCC internal clock sources calibration register,                         Address offset: 0x04 */
  __IO uint32_t CFGR;        /*!< RCC clock configuration register,                                        Address offset: 0x08 */
  __IO uint32_t PLLCFGR;     /*!< RCC system PLL configuration register,                                   Address offset: 0x0C */
  __IO uint32_t PLLSAI1CFGR; /*!< RCC PLL SAI1 configuration register,                                     Address offset: 0x10 */
  uint32_t      RESERVED;    /*!< Reserved,                                                                Address offset: 0x14 */
  __IO uint32_t CIER;        /*!< RCC clock interrupt enable register,                                     Address offset: 0x18 */
  __IO uint32_t CIFR;        /*!< RCC clock interrupt flag register,                                       Address offset: 0x1C */
  __IO uint32_t CICR;        /*!< RCC clock interrupt clear register,                                      Address offset: 0x20 */
  uint32_t      RESERVED0;   /*!< Reserved,                                                                Address offset: 0x24 */
  __IO uint32_t AHB1RSTR;    /*!< RCC AHB1 peripheral reset register,                                      Address offset: 0x28 */
  __IO uint32_t AHB2RSTR;    /*!< RCC AHB2 peripheral reset register,                                      Address offset: 0x2C */
  __IO uint32_t AHB3RSTR;    /*!< RCC AHB3 peripheral reset register,                                      Address offset: 0x30 */
  uint32_t      RESERVED1;   /*!< Reserved,                                                                Address offset: 0x34 */
  __IO uint32_t APB1RSTR1;   /*!< RCC APB1 peripheral reset register 1,                                    Address offset: 0x38 */
  __IO uint32_t APB1RSTR2;   /*!< RCC APB1 peripheral reset register 2,                                    Address offset: 0x3C */
  __IO uint32_t APB2RSTR;    /*!< RCC APB2 peripheral reset register,                                      Address offset: 0x40 */
  uint32_t      RESERVED2;   /*!< Reserved,                                                                Address offset: 0x44 */
  __IO uint32_t AHB1ENR;     /*!< RCC AHB1 peripheral clocks enable register,                              Address offset: 0x48 */
  __IO uint32_t AHB2ENR;     /*!< RCC AHB2 peripheral clocks enable register,                              Address offset: 0x4C */
  __IO uint32_t AHB3ENR;     /*!< RCC AHB3 peripheral clocks enable register,                              Address offset: 0x50 */
  uint32_t      RESERVED3;   /*!< Reserved,                                                                Address offset: 0x54 */
  __IO uint32_t APB1ENR1;    /*!< RCC APB1 peripheral clocks enable register 1,                            Address offset: 0x58 */
  __IO uint32_t APB1ENR2;    /*!< RCC APB1 peripheral clocks enable register 2,                            Address offset: 0x5C */
  __IO uint32_t APB2ENR;     /*!< RCC APB2 peripheral clocks enable register,                              Address offset: 0x60 */
  uint32_t      RESERVED4;   /*!< Reserved,                                                                Address offset: 0x64 */
  __IO uint32_t AHB1SMENR;   /*!< RCC AHB1 peripheral clocks enable in sleep and stop modes register,      Address offset: 0x68 */
  __IO uint32_t AHB2SMENR;   /*!< RCC AHB2 peripheral clocks enable in sleep and stop modes register,      Address offset: 0x6C */
  __IO uint32_t AHB3SMENR;   /*!< RCC AHB3 peripheral clocks enable in sleep and stop modes register,      Address offset: 0x70 */
  uint32_t      RESERVED5;   /*!< Reserved,                                                                Address offset: 0x74 */
  __IO uint32_t APB1SMENR1;  /*!< RCC APB1 peripheral clocks enable in sleep mode and stop modes register 1, Address offset: 0x78 */
  __IO uint32_t APB1SMENR2;  /*!< RCC APB1 peripheral clocks enable in sleep mode and stop modes register 2, Address offset: 0x7C */
  __IO uint32_t APB2SMENR;   /*!< RCC APB2 peripheral clocks enable in sleep mode and stop modes register, Address offset: 0x80 */
  uint32_t      RESERVED6;   /*!< Reserved,                                                                Address offset: 0x84 */
  __IO uint32_t CCIPR;       /*!< RCC peripherals independent clock configuration register,                Address offset: 0x88 */
  uint32_t      RESERVED7;   /*!< Reserved,                                                                Address offset: 0x8C */
  __IO uint32_t BDCR;        /*!< RCC backup domain control register,                                      Address offset: 0x90 */
  __IO uint32_t CSR;         /*!< RCC clock control & status register,                                     Address offset: 0x94 */
  __IO uint32_t CRRCR;       /*!< RCC clock recovery RC register,                                          Address offset: 0x98 */
  __IO uint32_t CCIPR2;      /*!< RCC peripherals independent clock configuration register 2,              Address offset: 0x9C */
} RCC_TypeDef;
```

The last bit of RCC->CR is set. The last bit is called MSION (cref [RM0394], page 197).
The MSION bit controls MSI clock enable. MSI clock enable is used to either turn the MSI oscillator ON or OFF.
A value of 1 turns the MSI oscillator on.

The comment on the call says: /* Set HSION bit */ but I think this is a typo because the line sets the MSION bit.

The MSI clock is documented in the section "6.2.3 MSI clock" (cref [RM0394], page 187).

"The MSI clock is used as system clock after restart from Reset, wakeup from Standby and
Shutdown low-power modes. After restart from Reset, the MSI frequency is set to its default
value 4 MHz. Refer to Section 6.3: Low-power modes."

Looking at the clocktree on [RM0394], page 184, it can be seen that the MSI can be used as 
a source for the SYSCLK signal (center of the figure 13, clock tree). SYSCLK is the main clock
signal for the microcontroller's CPU.

Apparently in the default coniguration, the MSI has a frequency of 4 MHz. This could be tested 
using an oscilloscope.









# Sources

[RM0394]
RM0394
Reference manual
STM32L41xxx/42xxx/43xxx/44xxx/45xxx/46xxx advanced Arm®-based 32-bit MCUs

[PM0214]
PM0214
Programming manual
STM32 Cortex®-M4 MCUs and MPUs programming manual

@incollection{MARTIN2016131,
title = {Chapter 4 - Cortex Microcontroller Software Interface Standard},
editor = {Trevor Martin},
booktitle = {The Designer's Guide to the Cortex-M Processor Family (Second Edition)},
publisher = {Newnes},
edition = {Second Edition},
pages = {131-153},
year = {2016},
isbn = {978-0-08-100629-0},
doi = {https://doi.org/10.1016/B978-0-08-100629-0.00004-9},
url = {https://www.sciencedirect.com/science/article/pii/B9780081006290000049},
author = {Trevor Martin},
keywords = {CMSIS-Core, CMSIS-DSP, CMIS-Pack, CMSIS-SVD, CMSIS-DAP, CMSIS-RTOS, CMSIS-Driver},
abstract = {Like desktop computing the software complexity of embedded applications is increasing exponentially. Now more than ever, developers are using third-party code to meet project deadlines. ARM has defined Cortex Microcontroller Software Interface standard (CMSIS) that allows easy integration of source code from multiple sources. CMSIS now has wide support throughout the industry and should be adopted for new projects.}
}







# Rust

```
// https://doc.rust-lang.org/book/ch08-03-hash-maps.html

let memory_block_8000 = vec![0x00, 0x01, 0x02, 0x03];

let mut memory_blocks = HashMap::new();

memory_blocks.insert(0x8000, memory_block_8000);

let mut bl = memory_blocks.get_mut(&0x8000).unwrap();

println!("bl {:?}", bl);

bl[0] = 0xFF;

println!("bl {:?}", bl);

let mut bl2 = memory_blocks.get_mut(&0x8000).unwrap();

println!("bl2 {:?}", bl2);
```