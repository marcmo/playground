use std::path::Path;

pub fn find_filename(path: &Path) -> String {
    path.file_name().map(String::from).unwrap_or_default()
}
