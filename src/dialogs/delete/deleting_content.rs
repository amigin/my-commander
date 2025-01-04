use dioxus::prelude::*;

use crate::MainState;

#[component]
pub fn DeletingContent() -> Element {
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
