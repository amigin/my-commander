use dioxus::prelude::*;

#[component]
pub fn ConfirmationContent(phrase: Element, on_ok: EventHandler<()>) -> Element {
    let buttons = crate::components::dialog_buttons("Delete", true, on_ok);
    rsx! {
        div { class: "dialog-content", {phrase} }
        {buttons}
    }
}
