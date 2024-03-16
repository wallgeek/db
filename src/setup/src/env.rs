use crate::def;
use std::env;
use std::path::PathBuf;

pub enum Mode {
    WithoutInventory,
    WithInventory,
    OnlyInventory
}

pub fn set_folder_path(o_folder_path: Option<&str>) {
    let mut folder_path = PathBuf::from(def::DATA_FILE_NAME);

    if let Some(folder_name) = o_folder_path {
        folder_path = PathBuf::from(folder_name);
    }

    env::set_var("FOLDER_PATH", folder_path);
}

pub fn get_folder_path() -> String {
    env::var("FOLDER_PATH").unwrap_or_else(|_| def::FOLDER_NAME.to_string())
}

pub fn set_index_setup() {

}

pub fn has_index_setup() -> bool {
    true
}

pub fn get_mode() -> Mode {
    Mode::WithoutInventory
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn folder_path() {
        // get path without env
        assert_eq!(get_folder_path(), def::FOLDER_NAME);

        // set env
        let another_folder_path = "foo/bar";
        let o_folder_path = Some(another_folder_path);
        
        set_folder_path(o_folder_path);

        assert_eq!(get_folder_path(), another_folder_path);
    }
}
