use crate::util::get_fixed_length_string;
use std::{convert::TryInto, usize};

pub struct SectionEntry {
    pub sh_name: u32,
    pub sh_type: u32,
    pub sh_flags: u64,
    pub sh_addr: u64,
    pub sh_offset: u64,
    pub sh_size: u64,
    pub sh_link: u32,
    pub sh_info: u32,
    pub sh_addralign: u64,
    pub sh_entsize: u64,
}
const LENGTH_OF_NAME:usize = 20;
const LENGTH_OF_TYPE:usize = 10;

impl SectionEntry {
    pub fn new(bytes: &[u8]) -> SectionEntry {
        SectionEntry {
            sh_name: u32::from_le_bytes(bytes[0..0x04].try_into().unwrap()),
            sh_type: u32::from_le_bytes(bytes[0x04..0x08].try_into().unwrap()),
            sh_flags: u64::from_le_bytes(bytes[0x08..0x10].try_into().unwrap()),
            sh_addr: u64::from_le_bytes(bytes[0x10..0x18].try_into().unwrap()),
            sh_offset: u64::from_le_bytes(bytes[0x18..0x20].try_into().unwrap()),
            sh_size: u64::from_le_bytes(bytes[0x20..0x28].try_into().unwrap()),
            sh_link: u32::from_le_bytes(bytes[0x28..0x2C].try_into().unwrap()),
            sh_info: u32::from_le_bytes(bytes[0x2C..0x30].try_into().unwrap()),
            sh_addralign: u64::from_le_bytes(bytes[0x30..0x38].try_into().unwrap()),
            sh_entsize: u64::from_le_bytes(bytes[0x38..0x40].try_into().unwrap()),
        }
    }

    fn interpret_section_name(&self, string_stable: &[u8]) -> String {
        let mut name = String::new();
        let mut offset = self.sh_name as usize;
        loop {
            let character = string_stable[offset];
            if character == 0 {
                break;
            }
            name.push(character as char);
            offset += 1;
        }
        get_fixed_length_string(&name, LENGTH_OF_NAME)
    }

    fn interpret_section_type(&self) -> String {
        let section_type = match &self.sh_type {
            0 => "NULL".to_string(),
            1 => "PROGBITS".to_string(),
            2 => "SYMTAB".to_string(),
            3 => "STRTAB".to_string(),
            4 => "RELA".to_string(),
            5 => "HASH".to_string(),
            6 => "DYNAMIC".to_string(),
            7 => "NOTE".to_string(),
            8 => "NOBITS".to_string(),
            9 => "REL".to_string(),
            10 => "SHLIB".to_string(),
            11 => "SHT_DYNSYM".to_string(),
            v @ _ => format!("?{:X}", v),
        };
        get_fixed_length_string(&section_type, LENGTH_OF_TYPE)
    }

    pub fn print_header(){
        let name = get_fixed_length_string("Name", LENGTH_OF_NAME);
        let section_type = get_fixed_length_string("Type", LENGTH_OF_TYPE);
        println!("index\t{}\t{}\tFlags\tAddress\tOffset\tSize\tLink\tInfo\tAlign\tEntSize", name, section_type);
    }
    pub fn print(&self, index: usize, section_header_string_table: &[u8]) {
        println!(
            "[{}]\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            index,
            self.interpret_section_name(section_header_string_table),
            self.interpret_section_type(),
            self.sh_flags.to_string().as_str(),
            format!("{:X}", self.sh_addr),
            format!("{:X}", self.sh_offset),
            format!("{:X}", self.sh_size),
            self.sh_link.to_string().as_str(),
            self.sh_info.to_string().as_str(),
            self.sh_addralign.to_string().as_str(),
            self.sh_entsize.to_string().as_str()
        );
    }
}
