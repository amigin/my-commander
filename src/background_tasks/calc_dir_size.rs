use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use dioxus::prelude::*;

use crate::{volume_path_and_file::VolumePathAndFile, MainState};

pub struct DirSizeCalculationHandler {
    pub dir: VolumePathAndFile,
    pub canceled: AtomicBool,
    pub left_panel: bool,
}

impl DirSizeCalculationHandler {
    pub fn new(dir: VolumePathAndFile, left_panel: bool) -> Self {
        Self {
            canceled: AtomicBool::new(false),
            dir,
            left_panel,
        }
    }

    pub fn cancel(&self) {
        self.canceled.store(true, Ordering::Relaxed);
    }

    pub fn is_canceled(&self) -> bool {
        self.canceled.load(Ordering::Relaxed)
    }
}

pub async fn calc_dir_size(
    handler: Arc<DirSizeCalculationHandler>,
    main_state: &mut Signal<MainState>,
) {
    println!("Starting calculation for dir: {}", handler.dir.as_str());
    let mut dir_size = 0;
    let mut dirs_to_discover = vec![handler.dir.to_string()];

    let mut no = 0;

    while let Some(dir_to_discover) = dirs_to_discover.pop() {
        let dir_reader = tokio::fs::read_dir(dir_to_discover.as_str()).await;

        let mut dir_reader = match dir_reader {
            Ok(dir_reader) => dir_reader,
            Err(err) => {
                println!(
                    "Skipping discovering dir {}. Err: {:?}",
                    dir_to_discover.as_str(),
                    err
                );

                continue;
            }
        };

        while !handler.is_canceled() {
            no += 1;

            if no > 50000 {
                main_state
                    .write()
                    .get_panel_state_mut(handler.left_panel)
                    .set_dir_size(&handler.dir, dir_size, false);
                no = 0;
            }

            let entry = dir_reader.next_entry().await;

            let entry = match entry {
                Ok(entry) => entry,
                Err(err) => {
                    println!(
                        "Skipping reading entry from dir {}. Err: {:?}",
                        dir_to_discover.as_str(),
                        err
                    );
                    break;
                }
            };

            if entry.is_none() {
                break;
            }

            let entry = entry.unwrap();

            let metadata = entry.metadata().await;

            if metadata.is_err() {
                continue;
            }

            let metadata = metadata.unwrap();

            if metadata.is_dir() {
                dirs_to_discover.push(entry.path().to_str().unwrap().to_string());
            } else {
                dir_size += metadata.len();
            }
        }

        if handler.is_canceled() {
            break;
        }
    }

    if handler.is_canceled() {
        main_state
            .write()
            .get_panel_state_mut(handler.left_panel)
            .set_dir_size_unknown(&handler.dir);
    } else {
        println!(
            "Calculation for dir {} finished. Size: {}",
            handler.dir.as_str(),
            dir_size
        );

        main_state
            .write()
            .get_panel_state_mut(handler.left_panel)
            .set_dir_size(&handler.dir, dir_size, true);
    }
}
