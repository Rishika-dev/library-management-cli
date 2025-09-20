use crate::book::{Book, BookStatus};
use crate::utils::read_line;
use prettytable::{Table, row};
use std::fs::OpenOptions;
use std::io::{self, Read, Write};

const FILE_NAME: &str = "Books.json";

pub fn load_books() -> io::Result<Vec<Book>> {
    let mut books = Vec::new();
    if let Ok(mut file) = OpenOptions::new().read(true).open(FILE_NAME) {
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        if !content.trim().is_empty() {
            books = serde_json::from_str(&content).unwrap_or(Vec::new());
        }
    }
    Ok(books)
}

pub fn save_books(books: &Vec<Book>) -> io::Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(FILE_NAME)?;
    serde_json::to_writer_pretty(&file, &books).expect("failed to write JSON");
    Ok(())
}

pub fn add_book() -> io::Result<()> {
    let id = read_line("Add the book id:")?;
    let title = read_line("Add the book title:")?;
    let author = read_line("Add author name:")?;
    let year = read_line("Add year:")?;

    let mut books = load_books()?;
    if books
        .iter()
        .any(|b| b.id == id.trim().parse::<i16>().unwrap_or(0))
    {
        println!("Book id already exists!");
        return Ok(());
    }
    let create_book = Book {
        id: id.trim().parse::<i16>().unwrap_or(0),
        title,
        author,
        year: year.trim().parse::<i16>().unwrap_or(0),
        status: BookStatus::Available,
    };

    books.push(create_book);
    save_books(&books)?;
    println!("Book added successfully!");
    Ok(())
}

pub fn display_book(books: &Vec<Book>) -> io::Result<()> {
    let id = read_line("Enter the book id:")?;
    let id = id.trim().parse::<i16>().unwrap_or(0);

    let mut table = Table::new();
    table.add_row(row!["ID", "Title", "Author", "Year", "Status"]);

    match books.iter().find(|b| b.id == id) {
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
            table.printstd();
        }
        None => println!("Book Not Found!"),
    }

    Ok(())
}
