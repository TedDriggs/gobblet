use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Size {
    Small = 1,
    Medium = 2,
    Large = 3,
}

impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let full_name = match self {
            Size::Small => "Small",
            Size::Medium => "Medium",
            Size::Large => "Large",
        };

        write!(
            f,
            "{}",
            if f.alternate() {
                &full_name[0..1]
            } else {
                full_name
            }
        )
    }
}
