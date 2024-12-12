use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Example {
    string: String,
    file_path: PathBuf,
    isize: isize,
    usize: usize,
    bool: bool,
    f32: f32,
    vec: Vec<String>,
    hash_map: Option<HashMap<String, usize>>,
    option: Option<usize>,
    tuple: Option<(String, usize)>,
    enumerable: Enumerable,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum Enumerable {
    Zero,
    One,
    Two,
}

impl Example {
    #[allow(clippy::self_named_constructors)]
    pub fn example() -> Example {
        Example {
            string: "Hello, world!".to_owned(),
            file_path: PathBuf::from("/path/to/file"),
            isize: -12345,
            usize: 12345,
            f32: -12_345.679,
            bool: true,
            vec: vec!["One".to_owned(), "Two".to_owned(), "Three".to_owned()],
            hash_map: Some(HashMap::from([
                ("one".to_owned(), 1),
                ("two".to_owned(), 2),
                ("three".to_owned(), 3),
            ])),
            option: None,
            tuple: Some(("one".to_owned(), 1)),
            enumerable: Enumerable::One,
        }
    }
    pub fn flat() -> Example {
        Example {
            string: "Hello, world!".to_owned(),
            file_path: PathBuf::from("/path/to/file"),
            isize: -12345,
            usize: 12345,
            f32: -12_345.679,
            bool: true,
            vec: vec!["One".to_owned(), "Two".to_owned(), "Three".to_owned()],
            hash_map: None,
            option: None,
            tuple: None,
            enumerable: Enumerable::One,
        }
    }
}
