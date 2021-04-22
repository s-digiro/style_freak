extern crate json;
extern crate ncurses;
extern crate mpd;

pub use style_view_screen::StyleViewScreen;

mod style_view_screen;

mod menu;

use menu::Menu;
use menu::StyleMenu;
use menu::Item;

use mpd::Client;

use crate::style_tree::StyleTree;

pub trait Screen {
    fn name(&self) -> &str;

    fn input(&mut self, _ch: i32, _mpd_conn: &mut Client, _style_tree: &StyleTree, _display: ncurses::WINDOW) {
    }

    fn draw(&self, win_h: i32, win_w: i32);

    fn on_entrance(&mut self, _mpd_conn: &mut Client, _style_tree: &StyleTree) {
    }

    fn on_tick(&mut self, _mpd_conn: &mut Client, _style_tree: &StyleTree) {
    }
}
