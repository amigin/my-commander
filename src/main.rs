use dialogs::RenderDialog;
use dioxus::{desktop::*, prelude::*};

use background_tasks::*;
use views::*;
mod actions;
mod background_tasks;
mod consts;
mod dialogs;
mod states;
mod utils;
mod views;
mod volume_path_and_file;
use states::*;
use wry::http::Response;
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
    use_asset_handler("logos", |request, response| {
        // We get the original path - make sure you handle that!
        let content = std::fs::read(request.uri().path()).unwrap();

        response.respond(Response::new(content));
    });
    use futures_util::StreamExt;

    let mut persistence_state: Signal<DataState<PersistenceState>> = use_signal(|| DataState::None);

    let persistence_state = match persistence_state.read().as_ref() {
        DataState::None => {
            spawn(async move {
                persistence_state.write().set_loading();
                let value = PersistenceState::load().await;
                persistence_state.write().set_loaded(value);
            });

            return rsx! {
                Layout { content: rsx! { "Starting application..." } }
            };
        }
        DataState::Loading => {
            return rsx! {
                Layout { content: rsx! { "Starting application..." } }
            }
        }
        DataState::Loaded(persistence_state) => persistence_state.clone(),
        DataState::Error(err) => {
            return rsx! {
                Layout {
                    content: rsx! {
                        div { class: "app-start-error", {err.as_str()} }
                    },
                }
            }
        }
    };

    let tx = use_coroutine(
        move |mut rx: UnboundedReceiver<BackgroundTask>| async move {
            println!("Starting size calculator coroutine");
            let mut main_state = consume_context::<Signal<MainState>>();
            loop {
                if let Some(task) = rx.next().await {
                    match task {
                        BackgroundTask::CalcDirSize(task) => {
                            crate::background_tasks::calc_dir_size(task, &mut main_state).await
                        }
                        BackgroundTask::SaveState(state) => {
                            crate::background_tasks::save_state(state).await
                        }
                    };
                }
            }
        },
    );

    use_context_provider(|| Signal::new(MainState::new(persistence_state, tx)));

    rsx! {

        Layout {
            content: rsx! {
                div { class: "left-panel",
                    Panel { left_panel: true }
                }
                div { class: "right-panel",
                    Panel { left_panel: false }
                }

                BottomPanel {}
            },
        }
        RenderDialog {}
    }
}

#[component]
fn Layout(content: Element) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/styled.css") }
        document::Link { rel: "stylesheet", href: asset!("/assets/app.css") }
        {content}
    }
}
