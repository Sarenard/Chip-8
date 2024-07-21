extern crate sdl;
use sdl::video::{SurfaceFlag, VideoFlag};
use sdl::event::{Event, Key};
use sdl::Rect;

use std;

mod chip8;

use chip8::vm::{
    KeyboardHandler,
    PixelHandler,
};

struct BasicPixelHandler<'a> {
    screen: &'a mut sdl::video::Surface,
}

impl<'a> PixelHandler for BasicPixelHandler<'a> {
    fn set_pixel(&mut self, x: usize, y: usize, on: bool) {
        let rect = Rect {
            x: (x as isize * SIZE) as i16,
            y: (y as isize * SIZE) as i16,
            w: SIZE as u16,
            h: SIZE as u16,
        };

        let color = if on { sdl::video::RGB(255, 255, 255) } else { sdl::video::RGB(0, 0, 0) };
        self.screen.fill_rect(Some(rect), color);
        self.screen.flip();
    }
}

// Example struct implementing the KeyboardHandler trait
struct BasicKeyboardHandler;

impl KeyboardHandler for BasicKeyboardHandler {
    fn is_pressed(&mut self, key: u8) -> bool {
        // Example implementation for checking if a key is pressed
        println!("Checking if key {} is pressed", key);
        false // For example purposes, always return false
    }
}

static SIZE: isize = 10;

fn main() {
    // TODO : take a file path in argument and differenciate ch8/a8

    sdl::init(&[sdl::InitFlag::Video]);
    sdl::wm::set_caption("Chip-8", "rust-sdl");

    let mut screen = match sdl::video::set_video_mode(64 * SIZE, 32 * SIZE, 32,
                                                  &[SurfaceFlag::HWSurface],
                                                  &[VideoFlag::DoubleBuf]) {
        Ok(screen) => screen,
        Err(err) => panic!("failed to set video mode: {}", err)
    };

    let pixel_handler = BasicPixelHandler {
        screen: &mut screen
    };
    let keyboard_handler = BasicKeyboardHandler;

    let mut vm = chip8::vm::VM ::new(
        pixel_handler,
        keyboard_handler
    );

    // read bytes from file
    let content = std::fs::read("roms/IBM Logo.ch8").unwrap();
    println!("content : {:?}", content);

    vm.setmemory(content);

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
