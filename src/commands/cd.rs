pub struct Cd;

impl Cd {
    pub fn run(path: &str) {
        if path == ".." {
            std::env::set_current_dir(&path);
        } else {
            let current_dir = std::fs::read_dir(".").expect("ERROR: Cannot read the dir");
            for entry in current_dir {
                let entry = entry.unwrap();
                if entry.path().is_dir() && entry.file_name() == path {
                    std::env::set_current_dir(&path);
                }
            }
        }
    }
}
