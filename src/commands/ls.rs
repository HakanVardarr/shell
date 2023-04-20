use colored::Colorize;

pub struct Ls;

impl Ls {
    pub fn run() {
        let current_dir = std::fs::read_dir(".").expect("ERROR: Cannot read the dir");

        for entry in current_dir {
            let entry = entry.unwrap();
            let path = entry.file_name().into_string().unwrap();

            if !path.starts_with(".") {
                if entry.path().is_dir() {
                    println!("{}", path.bright_blue());
                } else {
                    println!("{}", path);
                }
            }
        }
    }
}
