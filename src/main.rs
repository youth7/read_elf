
mod elf_header;
mod e_ident;
mod util;
use std::fs;




fn main() {
    let elf_obj_file = "./test.o";
    let bytes = fs::read(elf_obj_file).expect("我去，盆泥");
    let elf_header = elf_header::ELFHeader::new(&bytes);
    elf_header.show_message();
}
