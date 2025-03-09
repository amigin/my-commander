use dioxus::prelude::*;
use rust_extensions::StrOrString;

use crate::MainState;
#[component]
pub fn BottomPanel() -> Element {
    let (active_path, file_or_dir_selected, has_items_selected, file_selected) = {
        let main_state = consume_context::<Signal<MainState>>();
        let main_state_read_access = main_state.read();

        let active_panel = main_state_read_access.get_active_panel();
        let active_path = active_panel.volume_and_path.get_path();
        let active_path: StrOrString = if active_path.is_empty() {
            "/".into()
        } else {
            active_path.to_string().into()
        };

        let (file_or_dir_selected, file_selected) = match active_panel.try_get_selected_item() {
            Some(selected_item) => (
                selected_item.tp.is_file() || selected_item.tp.is_dir(),
                selected_item.tp.is_file(),
            ),
            None => (false, false),
        };

        (active_path, file_or_dir_selected, false, file_selected)
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
                            disabled: !file_selected,
                            onclick: move |_| {
                                crate::actions::view_file(None);
                            },
                            "F3 - View"
                        }
                    }
                    td {
                        button {
                            tabindex: -1,
                            class: "btn btn-secondary btn-light bottom-button",
                            disabled: !file_selected,
                            "F4 - Edit"
                        }
                    }
                    td {
                        button {
                            tabindex: -1,
                            class: "btn btn-secondary btn-light bottom-button",
                            disabled: !file_or_dir_selected && !has_items_selected,
                            "F5 - Copy"
                        }
                    }
                    td {
                        button {
                            tabindex: -1,
                            class: "btn btn-secondary btn-light bottom-button",
                            disabled: !file_or_dir_selected && !has_items_selected,
                            "F6 - Move"
                        }
                    }
                    td {
                        button {
                            tabindex: -1,
                            class: "btn btn-secondary btn-light bottom-button",
                            onclick: move |_| {
                                println!("F7 - Create Dir");
                                crate::actions::create_dir();
                            },
                            "F7 - New Folder"
                        }
                    }
                    td {
                        button {
                            tabindex: -1,
                            disabled: !file_or_dir_selected && !has_items_selected,
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
