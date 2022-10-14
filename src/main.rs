mod inputs;
mod matrix;
mod state;
mod states;

use inputs::bh1750::BH1750;
use matrix::Matrix;
use state::State;
use states::region_select::region_select_state;
use states::time::time_state;

pub fn main() {
    let mut matrix = Matrix::new();

    // initial state = RegionSelect
    let mut current_state = State::RegionSelect;
    let mut frame_count: u32 = 0;

    // measure brightness on seperate thread
    std::thread::spawn(move || loop {
        BH1750::measure_brightness();
    });

    loop {
        matrix.pre_draw();

        let brightness = BH1750::get_brightness();

        current_state = match current_state {
            State::RegionSelect => region_select_state(&mut matrix),
            State::Time => time_state(&mut matrix),
        };

        matrix = matrix.post_draw();

        println!("{}", brightness);
    }
}
