use crate::{dialogs::*, states::*};
use dioxus::prelude::*;

pub fn delete(left_panel: Option<bool>) {
    let mut main_state = consume_context::<Signal<MainState>>();

    let (panel_is_left, marked_items) = {
        let main_state_read_access = main_state.read();
        let panel_state = main_state_read_access.get_panel_state(left_panel);
        (panel_state.left_panel, panel_state.get_marked_items())
    };
    if marked_items.len() == 0 {
        return;
    }

    if marked_items.len() == 1 {
        if let Some(item) = marked_items.get(0) {
            if item.tp.is_back() {
                return;
            }
        }
    }

    let mut write_access = main_state.write();

    let volume_and_path = {
        let panel_state = write_access.get_panel_state(left_panel);
        let volume_and_path = panel_state.volume_and_path.clone();

        volume_and_path
    };

    let dialog = DialogState::DeleteConfirmation {
        volume_and_path,
        items: marked_items.into(),
        on_ok: EventHandler::new(move |_| {
            consume_context::<Signal<MainState>>()
                .write()
                .get_panel_state_mut(panel_is_left)
                .refresh_files(AutoSelectElement::None);
        }),
    };

    write_access.show_dialog(dialog);
}
