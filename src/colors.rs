extern crate ncurses;

use ncurses::*;

pub const SBC_DEFAULT: i16 = 0;
pub const SBC_CYAN: i16 = 1;
pub const SBC_BLACK: i16 = 2;
pub const SBC_YELLOW: i16 = 3;

pub fn init_colors() {
    use_default_colors();
    init_pair(SBC_CYAN, COLOR_CYAN, COLOR_BLACK);
    init_pair(SBC_BLACK, COLOR_BLACK, COLOR_BLACK);
    init_pair(SBC_YELLOW, COLOR_YELLOW, COLOR_BLACK);
}
