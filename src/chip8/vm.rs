use core::panic;

use crate::chip8;

use crate::chip8::insts::Instruction;

pub trait PixelHandler {
    fn set_pixel(&mut self, x: usize, y: usize, on: bool);
}

pub trait KeyboardHandler {
    fn is_pressed(&mut self, key: u8) -> bool;
}

static FONT: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct VM<T: PixelHandler, T2: KeyboardHandler> {
    pub memory: [u8; 4096], // 4096 bytes
    registers: [u8; 16], // 8-bit data registers
    stack: [u16; 16],
    nbstack: usize,
    i: u16,
    programcounter: usize,
    delaytimer: u8,
    soundtimer: u8,
    keystates: [bool; 16], // what keys are pressed
    pixelhandler: T,
    keyboardhandler: T2,
    framebuffer: [[bool; 32]; 64],
}

impl<T: PixelHandler, T2: KeyboardHandler> VM<T, T2> {
    pub fn new(pixelhandler: T, keyboardhandler: T2) -> Self {
        let mut memory: [u8; 4096] = [0; 4096];
        memory[..FONT.len()].copy_from_slice(&FONT);
        VM {
            memory: memory,
            registers: [0; 16],
            stack: [0; 16],
            nbstack: 0,
            i: 0,
            programcounter: 0x200, // start of programs
            delaytimer: 0,
            soundtimer: 0,
            keystates: [false; 16],
            pixelhandler,
            keyboardhandler,
            framebuffer: [[false; 32]; 64],
        }
    }

    pub fn update_pixel(&mut self, x: usize, y: usize, forceblack: bool) {
        if forceblack {
            self.pixelhandler.set_pixel(x, y, false);
            self.framebuffer[x][y] = false;
            return;
        }
        self.pixelhandler.set_pixel(x, y, !self.framebuffer[x][y]);
        self.framebuffer[x][y] = !self.framebuffer[x][y];
    }

    pub fn check_key(&mut self, key: u8) -> bool {
        self.keyboardhandler.is_pressed(key)
    }

    pub fn setmemory(&mut self, content: Vec<u8>) {
        let max_memory_size = self.memory.len() - 0x200; // Reserve the first 512 bytes for system area and font set
    
        let memory_size = std::cmp::min(content.len(), max_memory_size);
    
        self.memory[0x200..0x200 + memory_size].copy_from_slice(&content[..memory_size]);
    }

    pub fn process(&mut self) {
        let instruction1: u8 = self.memory[self.programcounter];
        let instruction2: u8 = self.memory[self.programcounter + 1];
        let instruction = ((instruction1 as u16) << 8) | instruction2 as u16;

        let instruction = chip8::insts::Instruction::new(instruction);

        println!("instruction : {:?}", instruction);

        match instruction {
            Instruction::ClearScreen => {
                for i in 0..64 {
                    for j in 0..32 {
                        self.update_pixel(i, j, true);
                    }
                }
            },

            Instruction::Jump(val) => {
                self.programcounter = val as usize - 2;
            }

            Instruction::SetRegister(reg, val) => {
                self.registers[reg as usize] = val;
            }

            Instruction::AddRegister(reg, val) => {
                self.registers[reg as usize] += val;
            }

            Instruction::SetI(val) => {
                self.i = val;
            }

            Instruction::Draw(reg1, reg2, size) => {
                let x = self.registers[reg1 as usize] % 64; // wrap
                let y = self.registers[reg2 as usize] % 32; // wrap
                self.registers[15] = 0;
                for i in 0..size { // pour chaque ligne
                    if y + i as u8 > 32 { // if we are outside of the screen
                        break;
                    }
                    let byte = self.memory[self.i as usize + i as usize];
                    for off in 0..8 {
                        let bit = ((byte & (0x1 << off)) >> off) == 1;
                        if bit {
                            self.update_pixel((x+8-off) as usize, (y+i as u8) as usize, false);
                        }
                    }
                }
            }

            Instruction::ERROR(nb) => {
                panic!("Bytecode not understood : {}", nb);
            }
        }

        self.programcounter += 2;
    }
}
