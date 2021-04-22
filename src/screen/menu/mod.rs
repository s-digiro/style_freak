extern crate ncurses as n;

pub use item::Item;
pub use style_menu::StyleMenu;

mod item;
mod style_menu;

pub struct Menu {
    items: Vec<Item>,
    sel: usize,           // Currently selected item
}

impl Menu {
    pub fn new() -> Menu {
        Menu {
            items: Vec::new(),
            sel: 0,
        }
    }

    pub fn search(&mut self, target: &str) {
        if target != "" {
            let sel = self.items.iter()
                .position(|item| item.val().to_ascii_uppercase()
                          .contains(&String::from(target).to_ascii_uppercase()))
                .unwrap_or(self.sel);
            self.sel = sel;
        }
    }

    pub fn sel(&self) -> &Item {
        &self.items[self.sel]
    }

    pub fn set_items(&mut self, items: Vec<Item>) {
        self.items = items;
        self.sel = 0;
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn i(&self) -> usize {
        self.sel
    }

    pub fn next(&mut self) {
        if self.items.len() == 0 {
            return
        }

        self.sel += 1;
        if self.sel >= self.items.len() {
            self.sel = 0;
        }
    }

    pub fn prev(&mut self) {
        if self.items.len() == 0 {
            return
        }

        if self.sel <= 0 {
            self.sel = self.items.len() - 1;
        } else {
            self.sel -= 1;
        }
    }

    pub fn draw(&self, y: i32, x: i32, h: i32, w: i32) {
        let mut line = 1;
        let max_line = h - 1;

        let mut center = h / 2;
        if h & 1 == 0 { // check if number is even. Faster than h % 2
            center = center - 1;
        }

        let first_visible = if self.sel as i32 <= center {
            0
        } else if self.sel as i32 >= self.items.len() as i32 - center {
            std::cmp::max(
                0,
                self.items.len() as i32 - h
            ) as usize
        } else {
            (self.sel as i32 - center) as usize
        };

        for (i, item) in self.items.iter().enumerate().skip(first_visible) {
            if self.sel == i {
                n::attron(n::A_REVERSE());
                let bg = String::from_utf8(vec![b' '; w as usize]).unwrap();
                n::mvaddnstr(y + line, x, &bg, w);
                n::mvaddnstr(y + line, x, &item.to_string(), w);
                n::attroff(n::A_REVERSE());
            } else {
                n::mvaddnstr(y + line, x, &item.to_string(), w);
            }

            line += 1;

            if line > max_line {
                break;
            }
        }
    }
}
