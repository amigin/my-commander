use dioxus::prelude::*;

use super::*;
use crate::MainState;

#[component]
pub fn RenderDialog() -> Element {
    let main_state = consume_context::<Signal<MainState>>();

    let dialog_state = {
        let main_state_read_access = main_state.read();
        main_state_read_access.get_dialog().clone()
    };

    if dialog_state.is_none() {
        return rsx! {};
    }

    let dialog_state = dialog_state.unwrap();

    match dialog_state {
        super::DialogState::ViewFile(file_name) => {
            return rsx! {
                DialogViewFile { file_name }
            }
        }

        super::DialogState::DeleteConfirmation {
            volume_and_path,
            items,
            on_ok,
        } => {
            return rsx! {
                DeleteDialog { volume_and_path, items, on_ok }
            }
        }

        super::DialogState::CreateDir(current_path) => {
            return rsx! {
                CreateDirDialog { current_path }
            }
        }
        super::DialogState::ErrorDialog { title, msg } => rsx! {
            ErrorDialog { title, message: msg }
        },
    }
}
