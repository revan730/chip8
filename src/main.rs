#![feature(int_roundings)]

#[macro_use]
extern crate lazy_static;

extern crate sdl2;

mod cpu;
mod instructions;
mod opcode_decoders;
mod drawable;
mod audible;
mod dummy_screen;
mod sdl_screen;
mod sdl_sound_device;
mod font;
mod constants;

use cpu::Cpu;
use font::FONT_TABLE;
use sdl2::audio::AudioDevice;
use sdl2::audio::AudioSpecDesired;
use sdl_sound_device::SDLSoundDevice;
use std::fs;
use std::io;
use std::env;
use std::process::{exit};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

use crate::constants::{SCREEN_WIDTH, SCREEN_HEIGHT};
use crate::drawable::Drawable;
use crate::audible::Audible;


fn find_sdl_gl_driver() -> Option<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            return Some(index as u32);
        }
    }
    None
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("args: {:?}", args);
    let rom_data = match load_file(&args[1]) {
        Err(e) => {
            println!("Failed to read ROM: {:?}", e);
            exit(123)
        },
        Ok(data) => data,
    };

    if rom_data.len() > 4096 - 512 {
        println!("File too large");
        exit(124)
    }

    let mut rom_correct_endianess = file_data_to_rom_layout(rom_data);
    fill_font_data(&mut rom_correct_endianess);


    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("CHIP-8", SCREEN_WIDTH as u32 * 20, SCREEN_HEIGHT as u32 * 20)
    .position_centered()
    .opengl()
    .build().unwrap();

    let mut canvas = window.into_canvas().index(find_sdl_gl_driver().unwrap()).present_vsync().build().unwrap();
    canvas.set_logical_size(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32).unwrap();
    canvas.clear();
    canvas.present();

    let mut screen = Box::new(sdl_screen::SDLScreen::new(canvas)) as Box<dyn Drawable>;
    let mut sdl_audio_device = create_audio_device(&sdl_context);
    

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut cpu: Cpu = Cpu::new();
    cpu.load_rom(rom_correct_endianess);

    let mut pressed_keys = [0 as u8; 16];

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { timestamp } => {
                    break 'running;
                },
                Event::KeyDown { timestamp, window_id, keycode: Some(keycode), scancode, keymod, repeat } => {
                    match keycode {
                        Keycode::Num1 => pressed_keys[0x1] = 1,
                        Keycode::Num2 => pressed_keys[0x2] = 1,
                        Keycode::Num3 => pressed_keys[0x3] = 1,
                        Keycode::Num4 => pressed_keys[0xC] = 1,
                        Keycode::Q => pressed_keys[0x4] = 1,
                        Keycode::W => pressed_keys[0x5] = 1,
                        Keycode::E => pressed_keys[0x6] = 1,
                        Keycode::R => pressed_keys[0xD] = 1,
                        Keycode::A => pressed_keys[0x7] = 1,
                        Keycode::S => pressed_keys[0x8] = 1,
                        Keycode::D => pressed_keys[0x9] = 1,
                        Keycode::F => pressed_keys[0xE] = 1,
                        Keycode::Z => pressed_keys[0xA] = 1,
                        Keycode::X => pressed_keys[0x0] = 1,
                        Keycode::C => pressed_keys[0xB] = 1,
                        Keycode::V => pressed_keys[0xF] = 1,
                        _ => {},
                    };
                },
                Event::KeyUp { timestamp, window_id, keycode: Some(keycode), scancode, keymod, repeat } => {
                    match keycode {
                        Keycode::Num1 => pressed_keys[0x1] = 0,
                        Keycode::Num2 => pressed_keys[0x2] = 0,
                        Keycode::Num3 => pressed_keys[0x3] = 0,
                        Keycode::Num4 => pressed_keys[0xC] = 0,
                        Keycode::Q => pressed_keys[0x4] = 0,
                        Keycode::W => pressed_keys[0x5] = 0,
                        Keycode::E => pressed_keys[0x6] = 0,
                        Keycode::R => pressed_keys[0xD] = 0,
                        Keycode::A => pressed_keys[0x7] = 0,
                        Keycode::S => pressed_keys[0x8] = 0,
                        Keycode::D => pressed_keys[0x9] = 0,
                        Keycode::F => pressed_keys[0xE] = 0,
                        Keycode::Z => pressed_keys[0xA] = 0,
                        Keycode::X => pressed_keys[0x0] = 0,
                        Keycode::C => pressed_keys[0xB] = 0,
                        Keycode::V => pressed_keys[0xF] = 0,
                        _ => {},
                    };
                },
                _ => {}
            }
        }

        cpu.tick(&mut sdl_audio_device);
        cpu.step(&mut screen, &mut sdl_audio_device, &pressed_keys);
        println!("{:?}", cpu);

        screen.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}


fn load_file(path: &str) -> io::Result<Vec<u8>> {
    fs::read(path)
}

fn file_data_to_rom_layout(data: Vec<u8>) -> [u8; 4096] {
    let mut resulting_array: [u8;4096] = [0;4096];
    let mut array_pos = 512; // First 512 bytes are reserved

    for byte in data.into_iter() {
        resulting_array[array_pos] = byte;
        array_pos += 1;
    }

    resulting_array
}

fn fill_font_data(data: &mut [u8; 4096]) {
    let mut i = 0;
    for character in FONT_TABLE.into_iter() {
        for byte in character.into_iter() {
            data[i] = *byte;
            i += 1;
        }
    }
}

fn create_audio_device(sdl_context: &sdl2::Sdl) -> Box<dyn Audible> {
    let device = SDLSoundDevice::new(sdl_context);

    Box::new(device) as Box<dyn Audible>
}