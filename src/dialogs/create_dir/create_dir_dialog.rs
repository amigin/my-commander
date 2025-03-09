use std::rc::Rc;

use dioxus::prelude::*;

use crate::dialogs::DialogTemplate;
#[component]
pub fn CreateDirDialog(current_path: Rc<String>) -> Element {
    let mut component_state = use_signal(|| CreateDirDialogModel::default());
    let component_state_read_access = component_state.read();

    let current_path = current_path.clone();
    let buttons = crate::components::dialog_buttons(
        "Create",
        EventHandler::new(move |_| {
            let component_state_read_access = component_state.read();
            crate::actions::create_dir_dialog_ok_button(
                current_path.as_str(),
                &component_state_read_access.dir_name,
            );
        }),
    );

    let content = rsx! {

        div { style: "padding: 20px;",
            input {
                id: "dir-name-input",
                class: "form-control",
                placeholder: "Enter directory name",
                oninput: move |c| {
                    component_state.write().dir_name = c.value();
                },
                value: component_state_read_access.dir_name.as_str(),
            }


        }
        {buttons}
    };

    crate::utils::set_focus("dir-name-input");

    rsx! {
        DialogTemplate {
            title: "Create directory",
            window_size_style: "width:500px",
            content,
        }
    }
}

#[derive(Default, Debug)]
pub struct CreateDirDialogModel {
    pub dir_name: String,
}
