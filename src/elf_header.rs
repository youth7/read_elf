use std::convert::TryInto;

const EI_CLASS: usize = 4;
const EI_DATA: usize = 5;
const EI_VERSION: usize = 6;
// const EI_OSABI:usize = 7;
// const EI_ABIVERSION:usize = 8;
pub struct ELFHeader {
    e_ident: [u8; 16],
    e_type: u16,
    e_machine: u16,
    e_shoff: u64,
}
const TAB: &str = "\t\t\t";
fn print_align(var_name: &str, value: &str) {
    let diff = 24 - var_name.len();
    let spaces = " ".repeat(diff);
    let new_var_name = format!("{}{}", var_name, spaces);
    // println!("{}需要补全的长度是{}", var_name, value.len());
    println!("{}：{}{}", new_var_name, TAB, value);
}

impl ELFHeader {
    pub fn new(bytes: &[u8]) -> ELFHeader {
        ELFHeader {
            e_ident: [
                bytes[0], // magic number
                bytes[1], // magic number
                bytes[2], // magic number
                bytes[3], // magic number
                bytes[4], // EI_CLASS
                bytes[5], // EI_DATA
                bytes[6], // EI_VERSION
                bytes[7], // EI_OSABI
                bytes[8], // EI_ABIVERSION
                0, 0, 0, 0, 0, 0, 0, // EI_PAD
            ],
            e_type: u16::from_le_bytes(bytes[16..=17].try_into().expect("卧槽盆泥")),
            e_machine: u16::from_le_bytes(bytes[18..=19].try_into().expect("卧槽盆泥")),
            e_shoff: u64::from_le_bytes(bytes[0x28..=0x2f].try_into().expect("卧槽盆泥")),
        }
    }

    pub fn show_message(&self) {
        fn print_magic_number(this: &ELFHeader) {
            let mut magic_nunber = String::from("");
            for i in 0..8 {
                magic_nunber.push_str(&format!("{:02X} ", this.e_ident[i]));
            }
            print_align("Magic", &magic_nunber);
        }

        fn print_class(this: &ELFHeader) {
            let class = match this.e_ident[EI_CLASS] {
                1 => "ELF32",
                2 => "ELF64",
                _ => "未知",
            };
            print_align("Class", &class);
        }

        fn print_data(this: &ELFHeader) {
            let data = match this.e_ident[EI_DATA] {
                1 => "小端",
                2 => "大端",
                _ => "未知",
            };
            print_align("Data", data);
        }

        fn print_version(this: &ELFHeader) {
            print_align("Version", &this.e_ident[EI_VERSION].to_string());
        }
        fn print_type(this: &ELFHeader) {
            let file_type = match this.e_type {
                0x00 => "ET_NONE",
                0x01 => "ET_REL",
                0x02 => "ET_EXEC",
                0x03 => "ET_DYN",
                0x04 => "ET_CORE",
                0xFE00 => "ET_LOOS",
                0xFEFF => "ET_HIOS",
                0xFF00 => "ET_LOPROC",
                0xFFFF => "ET_HIPROC",
                _ => "unknown",
            };
            print_align("Type", file_type);
        }

        print_magic_number(&self);
        print_class(&self);
        print_data(&self);
        print_version(&self);
        print_type(&self);
        print_align("Start of section headers", &self.e_shoff.to_string());
    }
}
