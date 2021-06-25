use std::usize;


mod util;
mod elf_header;
mod section;


fn main() {
    let elf_obj_file = "./test.o";
    let bytes = std::fs::read(elf_obj_file).unwrap();
    let elf_header = elf_header::ELFHeader::new(&bytes);
    elf_header.show_message();
    let sction_table = section::SectionTable::new(&bytes, elf_header.e_shoff as usize, elf_header.e_shnum as usize);
}
