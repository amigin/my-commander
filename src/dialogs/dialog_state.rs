use dioxus::prelude::*;

#[derive(Clone, Debug)]
pub enum DialogState {
    ViewFile(String),
    DeleteConfirmation {
        amount: usize,
        on_ok: EventHandler<()>,
    },
}
