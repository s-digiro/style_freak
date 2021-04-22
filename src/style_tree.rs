use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

pub type Style = usize;

// Base of tree is at index 0
pub struct StyleTree {
    names: Vec<String>,
    parents: Vec<Option<Style>>,
}

impl StyleTree {
    fn new() -> StyleTree {
        StyleTree {
            names: vec!["Root".to_string()],
            parents: vec![None],
        }
    }

    pub fn base(&self) -> Style {
        0
    }

    pub fn name(&self, style: Style) -> &str {
        &self.names[style]
    }

    pub fn children(&self, style: Style) -> Vec<Style> {
        self.parents.iter().enumerate()
            .filter(|(_, p)| **p == Some(style))
            .map(|(i, _)| i)
            .collect()
    }

    pub fn load_from_file(path: &str) -> Result<StyleTree, io::Error> {
        let mut tree = StyleTree::new();

        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let mut stack: Vec<Style> = Vec::new();

        for (_index, line) in reader.lines().enumerate() {
            let line = line?;
            let mut name_start = 0;
            let mut tabs = 0;

            // Count tabs and find start of word
            for (i, c) in line.chars().enumerate() {
                if c == '\t' {
                    tabs += 1;
                } else {
                    name_start = i;
                    break;
                }
            }

            let name = &line[name_start..];

            // remove from stack until we are at parent in path
            while stack.len() > tabs {
                stack.pop();
            }

            let parent = match stack.last() {
                Some(parent) => *parent,
                None => tree.base(),
            };

            let new_style = tree.names.len();
            tree.names.push(name.to_string());
            tree.parents.push(Some(parent));

            stack.push(new_style);
        }

        Ok(tree)
    }
}
