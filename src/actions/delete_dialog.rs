use crate::{dialogs::*, states::*};
use dioxus::prelude::*;

pub fn delete(left_panel: Option<bool>) {
    let mut main_state = consume_context::<Signal<MainState>>();

    let (panel_is_left, marked_amount) = {
        let main_state_read_access = main_state.read();

        let panel_state = main_state_read_access.get_panel_state(left_panel);

        let selected_amount = panel_state.statistics.marked_amount;

        (panel_state.left_panel, selected_amount)
    };

    println!("Amount: {}", marked_amount);

    let mut write_access = main_state.write();

    let (volume_and_path, selected_item) = {
        let panel_state = write_access.get_panel_state(left_panel);
        let volume_and_path = panel_state.volume_and_path.clone();

        if marked_amount > 1 {
            (volume_and_path, SelectedItem::MultiSelect(marked_amount))
        } else {
            let selected_item = panel_state.get_selected_or_marked_single_item();
            if selected_item.is_none() {
                return;
            }

            let selected_item = selected_item.unwrap();

            if selected_item.tp.is_back() {
                write_access.set_focus_to_active_panel();
                return;
            }

            (volume_and_path, SelectedItem::Single(selected_item.clone()))
        }
    };

    let dialog = DialogState::DeleteConfirmation {
        volume_and_path,
        selected_item,
        on_ok: EventHandler::new(move |_| {
            consume_context::<Signal<MainState>>()
                .write()
                .get_panel_state_mut(panel_is_left)
                .refresh_files(AutoSelectElement::None);
        }),
    };

    write_access.show_dialog(dialog);
}
