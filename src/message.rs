#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Message {
    NoOp,
    Quit,
    SelectMode,
    Escape,
}
