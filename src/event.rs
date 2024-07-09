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
}
