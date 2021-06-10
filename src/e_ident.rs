use super::util::print_align;
pub struct EIdent {
    magic_number: [u8; 4],
    ei_class: u8,
    ei_data: u8,
    ei_version: u8,
    ei_osabi: u8,
    ei_abiversion: u8,
    ei_pad: [u8; 7],
}

impl EIdent {
    pub fn new(bytes: &[u8]) -> EIdent {
        EIdent {
            magic_number: [bytes[0], bytes[1], bytes[2], bytes[3]],
            ei_class: bytes[4],
            ei_data: bytes[5],
            ei_version: bytes[6],
            ei_osabi: bytes[7],
            ei_abiversion: bytes[8],
            ei_pad: [0, 0, 0, 0, 0, 0, 0],
        }
    }
    fn print_magic_number(&self) {
        let mut magic_nunber = String::from("");
        for i in 0..4 {
            magic_nunber.push_str(&format!("{:02X} ", self.magic_number[i]));
        }
        print_align("Magic", &magic_nunber);
    }

    fn print_class(&self) {
        let class = match self.ei_class {
            1 => "ELF32",
            2 => "ELF64",
            _ => "未知",
        };
        print_align("Class", &class);
    }

    fn print_data(&self) {
        let data = match self.ei_data {
            1 => "小端",
            2 => "大端",
            _ => "未知",
        };
        print_align("Data", data);
    }

    fn print_version(&self) {
        print_align("Version", &self.ei_version.to_string());
    }

    pub fn print_e_ident(&self) {
        self.print_magic_number();
        self.print_class();
        self.print_data();
        self.print_version();
    }
}
