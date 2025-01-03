use super::*;
use crate::states::*;
use dioxus::prelude::*;

#[component]
pub fn DeleteConfirmationDialog(amount: usize, on_ok: EventHandler<()>) -> Element {
    use_effect(|| {
        crate::utils::set_focus("btn-cancel");
    });
    rsx! {
        DialogTemplate {
            window_size_style: "width:600px",
            title: "Delete confirmation".to_string(),
            content: rsx! {
                div { class: "confirmation-dialog", "Please confirm that you want to delete {amount} files/dirs" }
                div { class: "dialog-buttons",
                    div { class: "dialog-btn-group",
                        button {
                            tabindex: "1",
                            class: "mac-button",
                            onclick: move |_| {
                                on_ok.call(());
                            },
                            style: "margin-right: 10px",
                            "Delete"
                        }
                        button {
                            id: "btn-cancel",
                            tabindex: "1",
                            class: "mac-gray-button",
                            onclick: |_| {
                                consume_context::<Signal<MainState>>().write().dialog = None;
                            },
                            "Cancel"
                        }
                    }
                }
            },
        }
    }
}

/*

  button { class: "btn btn-primary", "Delete" }
                        button {
                            class: "btn btn-light",
                            onclick: |_| {
                                consume_context::<Signal<MainState>>().write().dialog = None;
                            },
                            "Cancel"
                        }
*/
