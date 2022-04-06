use std::fs;
use std::io::{BufWriter, Write};

#[derive(Debug)]
struct magic_numbers {
    MAG0: u8,
    MAG1: u8,
    MAG2: u8,
    MAG3: u8,
}

impl magic_numbers {
    fn as_slice(&self) -> [u8; 4] {
        [self.MAG0, self.MAG1, self.MAG2, self.MAG3]
    }
}

#[derive(Debug)]
struct e_ident {
    EI_MAG: magic_numbers,
    EI_CLASS: u8,
    EI_DATA: u8,
    EI_VERSION: u8,
    EI_OSABI: u8,
    EI_ABIVERISON: u8,
    EI_PAD: [u8; 7],
}

impl e_ident {
    fn to_vec(&self) -> Vec<u8> {
        let mags: Vec<u8> = self.EI_MAG.as_slice().to_vec();
        let other: Vec<u8> = vec![
            self.EI_CLASS,
            self.EI_DATA,
            self.EI_VERSION,
            self.EI_OSABI,
            self.EI_ABIVERISON,
        ];
        [mags, other, self.EI_PAD.to_vec()].concat()
    }
}

#[derive(Debug)]
struct elf64_header {
    ident: e_ident,
    e_type: u16,
    e_machine: u16,
    e_verison: u32,
    e_entry: u64, // if 32bit, [u8;4]
    e_phoff: u64, // same as above
    e_shoff: u64, // same as above
    e_flags: u32,
    e_ehsize: u16,
    e_phentsize: u16,
    e_phnum: u16,
    e_shentsize: u16,
    e_shnum: u16,
    e_shstrndx: u16,
}

impl elf64_header {
    fn to_vec(&self) -> Vec<u8> {
        let mut vec = Vec::new();
        vec.append(&mut self.ident.to_vec());
        vec.append(&mut self.e_type.to_le_bytes().to_vec());
        vec.append(&mut self.e_machine.to_le_bytes().to_vec());
        vec.append(&mut self.e_verison.to_le_bytes().to_vec());
        vec.append(&mut self.e_entry.to_le_bytes().to_vec());
        vec.append(&mut self.e_phoff.to_le_bytes().to_vec());
        vec.append(&mut self.e_shoff.to_le_bytes().to_vec());
        vec.append(&mut self.e_flags.to_be_bytes().to_vec());
        vec.append(&mut self.e_ehsize.to_le_bytes().to_vec());
        vec.append(&mut self.e_phentsize.to_le_bytes().to_vec());
        vec.append(&mut self.e_phnum.to_le_bytes().to_vec());
        vec.append(&mut self.e_shentsize.to_le_bytes().to_vec());
        vec.append(&mut self.e_shnum.to_le_bytes().to_vec());
        vec.append(&mut self.e_shstrndx.to_le_bytes().to_vec());

        vec
    }
}

struct section_header {
    sh_name: u32,
    sh_type: u32,
    sh_flags: u64,
    sh_addr: u64,
    sh_offset: u64,
    sh_size: u64,
    sh_link: u32,
    sh_info: u32,
    sh_addralign: u64,
    sh_entsize: u64,
}

impl section_header {
    fn to_vec(&self) -> Vec<u8> {
        let mut vec = Vec::new();
        vec.append(&mut self.sh_name.to_le_bytes().to_vec());
        vec.append(&mut self.sh_type.to_le_bytes().to_vec());
        vec.append(&mut self.sh_flags.to_le_bytes().to_vec());
        vec.append(&mut self.sh_addr.to_le_bytes().to_vec());
        vec.append(&mut self.sh_offset.to_le_bytes().to_vec());
        vec.append(&mut self.sh_size.to_le_bytes().to_vec());
        vec.append(&mut self.sh_link.to_le_bytes().to_vec());
        vec.append(&mut self.sh_info.to_le_bytes().to_vec());
        vec.append(&mut self.sh_addralign.to_le_bytes().to_vec());
        vec.append(&mut self.sh_entsize.to_le_bytes().to_vec());

        vec
    }
}

/*
typedef struct {
    uint32_t   p_type;
    uint32_t   p_flags;
    Elf64_Off  p_offset;
    Elf64_Addr p_vaddr;
    Elf64_Addr p_paddr;
    uint64_t   p_filesz;
    uint64_t   p_memsz;
    uint64_t   p_align;
} Elf64_Phdr;
 */

struct Elf64_Phdr {
    p_type: u32,
    p_flags: u32,
    p_offset: u64,
    p_vaddr: u64,
    p_paddr: u64,
    p_filesz: u64,
    p_memsz: u64,
    p_align: u64,
}

impl Elf64_Phdr {
    fn to_vec(&self) -> Vec<u8> {
        let mut vec = Vec::new();

        vec.append(&mut self.p_type.to_le_bytes().to_vec());
        vec.append(&mut self.p_flags.to_le_bytes().to_vec());
        vec.append(&mut self.p_vaddr.to_le_bytes().to_vec());
        vec.append(&mut self.p_paddr.to_le_bytes().to_vec());
        vec.append(&mut self.p_filesz.to_le_bytes().to_vec());
        vec.append(&mut self.p_memsz.to_le_bytes().to_vec());
        vec.append(&mut self.p_align.to_le_bytes().to_vec());

        vec
    }
}

fn main() {
    let MAG: magic_numbers = magic_numbers {
        MAG0: 0x7f,
        MAG1: 0x45,
        MAG2: 0x4c,
        MAG3: 0x46,
    };

    let EI: e_ident = e_ident {
        EI_MAG: MAG,
        EI_CLASS: 0x2,                               // ELF64, specify 32bit or 64bit
        EI_DATA: 0x1,                                // little endian, Data
        EI_VERSION: 0x1,                             // version 1
        EI_OSABI: 0x0,                               // Unix - SystemV
        EI_ABIVERISON: 0x0,                          // ABI Version: 0
        EI_PAD: [0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0], // not used, filled with 0
    };

    let HEADER: elf64_header = elf64_header {
        ident: EI,
        e_type: 0x1,
        e_machine: 0x3e,
        e_verison: 0x1,
        e_entry: 0x0,
        e_phoff: 0x0,
        e_shoff: 0x50,
        e_flags: 0x0,
        e_ehsize: 0x40,
        e_phentsize: 0x0,
        e_phnum: 0x0,
        e_shentsize: 0x40,
        e_shnum: 0x7,
        e_shstrndx: 0x6,
    };

    let zero_filled = section_header {
        sh_name: 0x00,
        sh_type: 0x00,
        sh_flags: 0x00,
        sh_addr: 0x00,
        sh_offset: 0x00,
        sh_size: 0x00,
        sh_link: 0x00,
        sh_info: 0x00,
        sh_addralign: 0x01,
        sh_entsize: 0x00,
    };

    let text: section_header = section_header {
        sh_name: 0x01,
        sh_type: 0x01,
        sh_flags: 0x06,
        sh_addr: 0x00,
        sh_offset: 0x00,
        sh_size: 0x10,
        sh_link: 0x00,
        sh_info: 0x00,
        sh_addralign: 0x01,
        sh_entsize: 0x00,
    };

    let data = section_header {
        sh_name: 0x02,
        sh_type: 0x01,
        sh_size: 0x00,
        sh_flags: 0x03,
        sh_addr: 0x00,
        sh_entsize: 0x00,
        sh_offset: 0x50,
        sh_info: 0x00,
        sh_link: 0x00,
        sh_addralign: 0x01,
    };

    let bss = section_header {
        sh_name: 0x03,
        sh_type: 0x08,
        sh_size: 0x00,
        sh_flags: 0x03,
        sh_addr: 0x00,
        sh_entsize: 0x00,
        sh_offset: 0x50,
        sh_info: 0x00,
        sh_link: 0x00,
        sh_addralign: 0x01,
    };

    let symtab = section_header {
        sh_name: 0x04,
        sh_type: 0x02,
        sh_size: 0x78,
        sh_flags: 0x00,
        sh_addr: 0x00,
        sh_entsize: 0x18,
        sh_offset: 0x50,
        sh_info: 0x04,
        sh_link: 0x05,
        sh_addralign: 0x08,
    };

    let strtab = section_header {
        sh_name: 0x05,
        sh_type: 0x03,
        sh_size: 0x08,
        sh_flags: 0x00,
        sh_addr: 0x00,
        sh_entsize: 0x00,
        sh_offset: 0xc8,
        sh_info: 0x00,
        sh_link: 0x00,
        sh_addralign: 0x01,
    };

    let shstrtab = section_header {
        sh_name: 0x06,
        sh_type: 0x03,
        sh_size: 0x2c,
        sh_flags: 0x00,
        sh_addr: 0x00,
        sh_entsize: 0x00,
        sh_offset: 0xd0,
        sh_info: 0x00,
        sh_link: 0x00,
        sh_addralign: 0x01,
    };

    let asm: [u8; 16] = [
        0x48, 0xc7, 0xc3, 0x0a, 0x00, 0x00, 0x00, 0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, 0xcd,
        0x80,
    ];

    let header: Vec<u8> = vec![
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x00,
        0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x03, 0x00, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x10, 0x00, 0x01, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x5f, 0x73, 0x74, 0x61, 0x72, 0x74, 0x00, 0x00, 0x2e, 0x73, 0x79, 0x6d, 0x74, 0x61,
        0x62, 0x00, 0x2e, 0x73, 0x74, 0x72, 0x74, 0x61, 0x62, 0x00, 0x2e, 0x73, 0x68, 0x73, 0x74,
        0x72, 0x74, 0x61, 0x62, 0x00, 0x2e, 0x74, 0x65, 0x78, 0x74, 0x00, 0x2e, 0x64, 0x61, 0x74,
        0x61, 0x00, 0x2e, 0x62, 0x73, 0x73, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x1b, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x21, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x50, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x27, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00,
        0x00, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x50, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00,
        0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x50, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x78,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00,
        0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x18, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x09, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc8, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x11, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xd0,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x2c, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];

    let mut f = BufWriter::new(fs::File::create("main.o").unwrap());

    f.write(&HEADER.to_vec()).unwrap();

    //f.write(&SEC.to_vec()).unwrap();

    f.write(&asm).unwrap();
    f.write(&zero_filled.to_vec()).unwrap();
    f.write(&text.to_vec()).unwrap();
    f.write(&data.to_vec()).unwrap();
    f.write(&bss.to_vec()).unwrap();
    f.write(&symtab.to_vec()).unwrap();
    f.write(&strtab.to_vec()).unwrap();
    f.write(&shstrtab.to_vec()).unwrap();
}
