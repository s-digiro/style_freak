extern crate json;
extern crate mpd;
extern crate ncurses;

mod style_tree;
mod screen;
mod colors;

use std::env;

use mpd::Client;

use ncurses::WINDOW;

use colors::*;

use style_tree::StyleTree;
use style_tree::Style;

use screen::Screen;
use screen::StyleViewScreen;

fn main() {
    let args: Vec<String> = env::args().collect();

    let style_path = match args.get(1) {
        Some(path) => path,
        None => {
            eprintln!("usage: {} <path> [ip:port]", args[0]);
            eprintln!("  If no ip:port supplied, 127.0.0.1:6600 will be used");
            return
        },
    };

    let style_tree = match StyleTree::load_from_file(style_path) {
        Ok(tree) => tree,
        Err(e) => {
            eprintln!("Error: Could not open style_tree");
            eprintln!("  {}", e);
            return
        },
    };

    let portip = match args.get(2) {
        Some(portip) => portip,
        None => "127.0.0.1:6600",
    };

    let mut mpd_conn = match Client::connect(portip) {
        Ok(conn) => conn,
        Err(_) => {
            eprintln!("Error: Could not connect to mpd");
            return
        },
    };

    let term = init_ncurses();

    colors::init_colors();
    ncurses::attron(ncurses::COLOR_PAIR(SBC_DEFAULT));


    let mut screen = StyleViewScreen::new();

    screen.on_tick(&mut mpd_conn, &style_tree);
    screen.on_entrance(&mut mpd_conn, &style_tree);
    screen.draw(ncurses::getmaxy(term), ncurses::getmaxx(term));

    loop {
        let ch = ncurses::getch();
        if ch != ncurses::ERR {
            match ch {
                ncurses::KEY_RESIZE => {
                    ncurses::erase();
                },
                113 => break, // q
                _ => screen.input(ch, &mut mpd_conn, &style_tree, term),
            }
        }

        ncurses::flushinp();

        ncurses::erase();
        screen.draw(ncurses::getmaxy(term), ncurses::getmaxx(term));

        ncurses::refresh();

        screen.on_tick(&mut mpd_conn, &style_tree);
    }

    shutdown_ncurses(term);
}

fn init_ncurses() -> WINDOW {
        let window = ncurses::initscr();

        ncurses::start_color();
        ncurses::cbreak();
        ncurses::keypad(window, true);
        ncurses::nodelay(window, true);
        ncurses::noecho();
        ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);
        ncurses::setlocale(ncurses::LcCategory::all, "");

        window
}

fn shutdown_ncurses(window: WINDOW) {
    ncurses::nocbreak();
    ncurses::keypad(window, false);
    ncurses::nodelay(window, false);
    ncurses::echo();
    ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_VISIBLE);
    ncurses::endwin();
}
