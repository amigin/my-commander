use dioxus::prelude::*;
use rust_extensions::StrOrString;

use crate::MainState;
#[component]
pub fn BottomPanel() -> Element {
    let active_path: StrOrString = {
        let main_state = consume_context::<Signal<MainState>>();
        let main_state_read_access = main_state.read();

        let active_path = main_state_read_access.get_active_path();
        if active_path.is_empty() {
            "/".into()
        } else {
            active_path.to_string().into()
        }
    };
    rsx! {
        div { class: "bottom-panel",
            table { style: "width:100%",
                tr {
                    td { style: "text-wrap: nowrap;", {active_path.as_str()} }
                    td { style: "width:100%",
                        input {
                            tabindex: -1,
                            class: "from-control-sm",
                            style: "border: 1px solid #ccc; width: 100%; border-radius: 3px",
                            onkeyup: move |event| {},
                        }
                    }
                }
            }

            table { style: "width:100%",
                tr {
                    td {
                        button {
                            tabindex: -1,
                            class: "btn btn-secondary btn-light bottom-button",
                            "F3 - View"
                        }
                    }
                    td {
                        button {
                            tabindex: -1,
                            class: "btn btn-secondary btn-light bottom-button",
                            "F4 - Edit"
                        }
                    }
                    td {
                        button {
                            tabindex: -1,
                            class: "btn btn-secondary btn-light bottom-button",
                            "F5 - Copy"
                        }
                    }
                    td {
                        button {
                            tabindex: -1,
                            class: "btn btn-secondary btn-light bottom-button",
                            "F6 - Move"
                        }
                    }
                    td {
                        button {
                            tabindex: -1,
                            class: "btn btn-secondary btn-light bottom-button",
                            "F7 - New Folder"
                        }
                    }
                    td {
                        button {
                            tabindex: -1,
                            onclick: move |_| {
                                println!("F8 - Delete");
                                crate::actions::delete(None);
                            },
                            class: "btn btn-secondary btn-light bottom-button",
                            "F8 - Delete"
                        }
                    }
                }
            }
        }
    }
}

/*

*/
