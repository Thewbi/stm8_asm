# Perihperals

## Datasheet for STM8S207R8T6

STM8S207xx STM8S208xx

STM8S207R8T6-STMicroelectronics-datasheet-181247560.pdf

page 34, lists the address 0x5000 as base address for GPIO and peripheral registers.


## Clock Divider Register (CLK_CKDIVR), page 94 STM8 reference manual.

The reference manual RM0016 for the STM8S Series and STM8AF Series 8-bit microcontrollers on page 30 says:

"3 Memory and register map
For details on the memory map, I/O port hardware register map and CPU/SWIM/debug
module/interrupt controller registers, refer to the product datasheets."

This means that in order to know which offset the Clock registers have, it is required to
consult the chip's data sheet.

For the STM8S207R8T6, page 37 of the data sheet, lists all CLK registers:

0x005000 - base address for peripheral registers
0x0050C0 - base address for clock peripheral registers

| CLK Register Addr | Reg          | Details                                 | Reset Value |
| ----------------- | ------------ | --------------------------------------- | ----------- |
| 0x50C0            | CLK_ICKR     | Internal clock control register         | 0x01        |
| 0x50C1            | CLK_ECKR     | External clock control register         | 0x00        |
| 0x50C2            |              | Reserved (1 byte)                       |             |
| 0x50C3            | CLK_CMSR     | Clock master status register            | 0xE1        |
| 0x50C4            | CLK_SWR      | Clock master switch register            | 0xE1        |
| 0x50C5            | CLK_SWCR     | Clock switch control register           | 0xXX        |
| 0x50C6            | CLK_CKDIVR   | Clock divider register                  | 0x18        |
| 0x50C7            | CLK_PCKENR1  | Peripheral clock gating register 1      | 0xFF        |
| 0x50C8            | CLK_CSSR     | Clock security system register          | 0x00        |
| 0x50C9            | CLK_CCOR     | Configurable clock control register     | 0x00        |
| 0x50CA            | CLK_PCKENR2  | Peripheral clock gating register 2      | 0xFF        |
| 0x50CB            | CLK_CANCCR   | CAN clock control register              | 0x00        |
| 0x50CC            | CLK_HSITRIMR | HSI clock calibration trimming register | 0x00        |
| 0x50CD            | CLK_SWIMCCR  | SWIM clock control register             | 0bXXXXXXX0  |


/** Clock Divider Register (RS=0x18) */
#define CLK_CKDIVR		*(unsigned char*)0x50C6

When a value of 0 is set,
- the High speed internal clock prescaler (HSI) is set to prescale of no division.
  The frequency is not divided at all
- the CPU clock prescaler has no division applied. The CPU clock is the same as the master clock.
- Effectively the HSI and CPU are running on max frequency.

The reset value is 0x18



# UART1 Peripheral

## Register Addresses

| CLK Register Addr | Reg          | Details                                 | Reset Value |
| ----------------- | ------------ | --------------------------------------- | ----------- |
| 0x00 5230         | UART1_SR     | UART1 status register                   | 0xC0        |
| 0x00 5231         | UART1_DR     | UART1 data register                     | 0xXX        |
| 0x00 5232         | UART1_BRR1   | UART1 baud rate register 1              | 0x00        |
| 0x00 5233         | UART1_BRR2   | UART1 baud rate register 2              | 0x00        |
| 0x00 5234         | UART1_CR1    | UART1 control register 1                | 0x00        |
| 0x00 5235         | UART1_CR2    | UART1 control register 2                | 0x00        |
| 0x00 5236         | UART1_CR3    | UART1 control register 3                | 0x00        |
| 0x00 5237         | UART1_CR4    | UART1 control register 4                | 0x00        |
| 0x00 5238         | UART1_CR5    | UART1 control register 5                | 0x00        |
| 0x00 5239         | UART1_GTR    | UART1 guard time register               | 0x00        |
| 0x00 523A         | UART1_PSCR   | UART1 prescaler register                | 0x00        |


## Configuring UART1 Peripheral (page 323)

### Configuration Procedure

Procedure:
1. Program the M bit in UART_CR1 to define the word length.
   (Hint: The register reset values automatically default to 8N1)
2. Program the number of stop bits in UART_CR3.
   (Hint: The register reset values automatically default to 8N1, 1 stop bit)
3. Select the desired baud rate by programming the baud rate registers in the following order:
    a: UART_BRR2
    b: UART_BRR1
    (Hint: Using a 16 MHz master clock UART_BRR2 = 0x03 and USART1_BRR1 = 0x68 yields
    see page 336, UART_DIV = 0x0683. 16000000/0x0683 = 9598 ~ 9600 baud)
4. Set the TEN (transmit enable) bit in UART_CR2 to enable transmitter mode.
5.
    a: Wait until the TXE bit inside the UART1_SR register is zero. TXE=0 means that
    The TXE bit is set by hardware and it indicates:
        • The data has been moved from TDR to the shift register and the data transmission has started.
        • The TDR register is empty.
        • The next data can be written in the UART_DR register without overwriting the previous data.
    b: Write the data to send in the UART_DR register (this clears the TXE bit. The TXE
    bit remains 0 (cleared) as long as the transmission of the word is ongoing. Once the
    TXE bit turns 1 again, the next word can be placed into UART_DR).
    Repeat this for each data to be transmitted in case of single buffer.
6. Once the last data is written to the UART_DR register, wait until TC is set to ‘1’, which
indicates that the last data transmission is complete.
This last step is required, for instance, to avoid last data transmission corruption when disabling the UART or entering Halt mode.


### TX / RX enable

The register USART1_CR2 contains two bits to enable transmission and reception for
the UART1 peripheral.

To enable UART1 TX, set the bit 3.
To enable UART1 RX, set the bit 2.

### Word Length - M - Bits in UART_CR1

Word = here: one transmitted character

Word Length = here: Amount of bits per character

22.7.5 Control register 1 (UART_CR1), page 366

Bit 4 M: word length.
This bit determines the word length. It is set or cleared by software.
0: 1 Start bit, 8 Data bits, n Stop bit (n depending on STOP[1:0] bits in the UART_CR3 register)
1: 1 Start bit, 9 Data bits, 1 Stop bit

Note: The M bit must not be modified during a data transfer (both transmission and reception)
In LIN slave mode, the M bit and the STOP[1:0] bits in the UART_CR3 register should be kept at 0.


22.7.7 Control register 3 (UART_CR3), page 369

Bits 5:4 STOP: STOP bits.
These bits are used for programming the stop bits.
00: 1 Stop bit
01: Reserved
10: 2 Stop bits
11: 1.5 Stop bits
Note: For LIN slave mode, both bits should be kept cleared.

### Stop Bits

The register USART1_CR3 has bits 5 and 4, which determine the amount of stop bits.

00 - no stop bits
01 - 1 Stop bits
10 - 1.5 Stop bits
11 - 2 Stop bits

### Baud Rate (page 336)

Select the desired baud rate by programming the baud rate registers in the following order:
1. UART_BRR2
2. UART_BRR1

USART1_BRR2

The divisor is constructed like this

```
UART_DIV = { UART_BRR2[7:4], UART_BRR1[7:0], UART_BRR2[3:0] }
```

The formula for the resulting baud rate is (page 336):

```
Tx / Rx baud rate = fMASTER / UART_DIV
```

fMaster = MasterClock

## Single Byte Communication (page 330)

Single byte communication
Clearing the TXE bit is always performed by a write to the data register.

The TXE bit is set by hardware and it indicates:
• The data has been moved from TDR to the shift register and the data transmission has
started.
• The TDR register is empty.
• The next data can be written in the UART_DR register without overwriting the previous
data.