use super::section_entry::SectionEntry;

const SIZE_OF_SECTION_ENTRY: usize = 64;
// use std::Vec;
pub struct SectionTable {
    entries: Vec<SectionEntry>,
}

impl SectionTable {
    pub fn new(bytes: &[u8], offset: usize, num_of_entry: usize) -> SectionTable {
        let mut entries = Vec::new();
        let mut i: usize = 0;
        while i < num_of_entry {
            let start = offset + i * SIZE_OF_SECTION_ENTRY;
            let end = start + SIZE_OF_SECTION_ENTRY;
            entries.push(SectionEntry::new(&bytes[start..end]));
            i += 1;
        }
        SectionTable { entries }
    }
    pub fn print_section_table(&self) {
        for entry in self.entries.iter(){
            entry.print();
        }

    }
}
