use crate::{dialogs::*, states::*};
use dioxus::prelude::*;

#[derive(Clone, Copy)]
pub struct RenderData {
    pub selected_file_type: Option<FileLineType>,
    pub selected_file_index: usize,
    pub total_items: usize,
    pub left_panel: bool,
    pub selected_amount: usize,
    pub dialog_shows: bool,
}

impl RenderData {
    pub fn new(left_panel: bool, selected_file_index: usize) -> Self {
        Self {
            selected_file_type: None,
            selected_file_index,
            total_items: 0,
            left_panel,
            selected_amount: 0,
            dialog_shows: false,
        }
    }
}

pub fn handle_key_press(
    event: Event<KeyboardData>,
    mut main_state: Signal<MainState>,
    render_data: RenderData,
) {
    if render_data.dialog_shows {
        return;
    }
    match event.key() {
        Key::Tab => {
            main_state.write().tab_pressed();
        }
        Key::Enter => {
            if let Some(selected_file_type) = render_data.selected_file_type {
                match selected_file_type {
                    FileLineType::Dir => {
                        main_state.write().press_enter(render_data.left_panel);
                    }
                    FileLineType::Back => {
                        main_state.write().press_enter(render_data.left_panel);
                    }
                    FileLineType::File => {}
                }
            }
        }
        Key::ArrowLeft => {
            main_state
                .write()
                .get_panel_state_mut(render_data.left_panel)
                .selected_file_index = 0;
        }
        Key::ArrowRight => {
            main_state
                .write()
                .get_panel_state_mut(render_data.left_panel)
                .select_last_file();
        }
        Key::ArrowDown => {
            if render_data.selected_file_index < render_data.total_items - 1 {
                main_state
                    .write()
                    .get_panel_state_mut(render_data.left_panel)
                    .set_selected_file(render_data.selected_file_index + 1);
            }
        }
        Key::ArrowUp => {
            if render_data.selected_file_index > 0 {
                main_state
                    .write()
                    .get_panel_state_mut(render_data.left_panel)
                    .set_selected_file(render_data.selected_file_index - 1);
            }
        }
        Key::F3 => {
            let file_path = {
                let write_access = main_state.read();
                let panel_state = write_access.get_panel_state(render_data.left_panel);
                let item = panel_state.get_selected_file();
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
                main_state.write().dialog = Some(DialogState::ViewFile(file_path));
            }
        }
        Key::F8 => {
            if let Some(selected_file_type) = render_data.selected_file_type {
                if selected_file_type.is_back() {
                    return;
                }
                let amount = if render_data.selected_amount == 0 {
                    1
                } else {
                    render_data.selected_amount
                };
                main_state.write().dialog = Some(DialogState::DeleteConfirmation {
                    amount,
                    on_ok: EventHandler::new(move |_| {}),
                });
            }
        }
        Key::Escape => {
            if main_state.read().dialog.is_some() {
                main_state.write().dialog = None;
            }
        }
        _ => match event.code() {
            Code::Space => {
                main_state
                    .write()
                    .get_panel_state_mut(render_data.left_panel)
                    .space_pressed(render_data.selected_file_index);
            }
            _ => {}
        },
    }
}
