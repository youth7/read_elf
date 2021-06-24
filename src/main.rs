
mod elf_header;
mod util;
mod section;


fn main() {
    let elf_obj_file = "./test.o";
    let bytes = std::fs::read(elf_obj_file).unwrap();
    let elf_header = elf_header::ELFHeader::new(&bytes);
    elf_header.show_message();
}
