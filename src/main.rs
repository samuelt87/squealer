use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use rusqlite::{Connection,Result};
use std::{io, thread, time::Duration};

fn main() -> Result<(), std::io::Error> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let in_memory_db = Connection::open_in_memory().unwrap();
    in_memory_db.execute_batch(
        "
        CREATE TABLE users (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            email TEXT NOT NULL UNIQUE
        );
        INSERT INTO users (name, email) VALUES ('Alice', 'temp@email.com')
        ",
    ).unwrap();

    terminal.draw(|f| {
        let size = f.size();
        let query = in_memory_db.query_row(
            "SELECT name, email FROM users WHERE id = 1",
            [],
            |row| {
                let name: String = row.get(0)?;
                let email: String = row.get(1)?;
                Ok((name, email))
            },
        ).unwrap();
        let para = Paragraph::new(format!("Name: {}\nEmail: {}", query.0, query.1))
            .block(Block::default().title("Block").borders(Borders::ALL));
        f.render_widget(para, size);
    })?;

    // Start a thread to discard any input events. Without handling events, the
    // stdin buffer will fill up, and be read into the shell when the program exits.
    thread::spawn(|| loop {
        let _ = event::read();
    });

    thread::sleep(Duration::from_millis(5000));

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
