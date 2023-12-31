use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::keyboard::Keycode;
use crate::cpu::CPU;

pub fn handle_user_input(cpu: &mut CPU, event_pump: &mut EventPump) {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                std::process::exit(0)
            }
            Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                cpu.memory_write(0xff, 0x77);
            }
            Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                cpu.memory_write(0xff, 0x73);
            }
            Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                cpu.memory_write(0xff, 0x61);
            }
            Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                cpu.memory_write(0xff, 0x64);
            }
            _ => {}
        }
    }
}