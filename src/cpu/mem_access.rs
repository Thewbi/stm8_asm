use std::collections::HashMap;

pub fn read_byte(memory_block_map: &HashMap<u32, Vec<u8>>, address: u32) -> u8 {

    // for (key, value) in memory_block_map {
    //     //println!("{}: {}", key, value);
    //     println!("0x{:02x?}", key);
    // }

    // DEBUG
    //println!("{:02x?}", address);

    let high_part: u32 = address & 0xFFFF0000;
    let low_part: u32 = address & 0x0000FFFF;

    let low_part: usize = low_part.try_into().unwrap();

    let memory_block = memory_block_map.get(&high_part).unwrap();

    // the loader accesses a half-word and the next half-word since some instructions
    // need an entire word even in thumb or thumb-2. At the very end of the .hex file,
    // the loader should not access the half-word past the end of the file. If it
    // does, return 0
    if low_part < memory_block.len() {
        //u16::from_le_bytes(memory_block[low_part..low_part+2].try_into().unwrap())
        memory_block[low_part] as u8
    } else {
        0
    }
}

pub fn write_byte(memory_block_map: &mut HashMap<u32, Vec<u8>>, address: u32, data: u8) {

    //println!("write_byte() {:08x?} = {}", address, data);

    // for (key, value) in memory_block_map {
    //     //println!("{}: {}", key, value);
    //     println!("0x{:02x?}", key);
    // }

    // DEBUG
    //println!("{:02x?}", address);

    let high_part: u32 = address & 0xFFFF0000;
    let low_part: u32 = address & 0x0000FFFF;

    let low_part: usize = low_part.try_into().unwrap();

    //let memory_block_ve8 = memory_block_map.get(&high_part).unwrap();
    //let memory_block_ve8: &mut Vec<u8> = memory_block_map.get(&high_part).unwrap();

    let memory_block_ve8 = memory_block_map.get_mut(&high_part).unwrap();

    //println!("BEFORE> {:08x?} = {:08x?} ({})dec", address, memory_block_ve8[low_part], memory_block_ve8[low_part]);

    memory_block_ve8[low_part] = data;

    //println!("AFTER > {:08x?} = {:08x?} ({})dec", address, memory_block_ve8[low_part], memory_block_ve8[low_part]);

}

pub fn write_halfword(memory_block_map: &mut HashMap<u32, Vec<u8>>, address: u32, data: u16) {

    //println!("write_halfword() {:08x?} = {}", address, data);

    // for (key, value) in memory_block_map {
    //     //println!("{}: {}", key, value);
    //     println!("0x{:02x?}", key);
    // }

    // DEBUG
    //println!("{:02x?}", address);

    let high_part: u32 = address & 0xFFFF0000;
    let low_part: u32 = address & 0x0000FFFF;

    let low_part: usize = low_part.try_into().unwrap();

    //let memory_block_ve8 = memory_block_map.get(&high_part).unwrap();
    //let memory_block_ve8: &mut Vec<u8> = memory_block_map.get(&high_part).unwrap();

    let memory_block_ve8 = memory_block_map.get_mut(&high_part).unwrap();

    //println!("BEFORE> {:08x?} = {:08x?} ({})dec", address, memory_block_ve8[low_part], memory_block_ve8[low_part]);

    memory_block_ve8[low_part] = ((data >> 8) & 0xFF) as u8;
    memory_block_ve8[low_part + 1] = ((data >> 0) & 0xFF) as u8;

    //println!("AFTER > {:08x?} = {:08x?} ({})dec", address, memory_block_ve8[low_part], memory_block_ve8[low_part]);
}

/**
 * Little Endian (u16 from vector)
 */
pub fn read_halfword_le(memory_block_map: &HashMap<u32, Vec<u8>>, address: u32) -> u16 {

    // for (key, value) in memory_block_map {
    //     //println!("{}: {}", key, value);
    //     println!("0x{:02x?}", key);
    // }

    // DEBUG
    //println!("{:02x?}", address);

    let high_part: u32 = address & 0xFFFF0000;
    let low_part: u32 = address & 0x0000FFFF;

    let low_part: usize = low_part.try_into().unwrap();

    let memory_block = memory_block_map.get(&high_part).unwrap();

    // the loader accesses a half-word and the next half-word since some instructions
    // need an entire word even in thumb or thumb-2. At the very end of the .hex file,
    // the loader should not access the half-word past the end of the file. If it
    // does, return 0
    if low_part < memory_block.len() {
        u16::from_le_bytes(memory_block[low_part..low_part+2].try_into().unwrap())
    } else {
        0
    }
}

pub fn read_halfword_be(memory_block_map: &HashMap<u32, Vec<u8>>, address: u32) -> u16 {

    // for (key, value) in memory_block_map {
    //     //println!("{}: {}", key, value);
    //     println!("0x{:02x?}", key);
    // }

    // DEBUG
    //println!("{:02x?}", address);

    let high_part: u32 = address & 0xFFFF0000;
    let low_part: u32 = address & 0x0000FFFF;

    let low_part: usize = low_part.try_into().unwrap();

    let memory_block = memory_block_map.get(&high_part).unwrap();

    // the loader accesses a half-word and the next half-word since some instructions
    // need an entire word even in thumb or thumb-2. At the very end of the .hex file,
    // the loader should not access the half-word past the end of the file. If it
    // does, return 0
    if low_part < memory_block.len() {
        u16::from_be_bytes(memory_block[low_part..low_part+2].try_into().unwrap())
    } else {
        0
    }
}

/**
 * Little Endian (u32 from vector)
 */
pub fn read_word_le(memory_block_map: &HashMap<u32, Vec<u8>>, address: u32) -> u32 {

    let high_part: u32 = address & 0xFFFF0000;
    let low_part: u32 = address & 0x0000FFFF;

    let low_part: usize = low_part.try_into().unwrap();

    let memory_block = memory_block_map.get(&high_part).unwrap();

    let val: u32 = ((memory_block[low_part+3] as u32) << 24) | ((memory_block[low_part+2] as u32) << 16) | ((memory_block[low_part+1] as u32) << 8) | ((memory_block[low_part+0] as u32) << 0);

    //println!("{:02x}", val);

    val
}

pub fn read_word_be(memory_block_map: &HashMap<u32, Vec<u8>>, address: u32) -> u32 {

    let high_part: u32 = address & 0xFFFF0000;
    let low_part: u32 = address & 0x0000FFFF;

    let low_part: usize = low_part.try_into().unwrap();

    let memory_block = memory_block_map.get(&high_part).unwrap();

    let val: u32 = ((memory_block[low_part+0] as u32) << 24) | ((memory_block[low_part+1] as u32) << 16) | ((memory_block[low_part+2] as u32) << 8) | ((memory_block[low_part+3] as u32) << 0);

    //println!("{:02x}", val);

    val
}
