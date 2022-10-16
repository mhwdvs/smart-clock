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
    //let matrix = Matrix::new(None);
    //let buffer_matrix = Matrix::new(None);

    //// initial state = RegionSelect
    //let mut current_state = State::RegionSelect;
    //let mut frame_count: u32 = 0;

    //// measure brightness on seperate thread
    //std::thread::spawn(move || loop {
    //    BH1750::measure_brightness();
    //});

    // measure button presses on seperate thread
    std::thread::spawn(move || loop {
        JoyFeatherwing::measure_joy_buttons();
    });

    //let mut brightness_update = 0;
    //loop {
    //    if brightness_update == 10 {
    //        let brightness = BH1750::get_brightness();
    //        println!("{}", brightness);

    //        matrix = buffer_matrix.set_brightness(brightness);
    //        brightness_update = 0;
    //    } else {
    //        brightness_update += 1;
    //    }

    //    matrix.pre_draw();

    //    current_state = match current_state {
    //        State::RegionSelect => region_select_state(&mut matrix),
    //        State::Time => time_state(&mut matrix),
    //    };

    //    matrix = matrix.post_draw();
    //}
    JoyFeatherwing::init();

    loop {
        // clear output
        print!("{esc}c", esc = 27 as char);

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
    }
}
