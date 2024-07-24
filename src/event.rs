use crate::item::Item;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Event {
    EvReaderNewItem,
    EvReaderFinished,
    EvMatcherNewItem,
    EvMatcherResetQuery,
    EvMatcherUpdateProcess,
    EvQueryChange,
    EvInputToggle,
    EvInputUp,
    EvInputDown,
    EvInputSelect,
    Stop,
    NewPassword,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FuzzyPassEvent {
    SourceEvent(SourceEvent),
    KeyboardEvent(KeyboardEvent),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SourceEvent {
    ReadFinished(Vec<Item>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyboardEvent {
    ItemSelected,
    QueryChanged(Vec<char>),
    Up,
    Down,
    Exit,
    NewPassword,
    UnknownEvent,
}
