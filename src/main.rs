use clap::{load_yaml, App};
use directories::ProjectDirs;
use rusqlite::{Connection, Result as RSQLResult, NO_PARAMS};
use std::fs::{create_dir_all, remove_dir_all};

struct Book {
    id: String,
    name: String,
    link: Option<String>,
}

impl Book {
    fn new(name: &str, link: Option<&str>) -> Book {
        Book {
            id: uuid::Uuid::new_v4().to_hyphenated().to_string(),
            name: name.to_string(),
            link: Some(link.unwrap().to_string()),
        }
    }

    fn save(&self) {
        let conn = get_db_connection();

        conn.execute(
            "insert into books (id, name, link) values (?1, ?2, ?3)",
            &[
                self.id.clone(),
                self.name.clone(),
                self.link.as_ref().unwrap().to_string(),
            ],
        )
        .unwrap();
    }
}

struct Bookmark {
    id: String,
    book_id: String,
    page: Option<u32>,
    section: Option<String>,
    link: Option<String>,
}

struct Notes {
    id: String,
    book_id: String,
    note: String,
}

fn get_db_connection() -> Connection {
    let proj_dirs = ProjectDirs::from("dev", "Andrew Halle", "Book Tracker").unwrap();
    Connection::open(proj_dirs.data_dir().join("books.db")).unwrap()
}

fn init() -> RSQLResult<()> {
    let proj_dirs = ProjectDirs::from("dev", "Andrew Halle", "Book Tracker").unwrap();
    create_dir_all(proj_dirs.data_dir()).unwrap();

    let conn = get_db_connection();

    conn.execute(
        "CREATE TABLE books (
           id TEXT PRIMARY KEY,
           name TEXT NOT NULL,
           link TEXT
         );",
        NO_PARAMS,
    )?;

    conn.execute(
        "CREATE TABLE bookmarks (
           id TEXT PRIMARY KEY,
           book_id TEXT,
           page INTEGER,
           section TEXT,
           link TEXT,
           FOREIGN KEY (book_id) REFERENCES books (id)
         );",
        NO_PARAMS,
    )?;

    conn.execute(
        "CREATE TABLE notes (
           id TEXT PRIMARY KEY,
           book_id TEXT,
           note TEXT,
           FOREIGN KEY (book_id) REFERENCES books (id)
         );",
        NO_PARAMS,
    )?;

    Ok(())
}

fn teardown() {
    let proj_dirs = ProjectDirs::from("dev", "Andrew Halle", "Book Tracker").unwrap();
    remove_dir_all(proj_dirs.data_dir()).unwrap();
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    if let Some(matches) = matches.subcommand_matches("init") {
        init().unwrap();
    } else if let Some(matches) = matches.subcommand_matches("teardown") {
        teardown();
    }
}
