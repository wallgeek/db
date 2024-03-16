pub mod def;
pub mod env;

use std::fs::OpenOptions;
use std::io::ErrorKind;
use std::path::PathBuf;

pub fn file(file_name: &str) -> String {
    let mut full_path = PathBuf::from(env::get_folder_path());
    
    full_path.push(file_name);

    let full_path_as_string = full_path.to_str().unwrap().to_owned();

    let open = OpenOptions::new()
        .read(true)
        .write(true)
        .create_new(true)
        .open(full_path);

    match open {
        Ok(_) => {
            println!("Data file created successfully");
            return full_path_as_string
        }
        Err(err) => {
            if err.kind() != ErrorKind::AlreadyExists {
                panic!("{:?}", err);
            } else {
                println!("Data file already exists");
                return full_path_as_string
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;
    use std::fs;

    #[test]
    #[should_panic]
    fn data_file_without_directory() {
        // data_file("foo/bar")
    }

    #[test]
    fn data_file_with_directory_without_file(){
        let folder_path = env::get_folder_path();
        
        if folder_path.as_str() == def::FOLDER_NAME {
            panic!("Please provide a different directory as a environment for test cases")
        }

        file(&folder_path);
    }
}
