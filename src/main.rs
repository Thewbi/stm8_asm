/*****************************************************************
 *
 * This main file parses a .hex binary file.
 * If you want to process a .asm file, use the main.rs_asm file.
 *
******************************************************************/

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use std::collections::HashMap;

mod ast;

mod encoder;

mod cpu;
use crate::cpu::cortex_m4::cortex_m4::CortexM4;
use crate::cpu::stm8::stm8::STM8;

use crate::cpu::mem_access::read_word_le;

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

use std::{fmt::Write, num::ParseIntError};

pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

pub fn encode_hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        write!(&mut s, "{:02x}", b).unwrap();
    }
    s
}

fn main() {

    // address where the CPU starts executing from
    // for ARM Cortex-M4, always assume the default address 0x08000000
    // This value is overriden by the type (03) record (Start Segment Address Record)
    //let mut _start_segment: u32 = cpu::cortex_m4::cortex_m4::START_ADDRESS;
    let mut _start_segment: u32 = cpu::stm8::stm8::START_ADDRESS;

    let mut memory_blocks = HashMap::new();

    //if let Ok(lines) = read_lines("C:/aaa_se/wdp/build_DEBUG_fse_122_rtos_md/src/output.hex") {
    //if let Ok(lines) = read_lines("res/wdp/output.hex") {
    //if let Ok(lines) = read_lines("res/samples/assembler_tutorial/first_program/a.hex") {
    //if let Ok(lines) = read_lines("res/samples/assembler_tutorial/conditions_and_branches/a.hex") {
    //if let Ok(lines) = read_lines("res/samples/assembler_tutorial/loops_with_branches/a.hex") {
    //if let Ok(lines) = read_lines("res/instructions_pseudo/ldr/a.hex") {
    //if let Ok(lines) = read_lines("res/samples/assembler_tutorial/printing_strings_to_terminal/a.hex") {
    //if let Ok(lines) = read_lines("res/C/samples/loop_example/sum.ihx") {
    //if let Ok(lines) = read_lines("res/C/samples/loop_example_2/sum.ihx") {
    //if let Ok(lines) = read_lines("res/C/Users/lapto/dev/stm8/stm8/examples/uart/bin/uart.ihx") {
    if let Ok(lines) = read_lines("C:/Users/lapto/dev/stm8/stm8/examples/uart/bin/uart.hex") {

        //let mut current_offset: u32 = 0x00800000; // for ARM Cortex-M4, always assume the default address 0x08000000
        let mut current_offset: u32 = 0x00000000;

        let mut segment_started: bool = false;

        // Consumes the iterator, returns an (Optional) String
        for line in lines.map_while(Result::ok) {

            // DEBUG
            println!("{}", line);

            // remove colon
            let without_colon = &line[1..];

            let data = decode_hex(without_colon).unwrap();

            // DEBUG
            println!("{:02x?}", data);

            //let test:u16 = data[1] << 8 | data[0];
            //let line_offset = ((data[1] as u32) << 24) | ((data[0] as u32) << 16) | ((0 as u32) << 8) | ((0 as u32) << 0);
            //let line_offset = ((data[1] as u32) << 8) | ((data[2] as u32) << 0);

            // find offset of the current line
            let line_offset = ((0 as u32) << 24) | ((0 as u32) << 16) | ((data[1] as u32) << 8) | ((data[2] as u32) << 0);
            println!("{:02x?}", line_offset);

            // iHex Format: [: (1 Byte)] [LEN (1 Byte)] [LOAD_OFFSET (2 Byte)] [TYPE (1 Byte)] [DATA (LEN Byte)] [CHKSUM]

            let payload_length = data[0];
            //let offset = ((data[1] as u16) << 8) | data[2] as u16;
            let record_type = data[3];

            // println!("payload_length: {:02x?}, offset: {:02x?}, record_type: {:02x?}", payload_length, offset, record_type);

            match record_type {

                // Data Record
                0x00 => {

                    // build high and low part of the current line offset
                    let high_part: u32 = line_offset & 0xFFFF0000;
                    let low_part: u32 = line_offset & 0x0000FFFF;

                    //println!("current_offset: {:?}", current_offset);
                    //println!("memory_blocks_2 {:?}", memory_blocks_2);

                    // some hex files do start with data lines without prior segment records!
                    // In this case, add a memory_block to stire the data in
                    if !segment_started {

                        // use the offset of the current line as global offset
                        current_offset = high_part;

                        if !memory_blocks.contains_key(&current_offset) {
                            //let memory_block: Vec<u8> = Vec::new(0, FFFF);
                            let memory_block: Vec<u8> = vec![0; 0xFFFF];
                            memory_blocks.insert(current_offset, memory_block);
                        }

                        segment_started = true;
                    }

                    let memory_block: &mut Vec<u8> = memory_blocks.get_mut(&current_offset).unwrap();

                    let mut i = 0;
                    for idx in 4 .. 4 + payload_length {

                        println!("addr: 0x{:02x}, data: 0x{:02x}", (low_part + i), data[idx as usize]);

                        //memory_block.push(data[ idx as usize ]);
                        memory_block[(low_part + i) as usize] = data[idx as usize];

                        i = i + 1;
                    }
                }

                // End of File Record
                0x01 => {
                    println!("(01) End of File");
                    segment_started = false;
                    current_offset = 0;
                }

                // Extended Segment Address Record
                0x02 => {
                    // Die im Datenfeld enthaltene Adresse wird dabei um 4 Bit nach links verschoben (entsprechend einer Multiplikation mit
                    // 2^4 = 16) und bei den folgenden Data Records (Typ 00) zu den dort enthaltenen 16-Bit-Adressen addiert.
                    // Der Extended Segment Address Record bleibt bis zur Änderung durch einen anderen Extended Segment Address Record wirksam.

                    println!("(02) Extended Segment Address Record");
                    // println!("(02) New Segment");
                    //current_offset = ((data[5] as u32) << 24) | ((data[4] as u32) << 16) | ((0 as u32) << 8) | ((0 as u32) << 0);
                    //current_offset = ((data[4] as u32) << 24) | ((data[5] as u32) << 16) | ((0 as u32) << 8) | ((0 as u32) << 0);
                    current_offset = ((0 as u32) << 24) | ((0 as u32) << 16) | ((data[4] as u32) << 8) | ((data[5] as u32) << 0);
                    current_offset = current_offset << 4;
                    println!("(02) New Segment Offset: {:02x?}", current_offset);

                    if !memory_blocks.contains_key(&current_offset) {
                        let memory_block: Vec<u8> = vec![0; 0xFFFF];
                        memory_blocks.insert(current_offset, memory_block);
                    }

                    segment_started = true;
                }

                // Start Segment Address Record
                0x03 => {
                    println!("(03) Start Segment Address Record");
                    _start_segment = ((data[4] as u32) << 24) | ((data[5] as u32) << 16) | ((data[6] as u32) << 8) | ((data[7] as u32) << 0);
                }

                // Extended Linear Address Record
                0x04 => {
                    println!("(04) New Segment");
                    current_offset = ((data[4] as u32) << 24) | ((data[5] as u32) << 16) | ((0 as u32) << 8) | ((0 as u32) << 0);
                    println!("(04) New Segment Offset: {:02x?}", current_offset);

                    let memory_block: Vec<u8> = Vec::new();
                    memory_blocks.insert(current_offset, memory_block);

                    segment_started = true;

                    //println!("memory_blocks_2 {:?}", memory_blocks_2);
                }

                _ => {
                    todo!();
                }
            }
        }

        //println!("memory_blocks {:02x?}", memory_blocks);
    }

    println!("_start_segment {:02x?}", _start_segment);

    let mut _main_stack_pointer_value: u32 = 0;
    let mut _reset_handler_address: u32 = 0;

    // for hex files compiled using a STM32 linker script
    // stack_pointer value is stored at 0x08000000
    // reset handler address is stored at 0x08000004
    // The STM32 core will read four bytes, place them into the stack pointer (this is not implemented as ASM code but it is hardwired into the CPU)
    // then it will execute the next four byte as a jump to the reset handler
    //
    //let load_real_hex_application: bool = true;

    // for small assembler scripts without real STM32 cpu
    // there is just code located at 0x08000000
    // The simulated CPU should just execute the instructions located at 0x08000000 and not execute and hardwired CPU logic for the first bytes!
    let load_real_cortexm4_hex_application: bool = false;
    if load_real_cortexm4_hex_application {

        // The Cortex CPU starts operating from 0x08000000
        // This address contains the interrupt vector table
        // The first double word (4 byte) contain the value that needs to go into the main stack pointer
        // The next double word (4 byte) contain the address of the reset handler
        // The Cortex CPU jumps to the reset handler and executes it after power up / cold reset and after a hot reset

        //
        // read the address of the main stack pointer from 0x08000000
        //

        //let memory_block = memory_blocks.get(&0x08000000).unwrap();

        // DEBUG
        //
        // let data_byte_0 = memory_block[0];
        // let data_byte_1 = memory_block[1];
        // let data_byte_2 = memory_block[2];
        // let data_byte_3 = memory_block[3];
        // println!("data_byte_0 {:02x?}", data_byte_0);
        // println!("data_byte_1 {:02x?}", data_byte_1);
        // println!("data_byte_2 {:02x?}", data_byte_2);
        // println!("data_byte_3 {:02x?}", data_byte_3);

        _main_stack_pointer_value = read_word_le(&memory_blocks, _start_segment);
        println!("main_stack_pointer_value: {:02x?}", _main_stack_pointer_value);

        _reset_handler_address = read_word_le(&memory_blocks, _start_segment + 4);
        println!("reset_handler_address: {:02x?}", _reset_handler_address);

        // detect THUMB
        if _reset_handler_address % 2 == 1 {
            println!("Thumb Detected!");
            // remove the THUMB bit
            _reset_handler_address = _reset_handler_address - 1;
        } else {
            todo!()
        }

    } else {

        println!("HEX File results: +++++++++++++++++");
        for (key, value) in &memory_blocks {

            //println!("{}: {}", key, value);
            println!("0x{:02x?}", key);

            for i in 0..0xFFFF {
                let byte_val = value[i as usize];
                if byte_val != 0 {
                    println!("{:02x?}: {:02x?} ", (key + i), byte_val);
                }
            }

            println!("");
        }
        println!("+++++++++++++++++++++++++++++++++++");

        println!("_start_segment: 0x{:02x?}", _start_segment);
        let high_part: u32 = _start_segment & 0xFFFF0000;
        let _low_part: u32 = _start_segment & 0x0000FFFF;

        let _memory_block = memory_blocks.get(&high_part).unwrap();
        _reset_handler_address = _start_segment;
        println!("reset_handler_address: {:02x?}", _reset_handler_address);

    }

    //
    // Select the CPU
    //

    //let mut cpu: CortexM4 = CortexM4::new();
    let mut cpu: STM8 = STM8::new();
    cpu.memory_blocks = memory_blocks;
    cpu.set_pc(_reset_handler_address);

    while !cpu.halt() {
        cpu.step();
    }

    println!("Done");
}
