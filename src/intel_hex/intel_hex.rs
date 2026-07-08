use std::collections::BTreeMap;

use crate::common::file_handling::read_lines;
use crate::common::hex::{decode_hex, encode_hex};

const START_ADDRESS: u32 = 0x00800000;

pub fn load_intel_hex(memory_block_map: &mut BTreeMap<u32, Vec<u8>>, filepath: &'static str) -> u32 {

    let debug: bool = false;

    let mut _start_segment: u32 = START_ADDRESS;

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
    //if let Ok(lines) = read_lines("C:/Users/lapto/dev/stm8/stm8/examples/uart/bin/uart.hex") {
    if let Ok(lines) = read_lines(filepath) {

        //let mut current_offset: u32 = 0x00800000; // for ARM Cortex-M4, always assume the default address 0x08000000
        let mut current_offset: u32 = 0x00000000;

        let mut segment_started: bool = false;

        // Consumes the iterator, returns an (Optional) String
        for line in lines.map_while(Result::ok) {

            // DEBUG
            if debug {
                println!("{}", line);
            }

            // remove colon
            let without_colon = &line[1..];

            let data = decode_hex(without_colon).unwrap();

            // DEBUG
            if debug {
                println!("{:02x?}", data);
            }

            //let test:u16 = data[1] << 8 | data[0];
            //let line_offset = ((data[1] as u32) << 24) | ((data[0] as u32) << 16) | ((0 as u32) << 8) | ((0 as u32) << 0);
            //let line_offset = ((data[1] as u32) << 8) | ((data[2] as u32) << 0);

            // find offset of the current line
            let line_offset = ((0 as u32) << 24) | ((0 as u32) << 16) | ((data[1] as u32) << 8) | ((data[2] as u32) << 0);
            if debug {
                println!("{:02x?}", line_offset);
            }

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

                        if !memory_block_map.contains_key(&current_offset) {
                            let memory_block: Vec<u8> = vec![0; 0xFFFF];
                            memory_block_map.insert(current_offset, memory_block);
                        }

                        segment_started = true;
                    }

                    let memory_block: &mut Vec<u8> = memory_block_map.get_mut(&current_offset).unwrap();

                    let mut i = 0;
                    for idx in 4 .. 4 + payload_length {

                        if debug {
                            println!("addr: 0x{:02x}, data: 0x{:02x}", (low_part + i), data[idx as usize]);
                        }

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

                    if !memory_block_map.contains_key(&current_offset) {
                        let memory_block: Vec<u8> = vec![0; 0xFFFF];
                        memory_block_map.insert(current_offset, memory_block);
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
                    memory_block_map.insert(current_offset, memory_block);

                    segment_started = true;

                    //println!("memory_blocks_2 {:?}", memory_blocks_2);
                }

                _ => {
                    todo!();
                }
            }
        }

        //println!("memory_block_map {:02x?}", memory_block_map);
    }

    _start_segment
}