use prettytable::{Table, row};
use serde::{Deserialize, Serialize};
use std::{
    fs::{ OpenOptions},
    io::{self, Read, Write},
};

#[derive(Serialize, Deserialize)]
enum BookStatus {
    Available,
    Borrowed,
}

#[derive(Serialize, Deserialize)]
struct Book {
    id: i16,
    title: String,
    author: String,
    year: i16,
    status: BookStatus,
}
fn main() -> io::Result<()> {
    // let mut books: Vec<Book> = Vec::new();
    add_book()?;
    display_book(&books)?;
    Ok(())
}

fn add_book() -> io::Result<()> {
    let id = read_line("Add the book id:")?;
    let title = read_line("Add the book title")?;
    let author = read_line("Add author name")?;
    let year = read_line("Add year")?;

    let create_book = Book {
        id: id.trim().parse::<i16>().unwrap_or(0),
        title,
        author,
        year: year.trim().parse::<i16>().unwrap_or(0),
        status: BookStatus::Available,
    };

    let mut existing_books = Vec::new();
    if let Ok(mut file) = OpenOptions::new().read(true).open("Books.json") {
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        if !content.trim().is_empty() {
            existing_books = serde_json::from_str(&content).unwrap_or(Vec::new());
        }
    }

    existing_books.push(create_book);

    // Write back the updated array
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("Books.json")?;
    serde_json::to_writer_pretty(&file, &existing_books).expect("failed to write JSON");

    Ok(())
}

fn display_book(books: &Vec<Book>) -> io::Result<()> {
    let id = read_line("Enter the book id:")?;
    let mut table = Table::new();
    table.add_row(row!["ID", "Title", "Author", "Year", "Status"]);

    match books
        .iter()
        .find(|b| b.id == id.trim().parse::<i16>().unwrap_or(0))
    {
        Some(book) => {
            table.add_row(row![
                book.id,
                book.title,
                book.author,
                book.year,
                match book.status {
                    BookStatus::Available => "Available",
                    BookStatus::Borrowed => "Borrowed",
                }
            ]);
        }
        None => println!("Book Not Found!"),
    }
    table.printstd();
    Ok(())
}

fn read_line(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}
