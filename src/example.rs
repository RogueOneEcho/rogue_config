use crate::example::Enumerable::One;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Example {
    string: String,
    path: PathBuf,
    isize: isize,
    usize: usize,
    bool: bool,
    f32: f32,
    vec: Vec<String>,
    hash_map: HashMap<String, usize>,
    option: Option<usize>,
    tuple: (String, usize),
    enumerable: Enumerable,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum Enumerable {
    Zero,
    One,
    Two,
}

impl Example {
    pub fn example() -> Example {
        Example {
            string: "Hello, world!".to_owned(),
            path: PathBuf::from("/path/to/file"),
            isize: -12345,
            usize: 12345,
            f32: -12345.6789,
            bool: true,
            vec: vec!["One".to_owned(), "Two".to_owned(), "Three".to_owned()],
            hash_map: HashMap::from([
                ("one".to_owned(), 1),
                ("two".to_owned(), 2),
                ("three".to_owned(), 3),
            ]),
            option: Some(12345),
            tuple: ("one".to_owned(), 1),
            enumerable: One,
        }
    }
}
