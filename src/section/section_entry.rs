use crate::util::print_align;
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
        while name.len()< 20{
            name.push(' ')
        }
        name
    }

    fn section_type(&self) -> String {
        match &self.sh_type {
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
            v @ _ => format!("{:X}", v),
        }
    }


    pub fn print_header(){
        println!("index\tname\tType\tFlags\tAddress\tOffset\tSize\tLink\tInfo\tAlign\tEntSize");
    }
    pub fn print(&self, index: usize, section_header_string_table: &[u8]) {
        println!(
            "[{}]\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            index,
            self.interpret_section_name(section_header_string_table),
            self.section_type(),
            self.sh_flags.to_string().as_str(),
            self.sh_addr.to_string().as_str(),
            self.sh_offset.to_string().as_str(),
            self.sh_size.to_string().as_str(),
            self.sh_link.to_string().as_str(),
            self.sh_info.to_string().as_str(),
            self.sh_addralign.to_string().as_str(),
            self.sh_entsize.to_string().as_str()
        );
    }
}
