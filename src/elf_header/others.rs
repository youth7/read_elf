//参照这里来解析：https://en.wikipedia.org/wiki/Executable_and_Linkable_Format#Section_header


use std::convert::TryInto;
use super::e_ident::EIdent;
use crate::util::print_align;
pub struct ELFHeader {
    pub e_ident: EIdent,
    pub e_type: u16,
    pub e_machine: u16,
    pub e_version: u32,
    pub e_entry: u64,
    pub e_phoff: u64,
    pub e_shoff: u64,
    pub e_flags:u32,
    pub e_ehsize:u16,
    pub e_phentsize:u16,
    pub e_phnum:u16,
    pub e_shentsize:u16,
    pub e_shnum:u16,
    pub e_shstrndx:u16,
}




impl ELFHeader {
    pub fn new(bytes: &[u8]) -> ELFHeader {
        ELFHeader {
            e_ident: EIdent::new(bytes),
            e_type: u16::from_le_bytes(bytes[16..=17].try_into().unwrap()),
            e_machine: u16::from_le_bytes(bytes[18..=19].try_into().unwrap()),
            e_version: u32::from_le_bytes(bytes[20..=23].try_into().unwrap()),
            e_entry: u64::from_le_bytes(bytes[24..=31].try_into().unwrap()),
            e_phoff: u64::from_le_bytes(bytes[0x20..=0x27].try_into().unwrap()),
            e_shoff: u64::from_le_bytes(bytes[0x28..=0x2f].try_into().unwrap()),
            e_flags: u32::from_le_bytes(bytes[0x30..=0x33].try_into().unwrap()),
            e_ehsize:u16::from_le_bytes(bytes[0x34..=0x35].try_into().unwrap()),
            e_phentsize:u16::from_le_bytes(bytes[0x36..=0x37].try_into().unwrap()),
            e_phnum:u16::from_le_bytes(bytes[0x38..=0x39].try_into().unwrap()),
            e_shentsize:u16::from_le_bytes(bytes[0x3a..=0x3b].try_into().unwrap()),
            e_shnum:u16::from_le_bytes(bytes[0x3c..=0x3d].try_into().unwrap()),
            e_shstrndx:u16::from_le_bytes(bytes[0x3e..=0x3f].try_into().unwrap()),
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
        print_align("Start of section headers", self.e_shoff.to_string().as_str());
    }

    fn print_phoff(&self){
        print_align("Start of program headers", self.e_phoff.to_string().as_str());
    }

    fn print_flags(&self){
        print_align("Flags", self.e_flags.to_string().as_str());
    }

    fn print_ehsize(&self){
        print_align("Size of ELF header", self.e_ehsize.to_string().as_str());
    }

    
    fn print_phentsize(&self){
        print_align("Size of program header entry", self.e_phentsize.to_string().as_str());
    }

    fn print_phnum(&self){
        print_align("Number of program headers", self.e_phnum.to_string().as_str());
    }

    fn print_shentsize(&self){
        print_align("Size of section header entry", self.e_shentsize.to_string().as_str());
    }


    fn print_shnum(&self){
        print_align("Number of section headers", self.e_shnum.to_string().as_str());
    }

    fn print_shstrndx(&self){
        print_align("Section header string table index", self.e_shstrndx.to_string().as_str());
    }
    
    pub fn show_message(&self) {
        self.e_ident.print_e_ident();
        self.print_type();
        self.print_machine();
        self.print_version();
        self.print_entry();
        self.print_phoff();
        self.print_shoff();
        self.print_flags();
        self.print_ehsize();
        self.print_phentsize();
        self.print_phnum();
        self.print_shentsize();
        self.print_shnum();
        self.print_shstrndx();
    }
}
