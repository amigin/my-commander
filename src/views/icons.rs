use dioxus::prelude::*;
pub fn render_icon(file_name: &str) -> Element {
    let ext = file_name.split('.').last();

    if let Some(ext) = ext {
        if ext.eq_ignore_ascii_case("pdf") {
            return rsx! {
                img { class: "file-ico", src: asset!("/assets/ico/pdf.svg") }
            };
        }
    }

    rsx! {
        img { class: "file-ico", src: asset!("/assets/ico/file.svg") }
    }
}
