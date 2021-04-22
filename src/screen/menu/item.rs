#[derive(Clone)]
#[derive(PartialEq)]
pub enum Item {
    Normal(String),
    Empty,
    All,
}

impl Item {
    pub fn norm(s: &str) -> Item {
        Item::Normal(s.to_string())
    }

    pub fn from(s: &str) -> Item {
        match s {
            "" => Item::Empty,
            s => Item::norm(s),
        }
    }

    pub fn val(&self) -> &str {
        match self {
            Item::Normal(s) => s,
            Item::Empty => "",
            Item::All => "",
        }
    }
}


impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Item::Normal(s) => write!(f, "{}", s),
            Item::Empty => write!(f, "<Empty>"),
            Item::All => write!(f, "<All>"),
        }
    }
}

impl std::fmt::Debug for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Item::Normal(s) => write!(f, "{}", s),
            Item::Empty => write!(f, "<Empty>"),
            Item::All => write!(f, "<All>"),
        }
    }
}
