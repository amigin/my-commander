use crate::{dialogs::*, states::*};
use dioxus::prelude::*;

pub fn delete(left_panel: Option<bool>) {
    let mut main_state = consume_context::<Signal<MainState>>();

    let panel_statistics = {
        let main_state_read_access = main_state.read();

        let panel_state = main_state_read_access.get_panel_state(left_panel);

        match panel_state.try_get_selected_item() {
            Some(selected_item) => {
                if selected_item.tp.is_back() {
                    main_state_read_access.set_focus_to_active_panel();
                    return;
                }
            }
            None => return,
        };

        panel_state.statistics.clone()
    };

    let amount = if panel_statistics.selected_amount == 0 {
        1
    } else {
        panel_statistics.selected_amount
    };

    let mut write_access = main_state.write();

    let (volume_and_path, selected_item) = {
        let panel_state = write_access.get_panel_state(left_panel);

        let volume_and_path = panel_state.volume_and_path.clone();
        let selected_item = panel_state.get_selected_item().clone();

        (volume_and_path, selected_item)
    };

    let dialog = DialogState::DeleteConfirmation {
        amount,
        volume_and_path,
        selected_item,
        on_ok: EventHandler::new(move |_| {
            consume_context::<Signal<MainState>>()
                .write()
                .get_panel_state_mut(panel_statistics.left_panel)
                .refresh_files();
        }),
    };

    write_access.show_dialog(dialog);
}
