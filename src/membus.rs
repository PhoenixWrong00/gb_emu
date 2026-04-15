use crate::gpu::*;

pub struct MemoryBus {
    memory: [u8; 0x10000],
    pub gpu: GPU,
}

impl MemoryBus {
    pub fn new() -> Self {
        MemoryBus { memory: [0; 0x10000],
                    gpu: GPU::new(),
        }
    }

    pub fn read_byte(&self, address:u16) -> u8 {
        let address = address as usize;
        match address {
            VRAM_BEGIN ..= VRAM_END => {
                self.gpu.read_vram(address - VRAM_BEGIN)
            } _ => {
                self.memory[address as usize]
            }
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        let address = address as usize;
        match address {
            VRAM_BEGIN ..= VRAM_END => {
                self.gpu.write_vram(address - VRAM_BEGIN, value);
            }
            _ => {
                self.memory[address as usize] = value;
            }
        }
    }

    pub fn read_word(&self, address: u16) -> u16 {
        let low = self.read_byte(address) as u16;
        let high = self.read_byte(address.wrapping_add(1)) as u16;
        (high << 8) | low
    }

    pub fn write_word(&mut self, address: u16, value: u16) {
        let low = (value & 0xFF) as u8;
        let high = ((value >> 8) & 0xFF) as u8;
        self.write_byte(address, low);
        self.write_byte(address.wrapping_add(1), high);
    }
}
