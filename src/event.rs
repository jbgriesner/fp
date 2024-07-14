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

pub enum FuzzyPassEvent {
    SourceEvent(SourceEvent),
    KeyboardEvent(KeyboardEvent),
}

pub enum SourceEvent {
    ReadFinished(Vec<Item>),
}

pub enum KeyboardEvent {
    ItemSelected,
    QueryChanged,
    Up,
    Down,
    Exit,
    NewPassword,
    UnknownEvent,
}
