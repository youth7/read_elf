use crate::util::print_align;
pub struct EIdent {
    magic_number: [u8; 4],
    ei_class: u8,
    ei_data: u8,
    ei_version: u8,
    ei_os_abi: u8,
    ei_abi_version: u8,
    ei_pad: [u8; 7],
}

impl EIdent {
    pub fn new(bytes: &[u8]) -> EIdent {
        let mut ei_pad = [0; 7];
        ei_pad.clone_from_slice(&bytes[9..=15]);
        EIdent {
            magic_number: [bytes[0], bytes[1], bytes[2], bytes[3]],
            ei_class: bytes[4],
            ei_data: bytes[5],
            ei_version: bytes[6],
            ei_os_abi: bytes[7],
            ei_abi_version: bytes[8],
            ei_pad
        }
    }
    fn print_magic_number(&self) {
        let mut magic_nunber = String::from("");
        for i in 0..4 {
            magic_nunber.push_str(format!("{:02X} ", self.magic_number[i]).as_str());
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

    fn print_elf_version(&self) {
        print_align("ELF Version", self.ei_version.to_string().as_str());
    }

    fn print_os_abi(&self) {
        print_align("OS/ABI", self.ei_os_abi.to_string().as_str());
    }

    fn print_os_abi_version(&self) {
        print_align("OS/ABI Version", self.ei_abi_version.to_string().as_str());
    }
    pub fn print_e_ident(&self) {
        self.print_magic_number();
        self.print_class();
        self.print_data();
        self.print_elf_version();
        self.print_os_abi();
        self.print_os_abi_version();
        self.ei_pad;
    }
}
