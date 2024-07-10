use crate::database::connect_to_database_file;
use crate::message::Message;
use crate::model;
use crate::model::{
    BrowseSqliteDBFiles, ConfigEditor, EditQuery, ExploreConnection, ExploreResults, Home,
    SaveResults,
};
use crate::App;
use crate::{app::UiState, database::execute_query};
use crossterm::event::{self, Event, KeyCode};
use std::error::Error;

//pub async fn handle_event(app: &mut App<'static>) -> Result<bool, Box<dyn Error>> {
//    let event = event::read()?;
//    match app.ui_state {
//        UiState::Query => handle_query_event(app, event).await,
//        UiState::Explorer => handle_explorer_event(app, event).await,
//    }
//}

struct EventHandler<Mode> {
    mode: Mode,
}

trait EventHandlerTrait<Mode> {
    fn handle_event(app: &mut App<'static>, model_app: model::App<Mode>) -> Message;
}

impl EventHandlerTrait<Home> for EventHandler<Home> {
    fn handle_event(app: &mut App<'static>, model_app: model::App<Home>) -> Message {
        let event = event::read();
        match event {}
    }
}

async fn handle_query_event(app: &mut App<'static>, event: Event) -> Result<bool, Box<dyn Error>> {
    match event {
        Event::Key(key) => match key.code {
            KeyCode::Tab => {
                execute_query(app).await?;
            }
            KeyCode::Esc => return Ok(true),
            _ => {
                app.query_input.input(key);
            }
        },
        _ => {}
    }
    Ok(false)
}

async fn handle_explorer_event(
    app: &mut App<'static>,
    event: Event,
) -> Result<bool, Box<dyn Error>> {
    match event {
        Event::Key(key) => match key.code {
            KeyCode::Esc => {
                app.ui_state = UiState::Query;
            }
            _ => match app.explorer {
                Some(ref mut explorer) => match key.code {
                    KeyCode::Enter => {
                        let file = explorer.current();
                        app.pool =
                            connect_to_database_file(&file.path().display().to_string().as_str())
                                .await;
                        execute_query(app).await?;
                        app.ui_state = UiState::Query;
                    }
                    _ => explorer.handle(&event)?,
                },
                None => {}
            },
        },
        _ => {}
    }
    Ok(false)
}
