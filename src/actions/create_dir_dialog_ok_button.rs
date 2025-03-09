use dioxus::prelude::*;

use crate::{
    dialogs::DialogState,
    states::{AutoSelectElement, MainState},
};
pub fn create_dir_dialog_ok_button(current_path: &str, dir_name: &str) {
    let mut dir_to_create = current_path.to_string();
    if !dir_to_create.ends_with(std::path::MAIN_SEPARATOR) {
        dir_to_create.push(std::path::MAIN_SEPARATOR);
    }

    dir_to_create.push_str(dir_name);

    let dir_name = dir_name.to_string();
    spawn(async move {
        let result = tokio::fs::create_dir(dir_to_create).await;
        let mut main_state = consume_context::<Signal<MainState>>();
        let mut main_state_write_access = main_state.write();
        match result {
            Ok(_) => {
                main_state_write_access
                    .get_active_panel_mut()
                    .refresh_files(AutoSelectElement::Dir(dir_name));

                main_state_write_access.hide_dialog();
            }
            Err(err) => {
                println!("Err: {}", err);
                main_state_write_access.show_dialog(DialogState::ErrorDialog {
                    title: "Can not create directory",
                    msg: format!("{}", err),
                });
            }
        }
    });
}
