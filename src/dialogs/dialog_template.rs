use dioxus::prelude::*;

use crate::MainState;

#[component]
pub fn DialogTemplate(window_size_style: String, title: String, content: Element) -> Element {
    rsx! {
        table {
            id: "dialog-pad",
            onkeydown: |c| {
                c.stop_propagation();
                c.cancel_bubble();
            },
            onkeypress: |c| {
                c.stop_propagation();
            },
            onkeyup: |c| {
                c.stop_propagation();
            },
            tr { style: "height: 100%; width:100%;",
                td {
                    div {
                        onkeypress: |ctx| {
                            if ctx.key() == Key::Escape {
                                consume_context::<Signal<MainState>>().write().hide_dialog();
                            }
                        },
                        id: "dialog-window",
                        style: "margin: auto; {window_size_style}",
                        table { class: "window-header", style: "width:100%",
                            tr {
                                td { style: "width:2%",
                                    div {
                                        class: "close-button",
                                        onclick: |_| {
                                            consume_context::<Signal<MainState>>().write().hide_dialog();
                                        },
                                    }
                                }
                                td { style: "width:98%",
                                    div { {title} }
                                }
                            }
                        }
                        {content}
                    }
                }
            }
        }
    }
}
