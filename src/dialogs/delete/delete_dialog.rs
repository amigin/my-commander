use super::super::*;
use crate::{
    render_folder_icon, render_icon, volume_path_and_file::VolumePathAndFile, PanelFileItem,
};
use dioxus::prelude::*;

#[component]
pub fn DeleteDialog(
    amount: usize,
    volume_and_path: VolumePathAndFile,
    selected_item: PanelFileItem,
    on_ok: EventHandler<()>,
) -> Element {
    let mut state = use_signal(|| StateMode::Confirmation);
    use_effect(|| {
        crate::utils::set_focus("btn-cancel");
    });

    let state_value = state.read().clone();

    let content = match state_value {
        StateMode::Confirmation => {
            let phrase = match selected_item.tp {
                crate::FileLineType::Dir => {
                    let icon = render_folder_icon();
                    rsx! {
                        div {
                            "Are you sure you want to delete "
                            {icon}
                            span { style: "font-weight:800", {selected_item.name} }
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
                            span { style: "font-weight:800", {selected_item.name} }
                            " file?"
                        }
                    }
                }
                crate::FileLineType::Back => {
                    panic!("Back should not be in the list");
                }
            };

            rsx! {
                ConfirmationContent {
                    amount,
                    phrase,
                    on_ok: EventHandler::new(move |_| {
                        state.set(StateMode::Deleting);
                    }),
                }
            }
        }
        StateMode::Deleting => rsx! {
            DeletingContent {}
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
