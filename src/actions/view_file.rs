use dioxus::prelude::*;

use crate::{dialogs::DialogState, MainState};

pub fn view_file(left_panel: Option<bool>) {
    let mut main_state = consume_context::<Signal<MainState>>();
    let file_path = {
        let write_access = main_state.write();
        let panel_state = write_access.get_panel_state(left_panel);
        let item = panel_state.get_selected_item();
        if item.tp.is_file() {
            Some(
                panel_state
                    .volume_and_path
                    .new_with_segment(item.name.as_str())
                    .into_string(),
            )
        } else {
            None
        }
    };
    if let Some(file_path) = file_path {
        main_state
            .write()
            .show_dialog(DialogState::ViewFile(file_path));
    }
}
