use std::rc::Rc;

use dioxus::prelude::*;
use rust_extensions::file_utils::FilePath;

use crate::{states::PanelFileItem, MainState};

#[component]
pub fn DeletingContent(current_path: String, items: Rc<Vec<PanelFileItem>>) -> Element {
    let mut started = use_signal(|| false);

    let started_value = started.read().clone();

    if !started_value {
        use_effect(move || {
            started.set(true);
            deleting(current_path.clone(), items.clone());
        });
    }

    rsx! {
        div { class: "dialog-content" }

        div { class: "dialog-buttons",
            div { class: "dialog-btn-group",
                button {
                    id: "btn-cancel",
                    tabindex: "1",
                    class: "mac-gray-button",
                    onclick: |_| {
                        consume_context::<Signal<MainState>>().write().hide_dialog();
                    },
                    "Stop and Cancel"
                }
            }
        }
    }
}

fn deleting(current_path: String, items: Rc<Vec<PanelFileItem>>) {
    spawn(async move {
        for itm in items.iter() {
            match itm.tp {
                crate::states::FileLineType::Dir => {
                    let mut path = FilePath::from_str(&current_path);
                    path.append_segment(&itm.name);
                    let _ = tokio::fs::remove_dir_all(path.as_str()).await;
                }
                crate::states::FileLineType::File => {
                    let mut path = FilePath::from_str(&current_path);
                    path.append_segment(&itm.name);
                    let _ = tokio::fs::remove_file(path.as_str()).await;
                }
                crate::states::FileLineType::Back => {}
            }
        }

        let mut main_state = consume_context::<Signal<MainState>>();
        let mut write_access = main_state.write();
        write_access
            .get_active_panel_mut()
            .refresh_files(crate::states::AutoSelectElement::None);
        write_access.hide_dialog();
    });
}
