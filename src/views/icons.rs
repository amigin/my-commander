use dioxus::prelude::*;
pub fn render_icon(file_name: &str) -> Element {
    let ext = file_name.split('.').last();

    if let Some(ext) = ext {
        if ext.eq_ignore_ascii_case("pdf") {
            return rsx! {
                img { class: "file-ico", src: asset!("/assets/ico/pdf.svg") }
            };
        }
        if ext.eq_ignore_ascii_case("svg") {
            return rsx! {
                img { class: "file-ico", src: asset!("/assets/ico/svg.svg") }
            };
        }
        if ext.eq_ignore_ascii_case("zip") {
            return rsx! {
                img { class: "file-ico", src: asset!("/assets/ico/zip.svg") }
            };
        }
        if ext.eq_ignore_ascii_case("png") {
            return rsx! {
                img { class: "file-ico", src: asset!("/assets/ico/png.svg") }
            };
        }
        if ext.eq_ignore_ascii_case("jpg") {
            return rsx! {
                img { class: "file-ico", src: asset!("/assets/ico/jpg.svg") }
            };
        }
        if ext.eq_ignore_ascii_case("jpeg") {
            return rsx! {
                img { class: "file-ico", src: asset!("/assets/ico/jpg.svg") }
            };
        }
        if ext.eq_ignore_ascii_case("csv") {
            return rsx! {
                img { class: "file-ico", src: asset!("/assets/ico/csv.svg") }
            };
        }
        if ext.eq_ignore_ascii_case("gif") {
            return rsx! {
                img { class: "file-ico", src: asset!("/assets/ico/gif.svg") }
            };
        }
        if ext.eq_ignore_ascii_case("giff") {
            return rsx! {
                img { class: "file-ico", src: asset!("/assets/ico/gif.svg") }
            };
        }
        if ext.eq_ignore_ascii_case("pfx") {
            return rsx! {
                img { class: "file-ico", src: asset!("/assets/ico/cert.svg") }
            };
        }
        if ext.eq_ignore_ascii_case("p12") {
            return rsx! {
                img { class: "file-ico", src: asset!("/assets/ico/cert.svg") }
            };
        }
        if ext.eq_ignore_ascii_case("torrent") {
            return rsx! {
                img { class: "file-ico", src: asset!("/assets/ico/torrent.png") }
            };
        }
        if ext.eq_ignore_ascii_case("mp4") {
            return rsx! {
                img { class: "file-ico", src: asset!("/assets/ico/mp4.svg") }
            };
        }
        if ext.eq_ignore_ascii_case("txt") {
            return rsx! {
                img { class: "file-ico", src: asset!("/assets/ico/txt.svg") }
            };
        }
        if ext.eq_ignore_ascii_case("docx") {
            return rsx! {
                img { class: "file-ico", src: asset!("/assets/ico/docx.svg") }
            };
        }
    }

    rsx! {
        img { class: "file-ico", src: asset!("/assets/ico/file.svg") }
    }
}
