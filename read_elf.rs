use std::fs;
use std::io;
use std::convert::TryInto;

const EI_CLASS:usize = 4;
const EI_DATA:usize = 5;
const EI_VERSION:usize = 6;
const EI_OSABI:usize = 7;
const EI_ABIVERSION:usize = 8;
struct ELFHeader{
    e_ident: [u8;16],
    e_type: u16,
    e_machine: u16,
    e_shoff:u64
}
 
impl ELFHeader{
    fn new(bytes:&[u8]) -> ELFHeader{
        ELFHeader{
            e_ident :[
                bytes[0],// magic number
                bytes[1],// magic number
                bytes[2],// magic number
                bytes[3],// magic number
                bytes[4],// EI_CLASS
                bytes[5],// EI_DATA
                bytes[6],// EI_VERSION
                bytes[7],// EI_OSABI
                bytes[8],// EI_ABIVERSION
                0,0,0,0,0,0,0// EI_PAD
            ],
            e_type: u16::from_le_bytes(bytes[16..=17].try_into().unwrap()),
            e_machine: u16::from_le_bytes(bytes[18..=19].try_into().unwrap()),
            e_shoff:u64::from_le_bytes(bytes[0x28..=0x2f].try_into().unwrap()),
        }            
    }
    
    fn show_message(&self){
        const tab: &str = "\t\t\t\t\t\t\t";
        fn print_magic_number(this: &ELFHeader){
            print!("Magic:{}", tab);
            for i in 0..8 {
                print!("{:0>2x} ", this.e_ident[i]);
            }
            println!("");
        };
        fn print_class(this: &ELFHeader){
            let class = match this.e_ident[EI_CLASS]{
                1 =>"ELF32",
                2 =>"ELF64",
                _ =>"未知"
            };
            println!("Class: {} {}", tab, class);
        };

        fn print_data(this: &ELFHeader){
            let  data = match this.e_ident[EI_DATA]{
                1 => "小端",
                2 => "大端",
                _ => "未知"
            };
            println!("Data: {} {}", tab, data);
        };
        fn print_version(this: &ELFHeader){
            println!("Version: {} {}", tab, this.e_ident[EI_VERSION]);
        };
        fn print_type(this: &ELFHeader){
            let file_type = match this.e_type{
                0x00 => "ET_NONE",
                0x01 => "ET_REL",
                0x02 => "ET_EXEC",
                0x03 => "ET_DYN",
                0x04 => "ET_CORE",
                0xFE00 => "ET_LOOS",
                0xFEFF => "ET_HIOS",
                0xFF00 => "ET_LOPROC",
                0xFFFF => "ET_HIPROC",
                _=> "unknown"
            };
            println!("Type: {} {}", tab, file_type);
        };
        print_magic_number(&self);
        print_class(&self);
        print_data(&self);
        print_version(&self);
        print_type(&self);
        println!("Start of section headers {} {}", tab, self.e_shoff);
        

    }
}




fn read_file(elf_obj_file: &str) -> Vec<u8> {
   let reslut = fs::read(elf_obj_file);
   match reslut {
        io::Result::Err(e) => {
            println!("盆泥 {:?}", e);
            Vec::new()
        },
        Ok(data) => data
   }

}

 

fn main(){
    let elf_obj_file = "./test.o";
    let bytes = read_file(elf_obj_file);
    let magic = ELFHeader::new(&bytes);
    magic.show_message();
}