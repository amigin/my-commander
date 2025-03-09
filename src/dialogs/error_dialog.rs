use dioxus::prelude::*;

use crate::states::MainState;

use super::*;

#[component]
pub fn ErrorDialog(title: &'static str, message: String) -> Element {
    let content = rsx! {

        table { style: "width:100%",
            tr {
                td { style: "padding: 20px;",
                    img {
                        src: asset!("/assets/ico/error.png"),
                        style: "width:32px",
                    }
                }
                td { {message.as_str()} }
            }
        }

        div { class: "dialog-buttons",
            div { class: "dialog-btn-group",
                button {
                    id: "btn-cancel",
                    tabindex: "1",
                    class: "mac-gray-button",
                    onclick: |_| {
                        consume_context::<Signal<MainState>>().write().hide_dialog();
                    },
                    "Close"
                }
            }
        }
    };
    rsx! {
        DialogTemplate { title, window_size_style: "width:500px", content }
    }
}
