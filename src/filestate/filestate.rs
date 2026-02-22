use crate::board::{self, Player};
use file_type::FileType;

use std::{error::Error, fs::File, io::Read};

#[allow(dead_code)]
#[derive(strum_macros::Display, Debug)]
pub enum FileError {
    UnknownFormat,
    UnknownBoardStyle,
}

impl Error for FileError {}

#[allow(dead_code)]
pub fn get_cells(filename: &str) -> Result<Vec<Option<Player>>, Box<dyn Error>> {
    let mut file = File::open(filename)?;
    let mut buffer: Vec<u8> = vec![];

    file.read_to_end(&mut buffer)?;
    let file_type = FileType::from_bytes(&buffer);

    if file_type.media_types() == ["text/plain"] {
        return from_text(buffer);
    }

    Err(Box::from(FileError::UnknownFormat))
}

#[allow(dead_code)]
fn from_text(buffer: Vec<u8>) -> Result<Vec<Option<Player>>, Box<dyn Error>> {
    let contents = String::from_utf8(buffer)?.replace("\r\n", "\n");

    if contents.starts_with("<board>") {
        return from_text_board(
            contents
                .strip_prefix("<board>")
                .expect("string should start with <board>")
                .trim(),
        );
    }

    if contents.starts_with("<cells>") {
        return from_text_cells(
            contents
                .strip_prefix("<cells>")
                .expect("string should start with <cells>")
                .trim(),
        );
    }

    Err(Box::from(FileError::UnknownBoardStyle))
}

#[allow(dead_code)]
fn from_text_cells(cell_string: &str) -> Result<Vec<Option<Player>>, Box<dyn Error>> {
    Ok(board::from_cell_string_to_state(cell_string))
}

#[allow(dead_code)]
fn from_text_board(board: &str) -> Result<Vec<Option<Player>>, Box<dyn Error>> {
    Ok(board::from_board_string_to_state(board))
}

#[allow(dead_code)]
fn from_image(_buffer: Vec<u8>) -> Result<Vec<Option<Player>>, Box<dyn Error>> {
    Ok(vec![])
}
