mod matrix;
mod state;
mod states;

use matrix::Matrix;
use state::State;
use states::region_select::region_select_state;
use states::time::time_state;

pub fn main() {
    let mut matrix = Matrix::new();

    // initial state = RegionSelect
    let mut current_state = State::RegionSelect;
    let mut frame_count: u32 = 0;

    loop {
        matrix.pre_draw();

        current_state = match current_state {
            State::RegionSelect => region_select_state(&mut matrix),
            State::Time => time_state(&mut matrix),
        };

        matrix = matrix.post_draw();
    }
}
