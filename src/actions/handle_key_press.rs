use crate::states::*;
use dioxus::prelude::*;

pub fn handle_key_press(
    mut main_state: Signal<MainState>,
    event: Event<KeyboardData>,
    panel_statistics: PanelFilesStatistics,
) {
    let (selected_file_type, selected_file_index) = {
        let read_access = main_state.read();
        let panel_state = read_access.get_panel_state(panel_statistics.left_panel.into());

        let selected_item = panel_state.try_get_selected_item();

        if selected_item.is_none() {
            return;
        }

        (selected_item.unwrap().tp, panel_state.selected_file_index)
    };

    match event.key() {
        Key::Tab => {
            main_state.write().tab_pressed();
        }
        Key::Enter => match selected_file_type {
            FileLineType::Dir => {
                consume_context::<Signal<MainState>>()
                    .write()
                    .press_enter(panel_statistics.left_panel.into());
            }
            FileLineType::Back => {
                consume_context::<Signal<MainState>>()
                    .write()
                    .press_enter(panel_statistics.left_panel.into());
            }
            FileLineType::File => {}
        },
        Key::ArrowLeft => {
            consume_context::<Signal<MainState>>()
                .write()
                .get_panel_state_mut(panel_statistics.left_panel.into())
                .selected_file_index = 0;
        }
        Key::ArrowRight => {
            main_state
                .write()
                .get_panel_state_mut(panel_statistics.left_panel.into())
                .select_last_file();
        }
        Key::ArrowDown => {
            if selected_file_index < panel_statistics.total_items - 1 {
                main_state
                    .write()
                    .get_panel_state_mut(panel_statistics.left_panel)
                    .set_selected_file(selected_file_index + 1);
            }
        }
        Key::ArrowUp => {
            if selected_file_index > 0 {
                main_state
                    .write()
                    .get_panel_state_mut(panel_statistics.left_panel)
                    .set_selected_file(selected_file_index - 1);
            }
        }
        Key::F3 => {
            super::view_file(panel_statistics.left_panel.into());
        }
        Key::F8 => {
            super::delete(panel_statistics.left_panel.into());
        }
        Key::Escape => {
            let mut main_state = consume_context::<Signal<MainState>>();
            if main_state.read().dialog_is_opened() {
                main_state.write().hide_dialog();
            }
        }
        _ => match event.code() {
            Code::Space => {
                let mut main_state = consume_context::<Signal<MainState>>();
                main_state
                    .write()
                    .get_panel_state_mut(panel_statistics.left_panel)
                    .space_pressed(selected_file_index);
            }
            _ => {}
        },
    }
}
