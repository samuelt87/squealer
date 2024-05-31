use crate::database::execute_query;
use crate::AppState;
use crossterm::event::{self, Event, KeyCode};
use std::error::Error;

pub async fn handle_event(app: &mut AppState<'static>) -> Result<bool, Box<dyn Error>> {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Tab => {
                    execute_query(app).await?;
                }
                KeyCode::Esc => return Ok(true),
                _ => {
                    app.query_input.input(key);
                }
            }
        }
    Ok(false)
}
