#[derive(Debug, Clone, PartialEq)]
pub struct VolumePathAndFile {
    data: String,
    volume_length: usize,
}

impl VolumePathAndFile {
    pub fn new(mut volume: String) -> Self {
        if !volume.ends_with(std::path::MAIN_SEPARATOR) {
            volume.push(std::path::MAIN_SEPARATOR);
        }

        let volume_length = volume.len();
        Self {
            data: volume,
            volume_length,
        }
    }

    pub fn new_with_path(volume: String, path: &str) -> Self {
        println!("Creating: {}", volume);
        let volume_length = volume.len();
        let mut result = Self {
            data: volume,
            volume_length,
        };

        if path.starts_with(std::path::MAIN_SEPARATOR) {
            result.append_segment(&path[1..]);
        } else {
            result.append_segment(path);
        };

        result
    }

    pub fn to_string(&self) -> String {
        self.data.clone()
    }

    pub fn into_string(self) -> String {
        self.data
    }

    pub fn new_with_segment(&self, segment: &str) -> Self {
        let mut result = self.clone();
        result.append_segment(segment);
        result
    }

    pub fn append_segment(&mut self, segment: &str) {
        if !self.data.ends_with(std::path::MAIN_SEPARATOR) {
            self.data.push(std::path::MAIN_SEPARATOR);
        }
        self.data.push_str(segment);
    }

    pub fn as_str(&self) -> &str {
        &self.data
    }

    pub fn clear(&mut self) {
        self.data.truncate(self.volume_length);
    }

    pub fn get_last_segment(&self) -> Option<(&str, &str)> {
        let index = self.get_last_segment_index()?;
        Some((&self.data[..index], &self.data[index + 1..]))
    }

    pub fn get_last_segment_index(&self) -> Option<usize> {
        let bytes = self.data.as_bytes();

        for i in (0..bytes.len()).rev() {
            if bytes[i] == std::path::MAIN_SEPARATOR as u8 {
                return Some(i);
            }
        }

        None
    }

    pub fn get_volume(&self) -> &str {
        &self.data[..self.volume_length]
    }

    pub fn get_path(&self) -> &str {
        if self.data.len() == self.volume_length {
            return "";
        }
        &self.data[self.volume_length..]
    }

    pub fn go_back(&mut self) -> Option<String> {
        if self.data.len() == self.volume_length {
            return None;
        }

        let index = self.get_last_segment_index()?;

        let result = self.data[index + 1..].to_string();

        if index < self.volume_length {
            self.data.truncate(self.volume_length);
        } else {
            self.data.truncate(index);
        }

        return Some(result);
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

impl Into<VolumePathAndFile> for String {
    fn into(self) -> VolumePathAndFile {
        VolumePathAndFile::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::VolumePathAndFile;

    #[test]
    fn base_creation_with_no_separator_at_the_end() {
        let path = VolumePathAndFile::new("/Test".to_string());

        assert_eq!(path.to_string(), "/Test/".to_string());
        assert_eq!(path.get_volume(), "/Test/");
        assert_eq!(path.get_path(), "");
    }

    #[test]
    fn base_creation_with_separator_at_the_end() {
        let path = VolumePathAndFile::new("/Test/".to_string());

        assert_eq!(path.to_string(), "/Test/".to_string());
        assert_eq!(path.get_volume(), "/Test/");
        assert_eq!(path.get_path(), "");
    }

    #[test]
    fn test_appending_and_removing_path() {
        let mut path = VolumePathAndFile::new("/Test".to_string());

        path.append_segment("path");

        assert_eq!(path.to_string(), "/Test/path");
        assert_eq!(path.get_volume(), "/Test/");
        assert_eq!(path.get_path(), "path");

        path.append_segment("path2");

        assert_eq!(path.to_string(), "/Test/path/path2");
        assert_eq!(path.get_volume(), "/Test/");
        assert_eq!(path.get_path(), "path/path2");

        let last_path = path.go_back();
        assert_eq!(last_path, Some("path2".to_string()));
        assert_eq!(path.to_string(), "/Test/path");
        assert_eq!(path.get_volume(), "/Test/");
        assert_eq!(path.get_path(), "path");

        let last_path = path.go_back();
        assert_eq!(last_path, Some("path".to_string()));
        assert_eq!(path.to_string(), "/Test/");
        assert_eq!(path.get_volume(), "/Test/");
        assert_eq!(path.get_path(), "");

        let last_path = path.go_back();
        assert!(last_path.is_none());
        assert_eq!(path.to_string(), "/Test/");
        assert_eq!(path.get_volume(), "/Test/");
        assert_eq!(path.get_path(), "");
    }
}
