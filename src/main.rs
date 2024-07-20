extern crate sdl;

use sdl::video::{SurfaceFlag, VideoFlag};
use sdl::event::{Event, Key};

use ux::{u12, u4};

mod chip8;

use chip8::vm::{
    KeyboardHandler,
    PixelHandler,
};

struct BasicPixelHandler;

impl PixelHandler for BasicPixelHandler {
    fn set_pixel(&mut self, x: usize, y: usize, on: bool) {
        // Example implementation for setting a pixel
        println!("Setting pixel at ({}, {}) on/off : {}", x, y, on);
    }
}

// Example struct implementing the KeyboardHandler trait
struct BasicKeyboardHandler;

impl KeyboardHandler for BasicKeyboardHandler {
    fn is_pressed(&mut self, key: u4) -> bool {
        // Example implementation for checking if a key is pressed
        println!("Checking if key {} is pressed", key);
        false // For example purposes, always return false
    }
}

static SIZE: isize = 10;

fn main() {

    sdl::init(&[sdl::InitFlag::Video]);
    sdl::wm::set_caption("Chip-8", "rust-sdl");

    let screen = match sdl::video::set_video_mode(64 * SIZE, 32 * SIZE, 32,
                                                  &[SurfaceFlag::HWSurface],
                                                  &[VideoFlag::DoubleBuf]) {
        Ok(screen) => screen,
        Err(err) => panic!("failed to set video mode: {}", err)
    };

    let pixel_handler = BasicPixelHandler;
    let keyboard_handler = BasicKeyboardHandler;

    let mut vm = chip8::vm::VM {
        bytecode: [[u4::new(0); 4]; 4096],
        memory: [0; 4096],
        stack: [0; 256],
        registers: [0; 16],
        adress: u12::new(0),
        delaytimer: 0,
        soundtimer: 0,
        keystates: [false; 16],
        pixelhandler: pixel_handler,
        keyboardhandler: keyboard_handler,
    };

    'main : loop {
        'event : loop {
            match sdl::event::poll_event() {
                Event::Quit => break 'main,
                Event::None => break 'event,
                Event::Key(k, _, _, _)
                    if k == Key::Escape
                        => break 'main,
                _ => {}
            }
        }
        // we do one tick
        vm.process();
    }

    sdl::quit();
}
