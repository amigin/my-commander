use dioxus::prelude::*;

use crate::states::MainState;
pub fn dialog_buttons(ok_text: &str, ok_focus: bool, on_ok: EventHandler<()>) -> Element {
    if ok_focus {
        crate::utils::set_focus("btn-confirm");
    }

    rsx! {
        div { class: "dialog-buttons",
            div { class: "dialog-btn-group",
                button {
                    id: "btn-confirm",
                    tabindex: "1",
                    class: "mac-button",
                    onclick: move |_| {
                        on_ok.call(());
                    },
                    onkeyup: |c| {
                        c.stop_propagation();
                    },
                    style: "margin-right: 10px",
                    {ok_text}
                }
                button {
                    id: "btn-cancel",
                    tabindex: "1",
                    class: "mac-gray-button",
                    onclick: |c| {
                        println!("Pressed Dialog Cancel");
                        consume_context::<Signal<MainState>>().write().hide_dialog();
                    },
                    onkeypress: |c| {
                        println!("Cancel Key Up Dialog");
                        println!("Propagates: {}", c.propagates());
                        c.stop_propagation();
                    },
                    "Cancel"
                }
            }
        }
    }
}
