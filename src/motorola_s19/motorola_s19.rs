use std::collections::BTreeMap;

use std::path::Path;

use std::fs::File;

use std::io::BufWriter;
use std::io::Write;

use crate::common::hex::{decode_hex, encode_hex};

use crate::common::file_handling::read_lines;

// https://de.wikipedia.org/wiki/S-Record
//
// S0 - header
// S1 - Datenreihe (2 Byte Address)
// S2 - Datenreihe (3 Byte Address)
// S3 - Datenreihe (4 Byte Address)
//
// S5 - Record count (Datensatzanzahl) (2 Byte Address)
// S6 - Record count (Datensatzanzahl) (3 Byte Address)
//
// S7 - End of block (Blockende) (4 Byte Address)
// S8 - End of block (Blockende) (3 Byte Address)
// S9 - End of block (Blockende) (2 Byte Address)
//
// Length (includes Address+Data+Checksum == Everything after the length byte to the end of the row)
pub fn load_motorola_srec_s19(memory_block_map: &mut BTreeMap<u32, Vec<u8>>, filepath: &str) {

    let debug: bool = false;

    if let Ok(lines) = read_lines(filepath) {

        // consumes the iterator, returns an (Optional) String
        for line in lines.map_while(Result::ok) {

            if debug {
                // DEBUG
                println!("{}", line);
            }

            // record type (two bytes, e.g. S1)
            let record_type = &line[0..2];

            // record len (two bytes, e.g. 24, to be interpreted as a hex number with two hex digits)
            let record_len = &line[2..4];

            match record_type {

                // header (may contain an ASCII string)
                "S0" => {

                    let mut chars = line.chars();
                    chars.next(); // record type (two bytes, e.g. S1)
                    chars.next();
                    chars.next(); // record len (two bytes, e.g. 24, to be interpreted as a hex number with two hex digits)
                    chars.next();
                    chars.next(); // four hex digits (= 2 Bytes) address
                    chars.next();
                    chars.next();
                    chars.next();

                    chars.next_back();
                    chars.next_back();

                    let record_data = chars.as_str();

                    if debug {
                        println!("{}", record_data.to_ascii_lowercase());
                    }

                    let mut i = 0;
                    let mut character: u8 = 0;
                    for c in record_data.chars() {

                        if i == 0 {

                            let digit = c as u8;

                            if digit >= 0x30 && digit <= 0x39 {
                                character = (digit - 0x30) << 4;
                            } else if digit >= 0x41 && digit <= 0x46 {
                                character = (digit - 55) << 4;
                            } else if digit >= 0x61 && digit <= 0x66 {
                                character = (digit - 87) << 4;
                            }

                            i = i + 1;

                        } else {

                            let digit = c as u8;

                            if digit >= 0x30 && digit <= 0x39 {
                                character = character | (digit - 0x30) << 0;
                            } else if digit >= 0x41 && digit <= 0x46 {
                                character = character | (digit - 55) << 0;
                            } else if digit >= 0x61 && digit <= 0x66 {
                                character = character | (digit - 87) << 0;
                            }

                            if debug {
                                //println!(" {:02x?} {}", character, character as char);
                                print!("{}", character as char);
                            }
                            i = 0;

                        }
                    }

                    if debug {
                        println!();
                    }
                }

                "S1" => {

                    //
                    // Datenreihe mit 2 Addressbytes
                    //

                    // record address
                    let record_address = &line[4..8];

                    // payload ( cut off the first 8 characters, and the two very last characters of the line (checksum))
                    let mut chars = line.chars();
                    chars.next(); // record type (two bytes, e.g. S1)
                    chars.next();
                    chars.next(); // record len (two bytes, e.g. 24, to be interpreted as a hex number with two hex digits)
                    chars.next();
                    chars.next(); // four hex digits (= 2 Bytes) address
                    chars.next();
                    chars.next();
                    chars.next();

                    chars.next_back(); // remove the checksum characters (1 Byte checksum)
                    chars.next_back();

                    let record_data = chars.as_str();

                    if debug {
                        // DEBUG
                        println!("{} | {} | {:04x?} | {}", record_type, record_len, record_address, record_data);
                    }

                    let data = decode_hex(record_data).unwrap();

                    if debug {
                        // DEBUG
                        println!("{:02x?}", data);
                    }

                    let mut address = u32::from_str_radix(record_address, 16).expect("Cannot parse!");
                    //println!("{:08x?}", line_offset);

                    // build high and low part of the current line offset
                    let _high_part: u32 = address & 0xFFFF0000;
                    let _low_part: u32 = address & 0x0000FFFF;
                    //println!("high_part: 0x{:08x?}", high_part);
                    //println!(" low_part: 0x{:08x?}", low_part);

                    //println!("{:08x?}", record_data);

                    let data = decode_hex(record_data).unwrap();
                    let payload_length = u32::from_str_radix(record_len, 16).expect("Cannot parse!");

                    //print!("addr: 0x{:02x} ", low_part);

                    let mut i = 0;
                    for idx in 0 .. payload_length-3 {

                        //println!("addr: 0x{:02x}, data: 0x{:02x}", (low_part + i), data[idx as usize]);

                        //print!("{:02x} ", data[idx as usize]);

                        //let address = (low_part + i) as u32;
                        //println!("0x{:08x} ", address);

                        let high_part: u32 = address & 0xFFFF0000;
                        let low_part: u32 = address & 0x0000FFFF;

                        if !memory_block_map.contains_key(&high_part) {

                            if debug {
                                println!("S1 creating block: 0x{:08x?}", address);
                            }
    
                            let mut memory_block: Vec<u8> = vec![0; 0x10000];
                            memory_block.iter_mut().map(|x| *x = 0xFF).count();
                            memory_block_map.insert(high_part, memory_block);
                        }
    
                        let memory_block: &mut Vec<u8> = memory_block_map.get_mut(&high_part).unwrap();

                        let temp = data[idx as usize];

                        //println!("0x{:08x} {:02x}", low_part, temp);
                        memory_block[low_part as usize] = temp;

                        i = i + 1;
                        address = address + 1;
                    }
                }

                "S2" => {

                    //
                    // Datenreihe mit 3 Addressbytes
                    //

                    // record address
                    let record_address = &line[4..10];

                    //println!("record_address: 0x{:08x?}", record_address);

                    // payload ( cut off the first 8 characters, and the two very last characters of the line (checksum))
                    let mut chars = line.chars();
                    chars.next(); // record type (two bytes, e.g. S1)
                    chars.next();
                    chars.next(); // record len (two bytes, e.g. 24, to be interpreted as a hex number with two hex digits)
                    chars.next();
                    chars.next(); // six hex digits (= 3 Bytes) address
                    chars.next();
                    chars.next();
                    chars.next();
                    chars.next();
                    chars.next();

                    chars.next_back(); // remove the checksum characters (1 Byte checksum)
                    chars.next_back();

                    let record_data = chars.as_str();

                    if debug {
                        // DEBUG
                        println!("{} | {} | {:06x?} | {}", record_type, record_len, record_address, record_data);
                    }

                    // if record_address == "01617D" {
                    //     println!("test")
                    // }

                    let _data = decode_hex(record_data).unwrap();

                    // if debug {
                    //     // DEBUG
                    //     println!("{:02x?}", data);
                    // }

                    let mut address = u32::from_str_radix(record_address, 16).expect("Cannot parse!");
                    //println!("{:08x?}", address);

                    // build high and low part of the current line offset
                    let _high_part: u32 = address & 0xFFFF0000;
                    let _low_part: u32 = address & 0x0000FFFF;
                    //println!("high_part: 0x{:08x?}", high_part);
                    //println!(" low_part: 0x{:08x?}", low_part);

                    //println!("{:08x?}", record_data);

                    let data = decode_hex(record_data).unwrap();
                    let payload_length = u32::from_str_radix(record_len, 16).expect("Cannot parse!");

                    //print!("addr: 0x{:02x} ", low_part);

                    let mut i = 0;
                    for idx in 0 .. payload_length-4 {

                        //println!("addr: 0x{:02x}, data: 0x{:02x}", (low_part + i), data[idx as usize]);

                        //print!("{:02x} ", data[idx as usize]);

                        // let address = (low_part + i) as u32;
                        // //println!("0x{:08x} ", address);

                        let high_part: u32 = address & 0xFFFF0000;
                        let low_part: u32 = address & 0x0000FFFF;

                        //println!("high_part: 0x{:08x?}", high_part);
                        //println!(" low_part: 0x{:08x?}", low_part);

                        if !memory_block_map.contains_key(&high_part) {

                            if debug {
                                println!("S2 creating block: 0x{:08x?}", high_part);
                            }
    
                            let mut memory_block: Vec<u8> = vec![0; 0x10000];
                            memory_block.iter_mut().map(|x| *x = 0xFF).count();
                            memory_block_map.insert(high_part, memory_block);
                        }
    
                        let memory_block: &mut Vec<u8> = memory_block_map.get_mut(&high_part).unwrap();

                        let temp = data[idx as usize];

                        //println!("0x{:08x} {:02x}", address, temp);
                        memory_block[low_part as usize] = temp;

                        i = i + 1;
                        address = address + 1;
                    }

                    //println!();
                }

                "S3" => {
                    
                    //
                    // Datenreihe mit 4 Addressbytes
                    //

                    // record address
                    let record_address = &line[4..12];

                    // payload ( cut off the first 8 characters, and the two very last characters of the line (checksum))
                    let mut chars = line.chars();
                    chars.next(); // record type (two bytes, e.g. S1)
                    chars.next();
                    chars.next(); // record len (two bytes, e.g. 24, to be interpreted as a hex number with two hex digits)
                    chars.next();
                    chars.next(); // six hex digits (= 3 Bytes) address
                    chars.next();
                    chars.next();
                    chars.next();
                    chars.next();
                    chars.next();
                    chars.next();
                    chars.next();

                    chars.next_back(); // remove the checksum characters (1 Byte checksum)
                    chars.next_back();

                    let record_data = chars.as_str();

                    if debug {
                        // DEBUG
                        println!("{} | {} | {:08x?} | {}", record_type, record_len, record_address, record_data);
                    }

                    let data = decode_hex(record_data).unwrap();

                    if debug {
                        // DEBUG
                        println!("{:02x?}", data);
                    }

                    let mut address = u32::from_str_radix(record_address, 16).expect("Cannot parse!");
                    //println!("{:08x?}", line_offset);

                    // build high and low part of the current line offset
                    let _high_part: u32 = address & 0xFFFF0000;
                    let _low_part: u32 = address & 0x0000FFFF;
                    //println!("high_part: 0x{:08x?}", high_part);
                    //println!(" low_part: 0x{:08x?}", low_part);

                    //println!("{:08x?}", record_data);

                    let data = decode_hex(record_data).unwrap();
                    let payload_length = u32::from_str_radix(record_len, 16).expect("Cannot parse!");

                    //print!("addr: 0x{:02x} ", low_part);

                    let mut i = 0;
                    for idx in 0 .. payload_length-5 {

                        //println!("addr: 0x{:02x}, data: 0x{:02x}", (low_part + i), data[idx as usize]);

                        //print!("{:02x} ", data[idx as usize]);

                        //let address = (low_part + i) as u32;
                        //println!("0x{:08x} ", address);

                        let high_part: u32 = address & 0xFFFF0000;
                        let low_part: u32 = address & 0x0000FFFF;

                        if !memory_block_map.contains_key(&high_part) {

                            if debug {
                                println!("S3 creating block: 0x{:08x?}", address);
                            }
    
                            let mut memory_block: Vec<u8> = vec![0; 0x10000];
                            memory_block.iter_mut().map(|x| *x = 0xFF).count();
                            memory_block_map.insert(high_part, memory_block);
                        }
    
                        let memory_block: &mut Vec<u8> = memory_block_map.get_mut(&high_part).unwrap();

                        let temp = data[idx as usize];
                        memory_block[low_part as usize] = temp;

                        i = i + 1;
                        address = address + 1;
                    }

                    //println!();

                }

                "S7" => {
                    // End of block (Blockende)
                }

                "S8" => {
                    // End of block (Blockende)
                }

                "S9" => {
                    // End of block (Blockende)
                }

                _ => {
                    todo!();
                }

            }

        }

    }
}

// creates a S0 header record for a string of text
pub fn block_header_text_to_s19(text: &'static str) -> String {

    use std::fmt::Write;

    let mut s = String::new();
    let mut sum: u32 = 0;

    s.push_str("S0");

    // record length (address (2 Byte) + payload (n Byte) + checksum (1 Byte))
    let len: u32 = text.len() as u32 + 2 + 1;
    write!(s, "{:02X}", len).ok();
    sum = sum + ((len as u32 >> 0) & 0xFF) as u32;

    // sum up address
    s.push_str("0000");
    sum = sum + 0;
    sum = sum + 0;

    for cr in text.chars() {
        write!(s, "{:02X}", cr as u8).ok();

        sum = sum + cr as u32;
    }

    // one's complement of sum and write checksum
    let checksum: u8 = !((sum & 0xFF) as u8);
    write!(s, "{:02X}", checksum).ok();

    s
}

// remove trailing runs of 0xFF from a line
fn trim_trailing_ff(x: &[u8]) -> &[u8] {
    // let from = match x.iter().position(|x| !x.is_ascii_whitespace()) {
    //     Some(i) => i,
    //     None => return &x[0..0],
    // };
    // let to = x.iter().rposition(|x| !x.is_ascii_whitespace()).unwrap();
    // &x[from..=to]

    let to = x.iter().rposition(|x| *x != 0xFFu8).unwrap();

    &x[0..=to]
}

// converts one run of 32 bytes into a Motorola S19 record (S1 and S2. S3 is not implemented!)
pub fn buffer_to_string(temp_buffer: &Vec<u8>, current_address: u32, debug: bool) -> Option<String> {

    use std::fmt::Write;

    // compute checksum and check for empty record
    let mut sum: u32 = 0;
    let mut data_found = false;

    for data_byte in temp_buffer.iter() {

        if *data_byte != 0xFF {
            data_found = true;
        }

        //sum = sum + (*data_byte as u32);
    }

    // https://stackoverflow.com/questions/27372976/whats-the-best-way-to-implement-a-string-buffer-in-rust
    let mut s = String::new();

    if data_found {

        // remove trailing runs of 0xFF characters
        //let trimmed = trim_trailing_ff(temp_buffer);
        let trimmed = temp_buffer;

        // add payload to the checksum
        for data_byte in trimmed.iter() {
            sum = sum + (*data_byte as u32);
        }

        // check how many bytes are required to correctly store the address (2, 3 or 4 byte)
        if current_address <= 0xFFFF {

            // record Type ( S1 == 2 Byte address )

            if debug {
                print!("S1");
            }
            s.push_str("S1");

            // record Length
            let len = trimmed.len() + 2 + 1; // 2 Byte address + 1 Byte Checksum
            if debug {
                print!("{:02x}", len);
            }
            write!(s, "{:02X}", len).ok();
            sum = sum + ((len as u32 >> 0) & 0xFF) as u32;

            // address
            if debug {
                print!("{:04x}", current_address);
            }
            write!(s, "{:04X}", current_address).ok();
            sum = sum + ((current_address >> 0) & 0xFF);
            sum = sum + ((current_address >> 8) & 0xFF);

            // payload
            if debug {
                print!("{}", encode_hex(&trimmed));
            }
            write!(s, "{}", encode_hex(&trimmed)).ok();

            // checksum
            let checksum: u8 = !((sum & 0xFF) as u8);
            if debug {
                print!("{:02x}", checksum);
            }
            write!(s, "{:02X}", checksum).ok();

            if debug {
                println!();
            }

        } else {

            // record type ( S2 == 3 Byte address )
            if debug {
                print!("S2");
            }
            s.push_str("S2");

            // record Length
            let len = trimmed.len() + 3 + 1; // 3 Byte address + 1 Byte Checksum
            if debug {
                print!("{:02x}", len);
            }
            write!(s, "{:02X}", len).ok();
            sum = sum + ((len as u32 >> 0) & 0xFF) as u32;

            // address
            if debug {
                print!("{:06x}", current_address);
            }
            write!(s, "{:06X}", current_address).ok();
            sum = sum + ((current_address >> 0) & 0xFF);
            sum = sum + ((current_address >> 8) & 0xFF);
            sum = sum + ((current_address >> 16) & 0xFF);

            // payload
            if debug {
                print!("{}", encode_hex(&trimmed));
            }
            write!(s, "{}", encode_hex(&trimmed)).ok();

            // checksum
            let checksum: u8 = !((sum & 0xFF) as u8);
            if debug {
                print!("{:02x}", checksum);
            }
            write!(s, "{:02X}", checksum).ok();

            if debug {
                println!();
            }
        }

        return Some(s);

    }

    None
}

// https://codesignal.com/learn/courses/fundamentals-of-text-data-manipulation-in-rust/lessons/writing-and-appending-text-files-in-rust-1
pub fn write_motorola_srec_s19(memory_block_map: &mut BTreeMap<u32, Vec<u8>>, filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
    
    let debug: bool = false;

    // specify the path for the output file
    let output_file_path = Path::new(filepath);

    // create a new file (this will overwrite if it already exists)
    let file = File::create(output_file_path)?;
    
    // wrap with BufWriter for efficient writing
    let mut writer = BufWriter::new(file);

    // header
    //println!("S013000044656275675C54485A3130302E736D38DC");
    //writeln!(writer, "S013000044656275675C54485A3130302E736D38DC"); // Debug\THZ100.sm8
    let block_header_text = block_header_text_to_s19("TopShelfLurch");
    writeln!(writer, "{}", block_header_text).ok();

    let mut _byte_counter = 0;
    let mut _current_address_valid: bool = false;
    let mut current_address: u32 = 0;
    let mut temp_buffer: Vec<u8> = vec![0xFF; 0];

    for (key, value) in memory_block_map {

        if debug {
            println!("0x{:04x?}", key);
        }

        _current_address_valid = false;
        temp_buffer.clear();
        _byte_counter = 0;

        // DEBUG
        //writeln!(writer, "{}", encode_hex(&value[0..16]));

        // for i in 0..0xFFFF {
        //     let byte_val = value[i as usize];
        //     if byte_val != 0xFF {
        //         println!("{:08x?}: {:02x?} ", (key + i), byte_val);
        //     }
        // }

        for i in 0..0x10000 {

            let byte_val = value[i as usize];

            if !_current_address_valid {
                current_address = key + _byte_counter;
                _current_address_valid = true;
            }
            temp_buffer.push(byte_val);

            if temp_buffer.len() == 32 {

                // flush
                let buffer_as_string_option = buffer_to_string(&temp_buffer, current_address, debug);
                match buffer_as_string_option {
                    None => {
                    }
                    Some(buffer_as_string) => {
                        writeln!(writer, "{}", buffer_as_string.as_str())?;
                    }
                }

                temp_buffer.clear();
                _current_address_valid = false;
            }

            _byte_counter = _byte_counter + 1;
        }


        



        // Initialize:
        //   - current address = -1 (to hashmap key as high part, low part remains 0x0000
        //   - byte counter = 0
        //   NOT NEEDED - empty counter = 0
        //   - reset the temp buffer

        // iterate over all bytes in the current memory block
        //   
        //   - if [current address] == -1, set [current address] to the current address (high part: hashmap key, low part: byte counter)
        //
        //   - insert current value into temp buffer
        //   - if temp buffer is full, flush current temp buffer to .s19 at [current address] and reset the temp buffer and set [current address] == -1
        //
        //   - increment the byte counter so that an absolute address can be computed from the hashmap key and the byte counter

        // After the last iteration over all hashmap blocks
        //   - flush current temp buffer to .s19 at [current address] no matter if it is full or not
        //   - write S804FFFFFFFE record

        // Flushing a temp buffer to the .s19
        //   - first find the length of the data (= amount of bytes from start minus trailing 0xFF)
        //     e.g. 11 22 33 44 55 66 FF FF FF FF FF FF has a length of 6
        //   - check the [current address]. 
        //         - If [current address] fits into 2 byte, write a S1 record (= 2 byte address) with a length of 0x23 == 2 byte Address, 32 byte payload, 1 byte checksum
        //         - If [current address] fits into 3 byte, write a S2 record (= 3 byte address) with a length of 0x24 == 3 byte Address, 32 byte payload, 1 byte checksum
        //   - write S








        // if the current value is a 0xFF 
        //   - increment the empty counter
        //   - if empty counter < 100, insert 0xFF into temp buffer if temp buffer does not have has max size already. if [current address] == -1, set [current address] to the current address (hashmap key + byte counter)
        //   - if empty counter is > 100, flush current temp buffer to .s19 at [current address] and reset the temp buffer. Set [current address] to -1

        // if the current value is not a 0xFF 
        //   - set empty counter to 0
        //   - if [current address] == -1, set [current address] to the current address (hashmap key + byte counter)
        //   - if temp buffer is full, flush current temp buffer to .s19 at [current address] and reset the temp buffer. Set [current address] == -1
        //   - insert current value into temp buffer
        //   - if [current address] == -1, set [current address] to the current address (hashmap key + byte counter)
        //   - if temp buffer has max size, flush current data to .s19 at at [current address] and reset the temp buffer. Set [current address] == -1
    }

    // flush the remaining bytes
    let buffer_as_string_option = buffer_to_string(&temp_buffer, current_address, false);
    match buffer_as_string_option {
        None => {
            
        }
        Some(buffer_as_string) => {
            writeln!(writer, "{}", buffer_as_string.as_str())?;
        }
    }

    // write some mysterious end record
    writeln!(writer, "S804FFFFFFFE").ok();
    
    // ensure everything is written to disk
    writer.flush()?;

    if debug {
        println!("Data written to {} successfully.", output_file_path.display());
    }

    // the BufWriter and File will be automatically closed when they go out of scope
    Ok(())
}