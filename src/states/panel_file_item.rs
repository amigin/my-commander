use std::{fs::Metadata, time::UNIX_EPOCH};

use rust_extensions::date_time::DateTimeAsMicroseconds;

#[derive(Debug, Clone, Copy)]
pub enum FileLineType {
    Dir,
    File,
    Back,
}

impl FileLineType {
    pub fn is_back(&self) -> bool {
        match self {
            FileLineType::Back => true,
            _ => false,
        }
    }
    pub fn is_dir(&self) -> bool {
        match self {
            FileLineType::Dir => true,
            _ => false,
        }
    }

    pub fn is_file(&self) -> bool {
        match self {
            FileLineType::File => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum FileItemSize {
    Unknown,
    Calculating(u64),
    Known(u64),
}

impl FileItemSize {
    pub fn is_unknown(&self) -> bool {
        match self {
            FileItemSize::Unknown => true,
            _ => false,
        }
    }

    pub fn is_calculating(&self) -> bool {
        match self {
            FileItemSize::Calculating(_) => true,
            _ => false,
        }
    }

    pub fn is_known(&self) -> bool {
        match self {
            FileItemSize::Known(_) => true,
            _ => false,
        }
    }
}

impl FileItemSize {
    pub fn get_size(&self) -> u64 {
        match self {
            FileItemSize::Unknown => 0,
            FileItemSize::Calculating(size) => *size,
            FileItemSize::Known(size) => *size,
        }
    }

    pub fn get_formatted_size_as_string(&self) -> String {
        match self {
            FileItemSize::Unknown => String::new(),
            FileItemSize::Calculating(size) => crate::utils::format_bytes(*size),
            FileItemSize::Known(size) => crate::utils::format_bytes(*size),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PanelFileItem {
    pub name: String,
    pub size: FileItemSize,
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
            size: FileItemSize::Unknown,
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
        let is_dir = metadata.is_dir();
        Self {
            tp: if is_dir {
                FileLineType::Dir
            } else {
                FileLineType::File
            },
            hidden: name.starts_with("."),
            name,
            size: if is_dir {
                FileItemSize::Unknown
            } else {
                FileItemSize::Known(size)
            },
            created,
            modified,
            marked: false,
        }
    }
}
