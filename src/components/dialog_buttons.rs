use dioxus::prelude::*;

use crate::states::MainState;
pub fn dialog_buttons(ok_text: &str, on_ok: EventHandler<()>) -> Element {
    rsx! {
        div { class: "dialog-buttons",
            div { class: "dialog-btn-group",
                button {
                    tabindex: "1",
                    class: "mac-button",
                    onclick: move |_| {
                        on_ok.call(());
                    },
                    style: "margin-right: 10px",
                    {ok_text}
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
