use ux::{u12, u4};

pub trait PixelHandler {
    fn set_pixel(&mut self, x: usize, y: usize, on: bool);
}

pub trait KeyboardHandler {
    fn is_pressed(&mut self, key: u4) -> bool;
}

pub struct VM<T: PixelHandler, T2: KeyboardHandler> {
    pub bytecode: [[u4; 4]; 4096],
    pub memory: [u8; 4096], // 4096 bytes
    pub registers: [u8; 16], // 8-bit data registers
    pub stack: [u64; 256],
    pub adress: u12,
    pub delaytimer: u8,
    pub soundtimer: u8,
    pub keystates: [bool; 16], // what keys are pressed
    pub pixelhandler: T,
    pub keyboardhandler: T2,
}

impl<T: PixelHandler, T2: KeyboardHandler> VM<T, T2> {
    pub fn update_pixel(&mut self, x: usize, y: usize, on: bool) {
        self.pixelhandler.set_pixel(x, y, on);
    }

    pub fn check_key(&mut self, key: u4) -> bool {
        self.keyboardhandler.is_pressed(key)
    }

    pub fn process(&mut self) {

    }
}
