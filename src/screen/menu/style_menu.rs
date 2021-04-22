extern crate ncurses as n;

use super::Item;

use crate::Style;
use crate::StyleTree;

pub struct StyleMenu {
    items: Vec<Item>,
    styles: Vec<Option<Style>>,
    sel: usize,           // Currently selected item
}

impl StyleMenu {
    pub fn new() -> StyleMenu {
        StyleMenu {
            items: Vec::new(),
            styles: Vec::new(),
            sel: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        (self.styles.is_empty() || (self.styles.len() == 1 && self.styles[0] == None))
            && (self.items.is_empty() || (self.items.len() == 1 && self.items[0] == Item::All))
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

    pub fn style_selection(&self) -> Vec<Style> {
        if *self.sel() == Item::All {
            self.styles.iter()
                .filter(|s| if let Some(_) = s {
                    true
                } else {
                    false
                }).map(|s| match s {
                    Some(s) => *s,
                    None => 0,
                }).collect()
        } else {
            vec![self.styles[self.sel].unwrap()]
        }
    }

    pub fn set_styles(&mut self, styles: Vec<Style>, tree: &StyleTree) -> bool {
        let mut items: Vec<Item> = styles.iter()
            .map(|s| Item::from(tree.name(*s))).collect();
        let mut styles: Vec<Option<Style>> = styles.iter()
            .map(|s| Some(*s)).collect();

        items.insert(0, Item::All);
        styles.insert(0, None);

        if !self.items_same(&items) {
            self.styles = styles;
            self.items = items;
            self.sel = 0;

            return true
        }

        false
    }

    fn items_same(&self, b: &Vec<Item>) -> bool {
        let mut ret = true;

        if self.items.len() != b.len() {
            ret = false;
        } else {
            for i in 0..b.len() {
                if self.items[i] != b[i] {
                    ret = false;
                }
            }
        }

        ret
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
        if h % 2 == 0 {
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
