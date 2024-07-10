use crate::message::Message;
use crate::MainEvent;
use crossterm::event::{Event, KeyCode};
use ratatui::Frame;

enum Component {
    None,
    Connections,
    Results,
    Queries,
}

impl Default for Component {
    fn default() -> Self {
        Component::None
    }
}

// Modes

#[derive(Default)]
pub struct Home;

#[derive(Default)]
pub struct EditQuery;

#[derive(Default)]
pub struct BrowseSqliteDBFiles;

#[derive(Default)]
pub struct ExploreResults;

#[derive(Default)]
pub struct SaveResults;

#[derive(Default)]
pub struct ExploreConnection;

#[derive(Default)]
pub struct ConfigEditor;

#[derive(Default)]
pub struct Quit;

pub struct ViewState<Mode> {
    mode: Mode,
    selected: Component,
}

pub type ViewStateBox = Box<dyn ViewStateTrait>;

impl<Mode: Default> Default for ViewState<Mode> {
    fn default() -> Self {
        Self {
            mode: Mode::default(),
            selected: Component::default(),
        }
    }
}

struct ViewStateBuilder<Mode> {
    mode: Option<Mode>,
    selected: Option<Component>,
}

impl<Mode: Default> ViewStateBuilder<Mode> {
    fn default() -> Self {
        Self {
            mode: None,
            selected: None,
        }
    }

    fn mode(mut self, mode: Mode) -> Self {
        self.mode = Some(mode);
        self
    }

    fn selected(mut self, selected: Component) -> Self {
        self.selected = Some(selected);
        self
    }

    fn build(self) -> ViewState<Mode> {
        ViewState {
            mode: self.mode.unwrap_or_default(),
            selected: self.selected.unwrap_or_default(),
        }
    }
}

impl ViewState<Home> {
    pub fn new() -> Self {
        Self::default()
    }
}

pub trait ViewStateTrait {
    fn handle_event(&self, event: MainEvent) -> Message {
        match event {
            MainEvent::Input(event) => self.handle_input(event),
            MainEvent::Tick => Message::NoOp,
        }
    }
    fn handle_input(&self, event: Event) -> Message;
    fn update(self: Box<Self>, message: Message) -> (ViewStateBox, Option<Message>);
    fn should_quit(&self) -> bool {
        false
    }
    fn render(&self, frame: &mut Frame);
}

impl ViewStateTrait for ViewState<Home> {
    fn handle_input(&self, event: Event) -> Message {
        match event {
            Event::Key(key) => match key.code {
                KeyCode::Char('q') => Message::Quit,
                KeyCode::Enter => Message::SelectMode,
                _ => Message::NoOp,
            },
            _ => Message::NoOp,
        }
    }

    fn update(self: Box<Self>, message: Message) -> (ViewStateBox, Option<Message>) {
        match message {
            Message::Quit => (
                Box::new(ViewStateBuilder::default().mode(Quit).build()),
                None,
            ),
            Message::SelectMode => match self.selected {
                Component::None => (self, None),
                Component::Connections => (
                    Box::new(ViewStateBuilder::default().mode(ExploreConnection).build()),
                    None,
                ),
                Component::Results => (
                    Box::new(ViewStateBuilder::default().mode(ExploreResults).build()),
                    None,
                ),
                Component::Queries => (
                    Box::new(ViewStateBuilder::default().mode(EditQuery).build()),
                    None,
                ),
            },
            Message::NoOp | Message::Escape => (self, None),
        }
    }

    fn render(&self, frame: &mut Frame) {
        // render home view
    }
}

impl ViewStateTrait for ViewState<Quit> {
    fn handle_input(&self, _event: Event) -> Message {
        Message::Quit
    }

    fn update(self: Box<Self>, _message: Message) -> (ViewStateBox, Option<Message>) {
        (self, Some(Message::Quit))
    }

    fn should_quit(&self) -> bool {
        true
    }

    fn render(&self, frame: &mut Frame) {
        // render quit view
    }
}

impl ViewStateTrait for ViewState<ExploreConnection> {
    fn handle_input(&self, event: Event) -> Message {
        match event {
            Event::Key(key) => match key.code {
                KeyCode::Char('q') | KeyCode::Esc => Message::Escape,
                _ => Message::NoOp,
            },
            _ => Message::NoOp,
        }
    }

    fn update(self: Box<Self>, message: Message) -> (ViewStateBox, Option<Message>) {
        match message {
            Message::Quit => (
                Box::new(ViewStateBuilder::default().mode(Quit).build()),
                None,
            ),
            Message::NoOp | Message::SelectMode => (self, None),
            Message::Escape => (
                Box::new(ViewStateBuilder::default().mode(Home).build()),
                None,
            ),
        }
    }

    fn render(&self, frame: &mut Frame) {
        // render explore connection view
    }
}

impl ViewStateTrait for ViewState<ExploreResults> {
    fn handle_input(&self, event: Event) -> Message {
        match event {
            Event::Key(key) => match key.code {
                KeyCode::Char('q') | KeyCode::Esc => Message::Escape,
                _ => Message::NoOp,
            },
            _ => Message::NoOp,
        }
    }

    fn update(self: Box<Self>, message: Message) -> (ViewStateBox, Option<Message>) {
        match message {
            Message::Quit => (
                Box::new(ViewStateBuilder::default().mode(Quit).build()),
                None,
            ),
            Message::NoOp | Message::SelectMode => (self, None),
            Message::Escape => (
                Box::new(ViewStateBuilder::default().mode(Home).build()),
                None,
            ),
        }
    }

    fn render(&self, frame: &mut Frame) {
        // render explore results view
    }
}

impl ViewStateTrait for ViewState<EditQuery> {
    fn handle_input(&self, event: Event) -> Message {
        match event {
            Event::Key(key) => match key.code {
                KeyCode::Char('q') | KeyCode::Esc => Message::Escape,
                _ => Message::NoOp,
            },
            _ => Message::NoOp,
        }
    }

    fn update(self: Box<Self>, message: Message) -> (ViewStateBox, Option<Message>) {
        match message {
            Message::Quit => (
                Box::new(ViewStateBuilder::default().mode(Quit).build()),
                None,
            ),
            Message::NoOp | Message::SelectMode => (self, None),
            Message::Escape => (
                Box::new(ViewStateBuilder::default().mode(Home).build()),
                None,
            ),
        }
    }

    fn render(&self, frame: &mut Frame) {
        // render edit query view
    }
}
