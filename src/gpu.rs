pub const VRAM_BEGIN: usize = 0x8000;
pub const VRAM_END: usize = 0x9FFF;
pub const VRAM_SIZE: usize = VRAM_END - VRAM_BEGIN + 1;

#[derive(Copy, Clone)]
pub enum TilePixelValue {
    Zero,
    One,
    Two,
    Three,
}

type Tile = [[TilePixelValue; 8]; 8];
fn empty_tile() -> Tile {
    [[TilePixelValue::Zero; 8]; 8]
}

pub struct GPU {
    pub vram: [u8; VRAM_SIZE],
    pub tile_set: [Tile; 384],
}

impl GPU {
    pub fn new() -> Self {
        GPU {
            vram: [0; VRAM_SIZE],
            tile_set: [empty_tile(); 384],
        }
    }

    pub fn read_vram(&self, address: usize) -> u8 {
        self.vram[address]
    }

    pub fn write_vram(&mut self, address: usize, value: u8) {
        self.vram[address] = value;
    }
}
