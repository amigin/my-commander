use dioxus::document;

pub fn format_bytes(v: u64) -> String {
    if v < 1024 {
        return format!("{}B", v);
    }

    let v = v as f64;
    let v = v / 1024.0;

    if v < 1024.0 {
        return format!("{:.2}KB", v);
    }

    let v = v / 1024.0;
    if v < 1024.0 {
        return format!("{:.2}MB", v);
    }

    let v = v / 1024.0;
    if v < 1024.0 {
        return format!("{:.2}GB", v);
    }

    let v = v / 1024.0;
    format!("{:.2}TB", v)
}

pub fn scroll_to_active_element() {
    document::eval(
        r#"
    
    setTimeout(()=>{
    document.getElementById('selected-line').scrollIntoViewIfNeeded();
    },10);
    "#,
    );
}
