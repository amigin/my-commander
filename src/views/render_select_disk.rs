use crate::states::*;
use dioxus::prelude::*;

pub fn render_select_disk(
    main_state_read_access: &MainState,
    panel_state: &PanelState,
    left_panel: bool,
) -> Element {
    let disks = main_state_read_access.disks.iter().map(|disk| {
        //println!("Volume: {}", panel_state.volume_and_path.get_volume());

        //println!("Disk.Path: {}", disk.path.as_str());

        rsx! {
            option {
                selected: panel_state.volume_and_path.get_volume() == disk.path.as_str(),
                value: disk.path.as_str(),
                {disk.get_display_name()}
            }
        }
    });

    rsx! {
        select {
            tabindex: -1,
            class: "form-select select-disk",
            oninput: move |ctx| {
                consume_context::<Signal<MainState>>()
                    .write()
                    .get_panel_state_mut(left_panel)
                    .set_selected_volume(ctx.value());
            },

            {disks}
        }
    }
}
