// GxRom, simple bank switchable 32kb PRG ROM and 8k CHR ROM
// Reference capabilities: https://wiki.nesdev.com/w/index.php/GxROM

use cartridge::NesHeader;
use mmc::mapper::*;

pub struct GxRom {
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
    pub mirroring: Mirroring,
    pub prg_bank: usize,
    pub chr_bank: usize,
}

impl GxRom {
    pub fn new(header: NesHeader, chr: &[u8], prg: &[u8]) -> GxRom {
        return GxRom {
            prg_rom: prg.to_vec(),
            chr_rom: chr.to_vec(),
            mirroring: header.mirroring,
            prg_bank: 0x00,
            chr_bank: 0x00,
        }
    }
}

impl Mapper for GxRom {
    fn mirroring(&self) -> Mirroring {
        return self.mirroring;
    }

    fn read_byte(&mut self, address: u16) -> Option<u8> {
        match address {
            0x0000 ... 0x1FFF => {
                let chr_rom_len = self.chr_rom.len();
                return Some(self.chr_rom[((self.chr_bank * 0x2000) + (address as usize)) % chr_rom_len]);
            },
            0x8000 ... 0xFFFF => {
                let prg_rom_len = self.prg_rom.len();
                return Some(self.prg_rom[((self.prg_bank * 0x8000) + (address as usize - 0x8000)) % prg_rom_len]);
            },
            _ => return None
        }
    }

    fn write_byte(&mut self, address: u16, data: u8) {
        match address {
            0x8000 ... 0xFFFF => {
                self.prg_bank = ((data & 0b0011_0000) >> 4) as usize;
                self.chr_bank =  (data & 0b0000_0011) as usize;
            }
            _ => {}
        }
    }
}
