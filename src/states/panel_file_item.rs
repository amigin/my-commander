use std::{fs::Metadata, time::UNIX_EPOCH};

use rust_extensions::date_time::DateTimeAsMicroseconds;

#[derive(Debug, Clone, Copy)]
pub enum FileLineType {
    Dir,
    File,
    Back,
}

impl FileLineType {
    pub fn is_dir(&self) -> bool {
        match self {
            FileLineType::Dir => true,
            _ => false,
        }
    }
}

pub struct PanelFileItem {
    pub name: String,
    pub size: u64,
    pub created: DateTimeAsMicroseconds,
    pub modified: DateTimeAsMicroseconds,
    pub hidden: bool,
    pub tp: FileLineType,
    pub marked: bool,
}

impl PanelFileItem {
    pub fn new_back() -> Self {
        Self {
            tp: FileLineType::Back,
            hidden: false,
            name: "..".to_string(),
            size: 0,
            created: DateTimeAsMicroseconds::new(0),
            modified: DateTimeAsMicroseconds::new(0),
            marked: false,
        }
    }
    pub fn new(metadata: Metadata, name: String) -> Self {
        let size = metadata.len();
        let created = DateTimeAsMicroseconds::new(
            metadata
                .created()
                .unwrap()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_micros() as i64,
        );
        let modified = DateTimeAsMicroseconds::new(
            metadata
                .modified()
                .unwrap()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_micros() as i64,
        );
        Self {
            tp: if metadata.is_dir() {
                FileLineType::Dir
            } else {
                FileLineType::File
            },
            hidden: name.starts_with("."),
            name,
            size,
            created,
            modified,
            marked: false,
        }
    }
}
