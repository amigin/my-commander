use dioxus::{desktop::*, prelude::*};

use views::*;
mod views;

mod consts;
mod states;
mod utils;

use states::*;
fn main() {
    dioxus::LaunchBuilder::desktop()
        .with_cfg(
            Config::new().with_window(
                WindowBuilder::new()
                    .with_resizable(true)
                    .with_maximized(true)
                    .with_title("My commander"),
            ),
        )
        .launch(app)
}

#[component]
fn app() -> Element {
    use_context_provider(|| Signal::new(MainState::new()));

    let mut main_state = consume_context::<Signal<MainState>>();

    let main_state_read_access = main_state.read();

    let content = match main_state_read_access.persistence_state.as_ref() {
        DataState::None => {
            spawn(async move {
                main_state.write().persistence_state.set_loading();
                let persistence_state = PersistenceState::load().await;

                main_state
                    .write()
                    .persistence_state
                    .set_loaded(persistence_state);
            });
            rsx! { "Starting application..." }
        }
        DataState::Loading => rsx! { "Starting application..." },
        DataState::Loaded(_) => {
            rsx! {
                div { class: "left-panel",
                    Panel { left_panel: true }
                }
                div { class: "right-panel",
                    Panel { left_panel: false }
                }

                BottomPanel {}
            }
        }
        DataState::Error(err) => {
            rsx! {
                div { class: "app-start-error", {err.as_str()} }
            }
        }
    };

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/styled.css") }
        document::Link { rel: "stylesheet", href: asset!("/assets/app.css") }
        {content}
    }
}
