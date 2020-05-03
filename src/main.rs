mod chip8;
mod display;
use chip8::{Chip8, WIDTH, HEIGHT};
use std::process;
use std::{thread, time};
use std::io;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::pixels::Color;
use sdl2::rect::Rect;



fn main() {
    println!("CHIP-8 emulator starting...");
    let mut display = display::Display::initialize();
    
    // create chip8 instance
    let mut chip8 = Chip8::new();

    if let Err(e) = chip8.load_rom("Roms\\TANK") {
        eprintln!("Couldn't load ROM: {}", e);

        process::exit(1);
    }
    
    'main: loop {
        // execute a cpu cycle (one instruction)
        if let Err(e) = chip8.cycle() {
            eprintln!("Execution error: {}", e)
        }

        if chip8.draw {
            // display.draw(*chip.display)
            chip8.draw = false;
        }
        for event in display.event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'main
                },
                // handle clicks
                _ => {}
            }
        }
        display.canvas.present(); //show
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));

        // if chip8.sound_timer > 0 {
        //     println!("BEEP!");
        // }
        // if chip8.draw {
        //     for yline in 0..HEIGHT {
        //         for xline in 0..WIDTH {
        //             let pixel = chip8.display[yline][xline];
        //             if pixel == 1 {
        //                 print!("*");
        //             }
        //             else {
        //                 print!(" ");
        //             }
        //         }
        //         println!();
        //     }
        //     chip8.draw = false;
        //     //Wait for input to proceed
        //     let mut input = String::from("");
        //     io::stdin()
        //         .read_line(&mut input)
        //         .ok()
        //         .expect("Couldn't read line");
        //     // let ten_millis = time::Duration::from_millis(100);
        //     // thread::sleep(ten_millis);
        // }
    }
}  