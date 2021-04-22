extern crate ncurses;

use std::borrow::Cow;

use mpd::Client;
use mpd::Query;
use mpd::Term;
use mpd::Song;

use crate::StyleTree;
use crate::Style;
use crate::colors::*;

use super::Screen;
use super::StyleMenu;
use super::Menu;
use super::Item;

#[derive(PartialEq, Clone)]
enum State {
    Style(usize),
    Artist,
    Album,
    Track,
}

pub struct StyleViewScreen {
    state: State,
    name: String,
    styles: Vec<StyleMenu>,

    artists: Menu,
    albums: Menu,
    tracks: Menu,

    songs: Vec<Song>,
    names: Vec<String>,
}

impl StyleViewScreen {
    pub fn new() -> StyleViewScreen {
        StyleViewScreen {
            name: String::from("Style View"),
            state: State::Style(0),
            styles: vec![],
            artists: Menu::new(),
            albums: Menu::new(),
            tracks: Menu::new(),
            songs: Vec::new(),
            names: vec![
                "Type".to_string(),
                "Style".to_string(),
                "Sub-Style".to_string(),
                "Genre".to_string(),
                "Sub-Genre".to_string(),
                "Tracks".to_string(),
            ],
        }
    }

    fn next_menu_empty(&self) -> bool {
        match self.next_state() {
            Some(state) => match state {
                State::Style(i) => self.styles.get(i).unwrap().is_empty(),
                State::Artist => self.artists.is_empty(),
                State::Album => self.albums.is_empty(),
                State::Track => self.tracks.is_empty(),
            },
            None => true,
        }
    }

    fn next_state(&self) -> Option<State> {
        match self.state {
            State::Style(i) => match self.styles.get(i + 1) {
                Some(_) => Some(State::Style(i + 1)),
                None => Some(State::Artist),
            },
            State::Artist => Some(State::Album),
            State::Album => Some(State::Track),
            State::Track => None,
        }
    }

    fn add_tracks(
        &self,
        mpd_conn: &mut Client,
        tree: &StyleTree,
        genres: Vec<Style>,
        artist: Option<&str>,
        album: Option<&str>,
        track: Option<&str>
    ) {

        let mut tracks = Vec::new();

        for genre in genres {
            let mut query = Query::new();
            let mut query_ref = &mut query;

            if let Some(artist) = artist {
                query_ref = query_ref.and(
                    Term::Tag(Cow::Borrowed("albumartist")),
                    artist
                );
            }

            if let Some(album) = album {
                query_ref = query_ref.and(
                    Term::Tag(Cow::Borrowed("Album")),
                    album
                );
            }

            if let Some(track) = track {
                query_ref = query_ref.and(
                    Term::Tag(Cow::Borrowed("Track")),
                    track
                );
            }

            tracks.append(
                &mut mpd_conn.find(
                    query_ref.and(
                        Term::Tag(Cow::Borrowed("genre")),
                        tree.name(genre),
                    ),
                    None
                ).unwrap()
            );
        }

        for track in tracks {
            mpd_conn.push(track).unwrap();
        }
    }

    fn prev_state(&self) -> Option<State> {
        match self.state {
            State::Style(i) => {
                if i as i32 - 1 >= 0 {
                    Some(State::Style(i - 1))
                } else {
                    None
                }
            },
            State::Artist => Some(State::Style(self.styles.len() - 1)),
            State::Album => Some(State::Artist),
            State::Track => Some(State::Album),
        }
    }

    fn fetch_styles(&mut self, tree: &StyleTree) -> bool {
        let parents = if let Some(menu) = self.styles.last() {
            menu.style_selection()
        } else {
            vec![0]
        };

        let mut children = Vec::new();
        for parent in parents {
            children.append(&mut tree.children(parent));
        }

        if children.is_empty() {
            true
        } else {
            let mut new_menu = StyleMenu::new();
            new_menu.set_styles(children, tree);

            self.styles.push(new_menu);

            false
        }
    }

    fn fetch_artists(&mut self, mpd_conn: &mut Client, tree: &StyleTree) {
        let genres = self.styles.last().unwrap().style_selection();

        let mut artists = Vec::new();

        for genre in genres {
            artists.append(
                &mut mpd_conn.list(
                    &Term::Tag(Cow::Borrowed("albumartist")),
                    Query::new().and(
                        Term::Tag(Cow::Borrowed("genre")),
                        tree.name(genre),
                    ),
                ).unwrap()
            );
        }

        self.artists.set_items(
            artists.iter().map(|s| Item::from(s)).collect()
        );
    }

    fn fetch_albums(&mut self, mpd_conn: &mut Client, tree: &StyleTree) {
        let genres = self.styles.last().unwrap().style_selection();

        let artist = self.artists.sel().val();

        let mut albums = Vec::new();

        for genre in genres {
            albums.append(
                &mut mpd_conn.list(
                    &Term::Tag(Cow::Borrowed("Album")),
                    Query::new().and(
                        Term::Tag(Cow::Borrowed("genre")),
                        tree.name(genre),
                    ).and(
                        Term::Tag(Cow::Borrowed("albumartist")),
                        artist,
                    ),
                ).unwrap()
            );
        }

        self.albums.set_items(
            albums.iter().map(|s| Item::from(s)).collect()
        );
    }

    fn fetch(&mut self, mpd_conn: &mut Client, tree: &StyleTree) {
        match self.state {
            State::Style(_) => {
                let result = self.fetch_styles(tree);
                if result {
                    self.fetch_artists(mpd_conn, tree);
                }
            },
            State::Artist => {
                self.fetch_albums(mpd_conn, tree);
            },
            State::Album => {
                self.fetch_tracks(mpd_conn, tree);
            },
            State::Track => (),
        }
    }

    fn fetch_tracks(&mut self, mpd_conn: &mut Client, tree: &StyleTree) {
        let genres = self.styles.last().unwrap().style_selection();

        let artist = self.artists.sel().val();

        let album = self.albums.sel().val();

        let mut new_items = Vec::new();

        for genre in genres {
            new_items.append(
                &mut mpd_conn.find(
                    Query::new().and(
                        Term::Tag(Cow::Borrowed("genre")),
                        tree.name(genre),
                    ).and(
                        Term::Tag(Cow::Borrowed("albumartist")),
                        artist,
                    ).and(
                        Term::Tag(Cow::Borrowed("album")),
                        album,
                    ),
                    None
                ).unwrap()
            );
        }

        self.songs = new_items;

        let new_items = self.songs.iter()
            .map(|s| match &s.title {
                Some(title) => Item::from(title),
                None => Item::Empty,
            }).collect();

        self.tracks.set_items(new_items);
    }
}

impl Screen for StyleViewScreen {
    fn name(&self) -> &str { &self.name }

    fn input(&mut self, ch: i32, mpd_client: &mut Client, tree: &StyleTree, display: ncurses::WINDOW) {
        match ch {
             47 => { // /
                let mut input = String::new();
                ncurses::nocbreak();
                ncurses::echo();
                ncurses::nodelay(display, false);
                ncurses::keypad(display, false);
                ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_VISIBLE);
                ncurses::mv(0, 0);
                ncurses::getstr(&mut input);

                match self.state {
                    State::Style(i) => {
                        self.styles[i].search(&input);
                        self.styles.truncate(i + 1);
                    },
                    State::Artist => self.artists.search(&input),
                    State::Album => self.albums.search(&input),
                    State::Track => self.tracks.search(&input),
                }
                self.fetch(mpd_client, tree);

                ncurses::cbreak();
                ncurses::keypad(display, true);
                ncurses::noecho();
                ncurses::nodelay(display, true);
                ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);
            },
            32 => match self.state { // Space
                State::Style(_) => {
                    let genres = self.styles.last().unwrap().style_selection();
                    self.add_tracks(mpd_client, tree, genres, None, None, None);
                },
                State::Artist => {
                    let genres = self.styles.last().unwrap().style_selection();
                    let artist = self.artists.sel().val();
                    self.add_tracks(mpd_client, tree, genres, Some(artist), None, None);
                },
                State::Album => {
                    let genres = self.styles.last().unwrap().style_selection();
                    let artist = self.artists.sel().val();
                    let album = self.albums.sel().val();
                    self.add_tracks(mpd_client, tree, genres, Some(artist), Some(album), None);
                },
                State::Track => {
                    let song = self.songs.get(self.tracks.i()).unwrap();
                    mpd_client.push(song).unwrap();
                },
            },
            104 | ncurses::KEY_LEFT => if let Some(state) = self.prev_state() { // h
                self.state = state;
            },
            108 | ncurses::KEY_RIGHT => if !self.next_menu_empty() { // l
                self.state = self.next_state().unwrap();
                self.fetch(mpd_client, tree);
            },
            106 | ncurses::KEY_DOWN => {
                match self.state { // j
                    State::Style(i) => {
                        self.styles[i].next();
                        self.styles.truncate(i + 1);
                    }
                    State::Artist => self.artists.next(),
                    State::Album => self.albums.next(),
                    State::Track => self.tracks.next(),
                }
                self.fetch(mpd_client, tree)
            },
            107 | ncurses::KEY_UP => {
                match self.state { // k
                    State::Style(i) => {
                        self.styles[i].prev();
                        self.styles.truncate(i + 1);
                    },
                    State::Artist => self.artists.prev(),
                    State::Album => self.albums.prev(),
                    State::Track => self.tracks.prev(),
                }
                self.fetch(mpd_client, tree)
            },
            _ => (),
        }
    }

    fn on_entrance(&mut self, mpd_conn: &mut Client, tree: &StyleTree) {
            self.fetch_styles(tree);

            let state = self.state.clone();
            for _ in 0..3 {
                match self.state {
                    State::Style(_) => {
                        if !self.styles.last().unwrap().is_empty() {
                            self.fetch_styles(tree);
                        } else {
                            self.styles.pop();
                            self.fetch_artists(mpd_conn, tree);
                            self.state = State::Artist;
                        }
                    },
                    State::Artist => {
                        self.fetch_albums(mpd_conn, tree);
                        self.state = State::Album;
                    },
                    State::Album => {
                        self.fetch_tracks(mpd_conn, tree);
                        self.state = State::Track;
                    },
                    State::Track => {
                        break;
                    }
                }
            }
            self.state = state;
    }

    fn draw(&self, win_h: i32, win_w: i32) {
        let menu_w = (win_w / 3) - 1;
        let menu_h = win_h - 2;

        ncurses::mvhline(1, 0, ncurses::ACS_HLINE(), win_w);

        match self.prev_state() {
            Some(state) => match state {
                State::Style(i) => {
                    ncurses::mvaddnstr(0, 0, &self.names[i], menu_w);
                    ncurses::attron(ncurses::COLOR_PAIR(SBC_CYAN));
                    self.styles[i].draw(1, 0, menu_h, menu_w);
                }
                State::Artist => {
                    ncurses::mvaddnstr(0, 0, "Artist", menu_w);
                    ncurses::attron(ncurses::COLOR_PAIR(SBC_CYAN));
                    self.artists.draw(1, 0, menu_h, menu_w);
                },
                State::Album => {
                    ncurses::mvaddnstr(0, 0, "Album", menu_w);
                    ncurses::attron(ncurses::COLOR_PAIR(SBC_CYAN));
                    self.albums.draw(1, 0, menu_h, menu_w);
                },
                State::Track => {
                    ncurses::mvaddnstr(0, 0, "Track", menu_w);
                    ncurses::attron(ncurses::COLOR_PAIR(SBC_CYAN));
                    self.tracks.draw(1, 0, menu_h, menu_w);
                }
            },
            None => (),
        }
        ncurses::attroff(ncurses::COLOR_PAIR(SBC_CYAN));
        ncurses::mvvline(0, menu_w, ncurses::ACS_VLINE(), win_h);

        match self.state {
            State::Style(i) => {
                ncurses::mvaddnstr(0, menu_w + 1, &self.names[i], menu_w);
                ncurses::attron(ncurses::COLOR_PAIR(SBC_YELLOW));
                self.styles[i].draw(1, menu_w + 1, menu_h, menu_w);
            },
            State::Artist => {
                ncurses::mvaddnstr(0, menu_w+1, "Artist", menu_w);
                ncurses::attron(ncurses::COLOR_PAIR(SBC_YELLOW));
                self.artists.draw(1, menu_w+1, menu_h, menu_w);
            },
            State::Album => {
                ncurses::mvaddnstr(0, menu_w+1, "Album", menu_w);
                ncurses::attron(ncurses::COLOR_PAIR(SBC_YELLOW));
                self.albums.draw(1, menu_w+1, menu_h, menu_w);
            },
            State::Track => {
                ncurses::mvaddnstr(0, menu_w+1, "Track", menu_w);
                ncurses::attron(ncurses::COLOR_PAIR(SBC_YELLOW));
                self.tracks.draw(1, menu_w+1, menu_h, menu_w);
            }
        }
        ncurses::attroff(ncurses::COLOR_PAIR(SBC_YELLOW));
        ncurses::mvvline(0, 2 * menu_w + 1, ncurses::ACS_VLINE(), win_h);

        match self.next_state() {
            Some(state) => match state {
                State::Style(i) => {
                    ncurses::mvaddnstr(0, 2 * menu_w + 2, &self.names[i], menu_w);
                    ncurses::attron(ncurses::COLOR_PAIR(SBC_CYAN));
                    self.styles[i].draw(1, 2 * menu_w + 2, menu_h, menu_w);
                },
                State::Artist => {
                    ncurses::mvaddnstr(0, 2 * menu_w + 2, "Artist", menu_w);
                    ncurses::attron(ncurses::COLOR_PAIR(SBC_CYAN));
                    self.artists.draw(1, 2 * menu_w + 2, menu_h, menu_w);
                },
                State::Album => {
                    ncurses::mvaddnstr(0, 2 * menu_w + 2, "Album", menu_w);
                    ncurses::attron(ncurses::COLOR_PAIR(SBC_CYAN));
                    self.albums.draw(1, 2 * menu_w + 2, menu_h, menu_w);
                },
                State::Track => {
                    ncurses::mvaddnstr(0, 2 * menu_w + 2, "Track", menu_w);
                    ncurses::attron(ncurses::COLOR_PAIR(SBC_CYAN));
                    self.tracks.draw(1, 2 * menu_w + 2, menu_h, menu_w);
                }
            },
            None => (),
        }

        ncurses::attroff(ncurses::COLOR_PAIR(SBC_CYAN));
    }
}
