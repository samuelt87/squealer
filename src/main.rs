mod app;
mod config;
mod database;
// mod event_handler;
mod message;
mod model;
mod terminal;
mod ui;
mod viewstate;

use app::App;
use crossterm::event::EventStream;
use crossterm::event::{self, Event};
use database::initial_database_conection;
use futures::StreamExt;
use message::Message;
use model::Home;
use std::error::Error;
use terminal::{restore_terminal, setup_terminal};
use tokio;
use tokio::sync::{broadcast, mpsc};
use tokio::task;
use viewstate::{ViewStateBox, ViewStateTrait};

enum MainEvent {
    Input(crossterm::event::Event),
    Tick,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (tx, mut rx) = mpsc::unbounded_channel();
    let (shutdown_tx, _) = broadcast::channel(1);

    let config = config::Config::new();
    let mut model_app = model::App::<Home>::new(config);
    let mut app = App::new(
        "SELECT id, name, email FROM users",
        initial_database_conection,
    )
    .await;

    let tx_crossterm = tx.clone();
    let mut shutdown_rx = shutdown_tx.subscribe();
    task::spawn(async move {
        let mut event_stream = EventStream::new();
        loop {
            tokio::select! {
                _ = shutdown_rx.recv() => break,
                Some(Ok(event)) = event_stream.next() => {
                    let main_event = MainEvent::Input(event);
                    tx_crossterm.send(main_event).unwrap();
                }
            }
        }
    });

    let tx_tick = tx.clone();
    let mut tick_shutdown_rx = shutdown_tx.subscribe();
    task::spawn(async move {
        loop {
            tokio::select! {
                _ = tick_shutdown_rx.recv() => break,
                _ = tokio::time::sleep(tokio::time::Duration::from_millis(100)) => {
                    let event = MainEvent::Tick;
                    tx_tick.send(event).unwrap();
                }
            }
        }
    });

    let mut viewstate: ViewStateBox = Box::new(viewstate::ViewState::new());

    let mut terminal = setup_terminal()?;

    fn handle_message(message: Message, viewstate: ViewStateBox) -> ViewStateBox {
        let (new_viewstate, new_message) = viewstate.update(message);
        match new_message {
            Some(message) => handle_message(message, new_viewstate),
            None => new_viewstate,
        }
    }

    while let Some(event) = rx.recv().await {
        let message = match event {
            MainEvent::Input(event) => viewstate.handle_input(event).clone(),
            MainEvent::Tick => viewstate.handle_event(event).clone(),
        };
        viewstate = handle_message(message, viewstate);
        if viewstate.should_quit() {
            break;
        }
        terminal.draw(|f| {
            viewstate.render(f);
        })?;
    }

    shutdown_tx.send(true).unwrap();
    drop(tx);
    restore_terminal(&mut terminal)
}
