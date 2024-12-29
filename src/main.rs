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
                    .with_title("My commander"),
            ),
        )
        .launch(app)
}

#[component]
fn app() -> Element {
    use_context_provider(|| Signal::new(MainState::new()));

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/styled.css") }
        document::Link { rel: "stylesheet", href: asset!("/assets/app.css") }



        div { class: "left-panel",
            Panel { left_panel: true }
        }
        div { class: "right-panel",
            Panel { left_panel: false }
        }



        BottomPanel {}
    }
}

/*
table { style: "width:100%; height:calc(var(--app-height) - var(--bottom-panel)) ",
           tr {
               td { style: "width:50vw; border-right: 1px solid #ccc",
                   Panel { left_panel: true }
               }
               td { style: "width:50vw; border-left: 1px solid #ccc",
                   Panel { left_panel: false }
               }
           }
       }
*/
