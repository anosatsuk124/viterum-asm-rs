use std::fs;
use std::io::{BufWriter, Write};
use std::mem::size_of;
mod header;
mod opcode;
use crate::header::*;
use crate::opcode::*;

fn main() {
    let MAG = magic_numbers {
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

    /*
    let asm: [u8; 16] = [
        0x48, 0xc7, 0xc3, 0x0a, 0x00, 0x00, 0x00, 0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, 0xcd,
        0x80,
    ];
    */

    let asm = [
        0x55, 0x48, 0x89, 0xe5, 0x48, 0x81, 0xec, 0xd0, 0x00, 0x00, 0x00, 0x48, 0x89, 0xe8, 0x48,
        0x83, 0xe8, 0x08, 0x50, 0x6a, 0x01, 0x5f, 0x58, 0x48, 0x89, 0x38, 0x57, 0x58, 0x48, 0x89,
        0xe8, 0x48, 0x83, 0xe8, 0x10, 0x50, 0x6a, 0x03, 0x5f, 0x58, 0x48, 0x89, 0x38, 0x57, 0x58,
        0x48, 0x89, 0xe8, 0x48, 0x83, 0xe8, 0x08, 0x50, 0x58, 0x48, 0x8b, 0x00, 0x50, 0x48, 0x89,
        0xe8, 0x48, 0x83, 0xe8, 0x10, 0x50, 0x58, 0x48, 0x8b, 0x00, 0x50, 0x5f, 0x58, 0x48, 0x01,
        0xf8, 0x50, 0x58, 0x48, 0x89, 0xec, 0x5d, 0x48, 0x89, 0xc3, 0x48, 0xc7, 0xc0, 0x01, 0x00,
        0x00, 0x00, 0xcd, 0x80,
    ];
    let mut bin: Vec<u8> = Vec::new();
    //bin.append(&mut mov_value(OP_EAX, 1).to_vec());
    bin.append(&mut int(0x80).to_vec());
    // let asm = int(0x80);

    let section = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x03, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x00,
        0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x00, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00,
        0x00, 0x10, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x5f, 0x73, 0x74, 0x61, 0x72, 0x74, 0x00, 0x00,
        0x2e, 0x73, 0x79, 0x6d, 0x74, 0x61, 0x62, 0x00, 0x2e, 0x73, 0x74, 0x72, 0x74, 0x61, 0x62,
        0x00, 0x2e, 0x73, 0x68, 0x73, 0x74, 0x72, 0x74, 0x61, 0x62, 0x00, 0x2e, 0x74, 0x65, 0x78,
        0x74, 0x00, 0x2e, 0x64, 0x61, 0x74, 0x61, 0x00, 0x2e, 0x62, 0x73, 0x73, 0x00, 0x00, 0x00,
        0x00, 0x00,
    ];

    let shoff = size_of::<elf64_header>() + bin.len() + section.len();

    let HEADER: elf64_header = elf64_header {
        ident: EI,
        e_type: 0x1,
        e_machine: 0x3e,
        e_verison: 0x1,
        e_entry: 0x0,
        e_phoff: 0x0,
        e_shoff: shoff.try_into().unwrap(),
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
        sh_addralign: 0x00,
        sh_entsize: 0x00,
    };

    let textoff = (size_of::<elf64_header>() + bin.len() - 0x02) as u64;

    let text: section_header = section_header {
        sh_name: 0x1b,
        sh_type: 0x01,
        sh_flags: 0x06,
        sh_addr: 0x00,
        sh_offset: textoff,
        sh_size: bin.len() as u64,
        sh_link: 0x00,
        sh_info: 0x00,
        sh_addralign: 0x01,
        sh_entsize: 0x00,
    };

    let data = section_header {
        sh_name: 0x21,
        sh_type: 0x01,
        sh_size: 0x00,
        sh_flags: 0x03,
        sh_addr: 0x00,
        sh_entsize: 0x00,
        sh_offset: textoff + 0x02,
        sh_info: 0x00,
        sh_link: 0x00,
        sh_addralign: 0x01,
    };

    let bss = section_header {
        sh_name: 0x27,
        sh_type: 0x08,
        sh_size: 0x00,
        sh_flags: 0x03,
        sh_addr: 0x00,
        sh_entsize: 0x00,
        sh_offset: textoff + 0x02,
        sh_info: 0x00,
        sh_link: 0x00,
        sh_addralign: 0x01,
    };

    let symtab = section_header {
        sh_name: 0x01,
        sh_type: 0x02,
        sh_size: 0x78,
        sh_flags: 0x00,
        sh_addr: 0x00,
        sh_entsize: 0x18,
        sh_offset: textoff + 0x08,
        sh_info: 0x04,
        sh_link: 0x05,
        sh_addralign: 0x08,
    };

    let strtab = section_header {
        sh_name: 0x09,
        sh_type: 0x03,
        sh_size: 0x08,
        sh_flags: 0x00,
        sh_addr: 0x00,
        sh_entsize: 0x00,
        sh_offset: textoff + 0x80,
        sh_info: 0x00,
        sh_link: 0x00,
        sh_addralign: 0x01,
    };

    let shstrtab = section_header {
        sh_name: 0x11,
        sh_type: 0x03,
        sh_size: 0x2c,
        sh_flags: 0x00,
        sh_addr: 0x00,
        sh_entsize: 0x00,
        sh_offset: textoff + 0x88,
        sh_info: 0x00,
        sh_link: 0x00,
        sh_addralign: 0x01,
    };

    let header = [
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

    // f.write(&asm).unwrap();
    f.write(&bin).unwrap();
    f.write(&section).unwrap();
    /*
    match size_of::<elf64_header>() + bin.len() {
        x if x & (x - 1) == 0 => f.write(&[0x00, 0x00]).unwrap(),
        _ => f.write(&[0x00]).unwrap(),
    };
    */
    f.write(&zero_filled.to_vec()).unwrap();
    f.write(&text.to_vec()).unwrap();
    f.write(&data.to_vec()).unwrap();
    f.write(&bss.to_vec()).unwrap();
    f.write(&symtab.to_vec()).unwrap();
    f.write(&strtab.to_vec()).unwrap();
    f.write(&shstrtab.to_vec()).unwrap();
    println!("{}", textoff);
    println!("{}", shoff);
}
