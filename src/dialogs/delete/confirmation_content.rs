use dioxus::prelude::*;

#[component]
pub fn ConfirmationContent(amount: usize, phrase: Element, on_ok: EventHandler<()>) -> Element {
    let buttons = crate::components::dialog_buttons("Delete", on_ok);
    rsx! {
        div { class: "dialog-content", {phrase} }
        {buttons}
    }
}
