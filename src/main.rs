mod inputs;
mod matrix;
mod state;
mod states;

use std::sync::Arc;
use std::sync::Mutex;

use inputs::bh1750::BH1750;
use inputs::joy_featherwing::Button;
use inputs::joy_featherwing::JoyFeatherwing;
use matrix::Matrix;
use state::State;
use states::region_select::region_select_state;
use states::time::time_state;

pub fn main() {
    let brightness_update_interval: u8 = 10;
    let mut brightness_frames_since_last_update: u8 = 0;

    let mut matrix = Matrix::new(None);

    //// initial state = RegionSelect
    let mut current_state = State::RegionSelect;

    JoyFeatherwing::init();

    //// measure brightness on seperate thread
    std::thread::spawn(move || loop {
        BH1750::measure_brightness();
    });

    // measure button presses on seperate thread
    std::thread::spawn(move || loop {
        JoyFeatherwing::measure_joy_buttons();
    });

    loop {
        if brightness_update_interval == brightness_frames_since_last_update {
            let brightness = BH1750::get_brightness();
            matrix.set_brightness(brightness);
            brightness_frames_since_last_update = 0;
        }
        brightness_frames_since_last_update += 1;

        let buttons = JoyFeatherwing::get_joy_buttons();
        for button in buttons {
            match button {
                Button::Down => println!("Down"),
                Button::Left => println!("Left"),
                Button::Right => println!("Right"),
                Button::Up => println!("Up"),
                Button::Select => println!("Select"),
                _ => {}
            }
        }

        matrix.pre_draw();

        current_state = match current_state {
            State::RegionSelect => region_select_state(&mut matrix),
            State::Time => time_state(&mut matrix),
        };

        matrix = matrix.post_draw();
    }
}
