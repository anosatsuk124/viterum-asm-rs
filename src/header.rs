#[derive(Debug)]
pub struct magic_numbers {
    pub MAG0: u8,
    pub MAG1: u8,
    pub MAG2: u8,
    pub MAG3: u8,
}

impl magic_numbers {
    pub fn as_slice(&self) -> [u8; 4] {
        [self.MAG0, self.MAG1, self.MAG2, self.MAG3]
    }
}

#[derive(Debug)]
pub struct e_ident {
    pub EI_MAG: magic_numbers,
    pub EI_CLASS: u8,
    pub EI_DATA: u8,
    pub EI_VERSION: u8,
    pub EI_OSABI: u8,
    pub EI_ABIVERISON: u8,
    pub EI_PAD: [u8; 7],
}

impl e_ident {
    pub fn to_vec(&self) -> Vec<u8> {
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
pub struct elf64_header {
    pub ident: e_ident,
    pub e_type: u16,
    pub e_machine: u16,
    pub e_verison: u32,
    pub e_entry: u64, // if 32bit, [u8;4]
    pub e_phoff: u64, // same as above
    pub e_shoff: u64, // same as above
    pub e_flags: u32,
    pub e_ehsize: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16,
}

impl elf64_header {
    pub fn to_vec(&self) -> Vec<u8> {
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

pub struct section_header {
    pub sh_name: u32,
    pub sh_type: u32,
    pub sh_flags: u64,
    pub sh_addr: u64,
    pub sh_offset: u64,
    pub sh_size: u64,
    pub sh_link: u32,
    pub sh_info: u32,
    pub sh_addralign: u64,
    pub sh_entsize: u64,
}

impl section_header {
    pub fn to_vec(&self) -> Vec<u8> {
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

pub struct Elf64_Phdr {
    pub p_type: u32,
    pub p_flags: u32,
    pub p_offset: u64,
    pub p_vaddr: u64,
    pub p_paddr: u64,
    pub p_filesz: u64,
    pub p_memsz: u64,
    pub p_align: u64,
}

impl Elf64_Phdr {
    pub fn to_vec(&self) -> Vec<u8> {
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
