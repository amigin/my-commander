use std::env;

pub struct DiskInfo {
    pub name: String,
    pub path: String,
    pub available_space: u64,
    pub total_space: u64,
    pub default_path: String,
}

impl DiskInfo {
    pub fn get_display_name(&self) -> String {
        format!(
            "{} [{} of {}]",
            self.name,
            crate::utils::format_bytes(self.available_space),
            crate::utils::format_bytes(self.total_space)
        )
    }
}
pub struct DisksState {
    pub disks: Vec<DiskInfo>,
}

impl DisksState {
    pub fn new() -> Self {
        let disks_to_iterate = sysinfo::Disks::new_with_refreshed_list();
        let mut disks = Vec::with_capacity(disks_to_iterate.len());
        for disk in disks_to_iterate.list() {
            let path = disk.mount_point().as_os_str().to_str();

            if path.is_none() {
                continue;
            }

            let path = path.unwrap().to_string();

            let default_path = if path == "/" {
                env::var("HOME").unwrap_or_default()
            } else {
                "".to_string()
            };

            disks.push(DiskInfo {
                name: match disk.name().to_str() {
                    Some(name) => name.to_string(),
                    None => "Unknown".to_string(),
                },
                path,
                default_path,
                available_space: disk.available_space(),
                total_space: disk.total_space(),
            });
        }

        Self { disks }
    }

    pub fn iter(&self) -> impl Iterator<Item = &DiskInfo> {
        self.disks.iter()
    }
}
