use dioxus::prelude::*;

use crate::MainState;

#[component]
pub fn DialogTemplate(window_size_style: String, title: String, content: Element) -> Element {
    rsx! {
        table { id: "dialog-pad",
            tr { style: "height: 100%; width:100%;",
                td {
                    div {
                        onkeypress: |ctx| {
                            if ctx.key() == Key::Escape {
                                consume_context::<Signal<MainState>>().write().dialog = None;
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
                                            consume_context::<Signal<MainState>>().write().dialog = None;
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
