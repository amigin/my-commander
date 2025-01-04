use dioxus::prelude::*;

use crate::MainState;
#[component]
pub fn ConfirmationContent(amount: usize, phrase: Element, on_ok: EventHandler<()>) -> Element {
    rsx! {
        div { class: "dialog-content", {phrase} }
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
                        consume_context::<Signal<MainState>>().write().hide_dialog();
                    },
                    "Cancel"
                }
            }
        }
    }
}
