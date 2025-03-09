use dioxus::prelude::*;

use crate::{dialogs::DialogState, states::*};

pub fn create_dir() {
    let mut main_state = consume_context::<Signal<MainState>>();
    let mut write_access = main_state.write();

    let panel = write_access
        .get_active_panel()
        .volume_and_path
        .as_str()
        .to_string();

    write_access.show_dialog(DialogState::CreateDir(panel.into()));
}
