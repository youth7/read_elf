//参照这里来解析：https://en.wikipedia.org/wiki/Executable_and_Linkable_Format#Section_header


use std::convert::TryInto;
use super::e_ident::EIdent;
use crate::util::print_align;
pub struct ELFHeader {
    e_ident: EIdent,
    e_type: u16,
    e_machine: u16,
    e_version: u32,
    e_entry: u64,
    e_shoff: u64,
}


impl ELFHeader {
    pub fn new(bytes: &[u8]) -> ELFHeader {
        ELFHeader {
            e_ident: EIdent::new(bytes),
            e_type: u16::from_le_bytes(bytes[16..=17].try_into().unwrap()),
            e_machine: u16::from_le_bytes(bytes[18..=19].try_into().unwrap()),
            e_version: u32::from_le_bytes(bytes[20..=23].try_into().unwrap()),
            e_entry: u64::from_le_bytes(bytes[24..=31].try_into().unwrap()),
            e_shoff: u64::from_le_bytes(bytes[0x28..=0x2f].try_into().unwrap()),
        }
    }

    fn print_type(&self) {
        let file_type = match self.e_type {
            0x00 => "ET_NONE",
            0x01 => "ET_REL",
            0x02 => "ET_EXEC",
            0x03 => "ET_DYN",
            0x04 => "ET_CORE",
            _ => "others",
        };
        print_align("Type", file_type);
    }
    fn print_machine(&self){
        print_align("Machine", self.e_machine.to_string().as_str());
    
    }
    fn print_version(&self){
        print_align("Version", self.e_version.to_string().as_str());
    }

    fn print_entry(&self){
        let entry = format!("0x{:02X}", self.e_entry);
        print_align("Entry", &entry);
    }

    fn print_shoff(&self){
        print_align("Start of section headers", &self.e_shoff.to_string());
    }
    
    pub fn show_message(&self) {
        self.e_ident.print_e_ident();
        self.print_type();
        self.print_machine();
        self.print_version();
        self.print_entry();
        self.print_shoff();
    }
}
