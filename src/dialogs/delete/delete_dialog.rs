use std::rc::Rc;

use super::super::*;
use crate::{
    render_folder_icon, render_icon, states::PanelFileItem, volume_path_and_file::VolumePathAndFile,
};
use dioxus::prelude::*;

#[component]
pub fn DeleteDialog(
    volume_and_path: VolumePathAndFile,
    items: Rc<Vec<PanelFileItem>>,
    on_ok: EventHandler<()>,
) -> Element {
    let mut state = use_signal(|| StateMode::Confirmation);

    crate::utils::set_focus("btn-cancel");

    let state_value = state.read().clone();

    let content = match state_value {
        StateMode::Confirmation => {
            let phrase = if items.len() == 1 {
                let selected_item = items.get(0).unwrap();

                match selected_item.tp {
                    crate::FileLineType::Dir => {
                        let icon = render_folder_icon();
                        rsx! {
                            div {
                                "Are you sure you want to delete "
                                {icon}
                                span { style: "font-weight:800", {selected_item.name.as_str()} }
                                " directory?"
                            }
                        }
                    }
                    crate::FileLineType::File => {
                        let icon = render_icon(selected_item.name.as_str());
                        rsx! {
                            div {
                                "Are you sure you want to delete "
                                {icon}
                                span { style: "font-weight:800", {selected_item.name.as_str()} }
                                " file?"
                            }
                        }
                    }
                    crate::FileLineType::Back => {
                        panic!("Back should not be in the list");
                    }
                }
            } else {
                rsx! {
                    div {
                        "Are you sure you want to delete "
                        span { style: "font-weight:800", {items.len().to_string()} }
                        " items?"
                    }
                }
            };

            rsx! {
                ConfirmationContent {
                    phrase,
                    on_ok: EventHandler::new(move |_| {
                        state.set(StateMode::Deleting);
                    }),
                }
            }
        }
        StateMode::Deleting => rsx! {
            DeletingContent { current_path: volume_and_path.as_str().to_string(), items }
        },
    };

    rsx! {
        DialogTemplate {
            window_size_style: "width:600px",
            title: "Delete confirmation".to_string(),
            content,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum StateMode {
    Confirmation,
    Deleting,
}
