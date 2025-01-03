use crate::consts::*;
use dioxus::document;

pub fn format_bytes(v: u64) -> String {
    if v < 1024 {
        return format!("{}", v);
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
    },200);
    "#,
    );
}

pub fn set_panel_focus(left_panel_active: bool) {
    let id = if left_panel_active {
        LEFT_PANEL_ID
    } else {
        RIGHT_PANEL_ID
    };

    let command = format!(
        r#"
        document.getElementById('{id}').focus();
        "#,
    );

    document::eval(command.as_str());
}

pub fn set_focus(id: &str) {
    let command = format!(
        r#"
            setTimeout(()=>{{
            document.getElementById('{id}').focus();
          }}, 50)
        "#,
        id = id
    );

    document::eval(command.as_str());
}
