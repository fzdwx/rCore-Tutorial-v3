use std::fs::read_dir;

fn main() {
    if let Ok(dir) = read_dir("./") {
        for dir_entry in dir {
            if let Ok(dir_entry) = dir_entry {
                println!("{:?}", dir_entry.file_name())
            }
        }
    }
}
