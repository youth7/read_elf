use std::usize;

use super::section_entry::SectionEntry;

const SIZE_OF_SECTION_ENTRY: usize = 64;
// use std::Vec;
pub struct SectionTable {
    entries: Vec<SectionEntry>,
    string_table: Vec<u8>,
}

impl SectionTable {
    pub fn new(
        bytes: &[u8],
        offset: usize,
        num_of_entry: usize,
        string_table_index: usize,
    ) -> SectionTable {
        let mut entries: Vec<SectionEntry> = Vec::new();
        let mut i: usize = 0;
        while i < num_of_entry {
            let start = offset + i * SIZE_OF_SECTION_ENTRY;
            let end = start + SIZE_OF_SECTION_ENTRY;
            entries.push(SectionEntry::new(&bytes[start..end]));
            i += 1;
        }
        let string_table_info = entries.get(string_table_index).unwrap();
        let start = string_table_info.sh_offset as usize;
        let end = string_table_info.sh_size as usize + start;
        let string_table: Vec<u8> = Vec::from(&bytes[start..end]);

        SectionTable {
            entries,
            string_table,
        }
    }
    pub fn print_section_table(&self) {
        SectionEntry::print_header();
        for (i, entry) in self.entries.iter().enumerate() {
            entry.print(i, self.string_table.as_slice());
        }
    }
}
