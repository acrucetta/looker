use crate::data_ingestion;
use crate::data_ingestion::text_processing::process_text;
use data_ingestion::file_handler::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

use super::Index;

#[derive(Clone, Serialize, Deserialize, Eq, PartialEq, Debug, Hash, PartialOrd, Ord)]
pub struct Document {
    pub path: String,
    pub contents: String,
}

impl Document {
    pub fn new(path: String, contents: String) -> Document {
        Document { path, contents }
    }

    // To string method
    pub fn to_string(&self) -> String {
        format!("{}: {}", self.path, self.contents)
    }
}

impl std::fmt::Display for Document {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.path)
    }
}

pub fn process_directory<P: AsRef<Path>>(
    path: P,
    index: &mut Index,
) -> Result<(), Box<dyn std::error::Error>> {
    let path = path.as_ref();
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                process_file(&path, index)?;
            } else if path.is_dir() {
                process_directory(&path, index)?;
            }
        }
    } else {
        return Err(From::from("Input path must be a directory."));
    }

    Ok(())
}

pub fn process_file<P: AsRef<Path>>(
    path: P,
    index: &mut Index,
) -> Result<(), Box<dyn std::error::Error>> {
    let path = path.as_ref();
    let file_extension = path.extension().unwrap().to_str().unwrap();
    match file_extension {
        "md" => {
            let file_handler = data_ingestion::MarkdownHandler;
            let content = file_handler.read_contents(path.to_str().unwrap())?;
            let processed_text = process_text(&content);
            let document = Document::new(path.to_str().unwrap().to_owned(), content);
            index.store_processed_text_in_index(&document);

            Ok(())
        }
        "txt" => {
            let file_handler = data_ingestion::PlainTextHandler;
            let content = file_handler.read_contents(path.to_str().unwrap())?;
            let processed_text = process_text(&content);
            let document = Document::new(path.to_str().unwrap().to_owned(), content);
            index.store_processed_text_in_index(&document);

            Ok(())
        }
        _ => {
            return Err(From::from(format!(
                "File extension {} is not supported.",
                file_extension
            )));
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_process_file() {
        let file_path = "data/lorem_ipsum.txt";
        let mut index = super::Index::new();
        super::process_file(file_path, &mut index).unwrap();
        print!("{:?}", index.inverted_index);
        assert_eq!(index.inverted_index.len(), 74);
    }

    #[test]
    fn test_process_directory() {
        let dir_path = "data";
        let mut index = super::Index::new();
        super::process_directory(dir_path, &mut index).unwrap();
        print!("{:?}", index.inverted_index);
        assert_eq!(index.inverted_index.len(), 3);
    }
}
