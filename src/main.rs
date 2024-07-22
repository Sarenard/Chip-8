extern crate sdl;
use sdl::video::{SurfaceFlag, VideoFlag};
use sdl::event::{Event, Key};
use sdl::Rect;

use std::time::{self, SystemTime, UNIX_EPOCH};

use clap::Parser;

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
    }
}

// Example struct implementing the KeyboardHandler trait
struct BasicKeyboardHandler {
    status: [bool; 16],
}

impl KeyboardHandler for BasicKeyboardHandler {
    fn is_pressed(&mut self, key: u8) -> bool {
        self.status[key as usize]
    }
}

static SIZE: isize = 10;
static FPS: u128 = 60;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path of the file to open
    #[arg(short, long)]
    file: String,
}

fn main() {
    // TODO : take a file path in argument and differenciate ch8/a8
    let args = Args::parse();

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
    let keyboard_handler = BasicKeyboardHandler {
        status: [false; 16],
    };

    let mut vm = chip8::vm::VM ::new(
        pixel_handler,
        keyboard_handler
    );

    // read bytes from file
    let content = std::fs::read(args.file).unwrap();
    println!("content : {:?}", content);

    vm.setmemory(content);

    let mut last = SystemTime::now();

    let accepted = [
        Key::Num1, Key::Num2, Key::Num3, Key::Num4,
        Key::A,    Key::Z,    Key::E,    Key::R,
        Key::Q,    Key::S,    Key::D,    Key::F,
        Key::W,    Key::X,    Key::C,    Key::V,
    ];

    'main : loop {
        'event : loop {
            match sdl::event::poll_event() {
                Event::Quit => break 'main,
                Event::None => break 'event,
                Event::Key(k, is_pressed, _, _) => {
                    if k == Key::Escape {
                        break 'main;
                    }
                    if accepted.contains(&k) {
                        let nb = accepted.iter().position(|&x| x == k).unwrap();
                        vm.keyboardhandler.status[nb] = is_pressed;
                    }
                }
                _ => {}
            }
        }
        // we do one tick
        let time = SystemTime::now().duration_since(last).unwrap();
        if time.as_millis() > 1000 / FPS {// 16/60, 60FPS
            last = SystemTime::now();
            vm.decrease_timer();
        }
        vm.process();
        vm.pixelhandler.screen.flip();
        println!("{:?}", vm.keyboardhandler.status);
    }

    sdl::quit();
}
