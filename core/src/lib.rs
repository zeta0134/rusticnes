pub mod addressing;
pub mod apu;
pub mod asm;
pub mod cartridge;
pub mod cycle_cpu;
pub mod fds;
pub mod tracked_events;
pub mod ines;
pub mod memory;
pub mod memoryblock;
pub mod mmc;
pub mod nes;
pub mod nsf;
pub mod opcodes;
pub mod opcode_info;
pub mod palettes;
pub mod ppu;
pub mod unofficial_opcodes;

pub fn version() -> &'static str {
    option_env!("CARGO_PKG_VERSION").unwrap_or("unknown")
}
