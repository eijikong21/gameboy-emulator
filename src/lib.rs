use wasm_bindgen::prelude::*;

// This tells the compiler these files exist
pub mod cpu;
pub mod mmu;
pub mod ppu;

use crate::cpu::CPU;
use crate::mmu::MMU;
use crate::ppu::PPU;

#[wasm_bindgen]
pub struct WebEmulator {
    cpu: CPU,
    ppu: PPU,
}

#[wasm_bindgen]
impl WebEmulator {
    #[wasm_bindgen(constructor)]
    pub fn new(rom_data: Vec<u8>) -> Self {
        let mmu = MMU::new(rom_data, "web_save.sav");
        Self { 
            cpu: CPU::new(mmu),
            ppu: PPU::new() 
        }
    }

    pub fn update_frame(&mut self) {
        let mut cycles_this_frame = 0;
        while cycles_this_frame < 70224 {
            let cycles = self.cpu.step() as u32;
            cycles_this_frame += cycles;
            self.ppu.tick(&mut self.cpu.bus, cycles as u8);
        }
    }

    pub fn get_screen_ptr(&self) -> *const u32 {
        self.ppu.frame_buffer.as_ptr()
    }
    
    // ADD THIS METHOD
    pub fn get_frame_buffer(&self) -> Vec<u32> {
        self.ppu.frame_buffer.to_vec()
    }
}