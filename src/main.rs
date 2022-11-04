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
    let mut matrix = Matrix::new(None);

    //// initial state = RegionSelect
    let mut current_state = State::RegionSelect;
    let mut frame_count: u32 = 0;

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
        let brightness = BH1750::get_brightness();
        println!("Brightness: {}", brightness);
        matrix.set_brightness(brightness);

        JoyFeatherwing::measure_joy_buttons();

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
