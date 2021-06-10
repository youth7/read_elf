//参照这里来解析：https://en.wikipedia.org/wiki/Executable_and_Linkable_Format#Section_header


use std::convert::TryInto;
use super::e_ident::EIdent;
use crate::util::print_align;
pub struct ELFHeader {
    e_ident: EIdent,
    e_type: u16,
    e_machine: u16,
    e_version: u32,
    e_shoff: u64,
}


impl ELFHeader {
    pub fn new(bytes: &[u8]) -> ELFHeader {
        ELFHeader {
            e_ident: EIdent::new(bytes),
            e_type: u16::from_le_bytes(bytes[16..=17].try_into().expect("卧槽盆泥")),
            e_machine: u16::from_le_bytes(bytes[18..=19].try_into().expect("卧槽盆泥")),
            e_version: u32::from_le_bytes(bytes[20..=23].try_into().expect("卧槽盆泥")),
            e_shoff: u64::from_le_bytes(bytes[0x28..=0x2f].try_into().expect("卧槽盆泥")),
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
    pub fn show_message(&self) {
        self.e_ident.print_e_ident();
        self.print_type();
        print_align("Start of section headers", &self.e_shoff.to_string());
    }
}
