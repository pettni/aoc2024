use std::fmt;
use std::path::PathBuf;

pub mod container;
pub mod hash;
pub mod heap;
pub mod map2d;
pub mod math;
pub mod parsing;
pub mod solutions;
pub mod trie;
pub mod vec2;

#[derive(Debug, PartialEq, Default)]
pub enum Answer {
    #[default]
    Unimplemented,
    Number(i64),
    String(&'static str),
}

impl fmt::Display for Answer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Answer::Number(n) => n.fmt(f),
            Answer::String(s) => s.fmt(f),
            Answer::Unimplemented => "Unimplemented".fmt(f),
        }
    }
}

pub type Solutions = (fn(&str) -> Answer, fn(&str) -> Answer);

pub fn get_default_data_path(day: u32) -> PathBuf {
    PathBuf::from(format!("data/{:02}.txt", day))
}
