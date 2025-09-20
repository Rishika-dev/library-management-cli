mod book;
mod storage;
mod utils;

use crate::book::{Book, BookStatus};
use crate::storage::{add_book, load_books, save_books, display_book};
use crate::utils::read_line;
use std::io;

fn main() -> io::Result<()> {
    loop {
        println!("***** Welcome to BOOK CLI *****");
        println!("1. Add Book");
        println!("2. Display Book");
        println!("3. Exit");

        let choice = read_line("Enter your choice: ")?;
        match choice.trim() {
            "1" => add_book()?,
            "2" => {
                let books = load_books()?;
                display_book(&books)?;
            }
            "3" => break,
            _ => println!("Invalid choice!"),
        }
    }

    Ok(())
}
