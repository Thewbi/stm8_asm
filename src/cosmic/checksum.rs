use std::collections::BTreeMap;

use crate::cpu::mem_access::read_byte;

use crate::common::bit_handling::rol_u16;

pub fn compute_cosmic_crc16(memory_block_map: &mut BTreeMap<u32, Vec<u8>>, start_address: u32, end_address: u32, invert: bool) -> u16 {

    let debug: bool = false;
    //let debug: bool = true;

    //let address: u32 = 0x8000;
    //let end_address: u32 = 0x8F08;

    //let address: u32 = 0x8000;
    //let end_address: u32 = 0x162b7;

    //let _address: u32 = 0x8000;
    //let _end_address: u32 = 0x16170;

    let mut crc16: u16 = 0;

    let mut k = 0;
    let mut _first_row: bool = true;

    //for i in 0..0xF08 {
    //for i in 0..0xE170 {
    //for i in 0..0x6170 { // no 9c67
    //for temp_address in 0x8000..0x16170 { // no 9c67
    //for temp_address in 0x8000..0x1617d {
    //for i in 0..0x80 {

    if debug {
        println!("START compute_cosmic_crc16. START_ADDRESS: 0x{:08x}, END_ADDRESS: 0x{:08x}", start_address, end_address);
    }

    for temp_address in start_address..end_address {

        if debug {
            if k % 16 == 0 {
                println!("");
                print!("{:08x} ", temp_address);
            }
        }

        // load next byte
        //let temp_address: u32 = address + i;
        
        // if temp_address == 0x10000 {
        //     println!("Addr: temp_address 0x{:08x?}", temp_address);
        // }

        let data_byte: u8 = read_byte(&memory_block_map, temp_address);

        //println!("0x{:08x} {:02x}  ", temp_address, data_byte);

        //if debug {
            print!("{:02x} ({})dec ", data_byte, data_byte);
        //}

        // circular shift left
        //println!("BEFORE: crc16 {:16b}", crc16);
        crc16 = rol_u16(crc16, 1);
        //println!("BEFORE: crc16 {:16b}", crc16);

        // xor the loaded byte into the CRC16 
        //println!("XOR:    crc16 {:16b}, data_byte {:8b}", crc16, data_byte);
        crc16 = crc16 ^ (data_byte as u16);
        //println!("XOR:    crc16 {:16b}", crc16);
        println!("XOR: crc16 {:04x}, ({})dec", crc16, crc16);

        // if k % 16 == 0 && !first_row {
        //     println!("");
        // }

        // first_row = false;

        k = k + 1;
    }

    if debug {
        println!("");
    }

    //
    // compute the one's complement
    //

    if debug {
        println!("Before Inverting Result (Binary):         crc16 {:16b}", crc16);
        println!("Before Inverting Result (Hex):            crc16 {:04x}", crc16);
    }

    if invert {
   
        crc16 = !crc16;

        if debug {
            // this is the CRC16 as output by the COSMIC toolchain
            println!("After Inverting Result (Binary):          crc16 {:16b}", crc16);    
            println!("After Inverting Result (Hex):             crc16 {:04x}", crc16); 
        }
    }

    if debug {
        println!("END compute_cosmic_crc16");
    }

    // this is the CRC16 as output by the COSMIC toolchain
    crc16
}