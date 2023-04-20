pub struct Ls;

impl Ls {
    pub fn run() {
        let current_dir = std::fs::read_dir(".").expect("ERROR: Cannot read the dir");

        for entry in current_dir {
            let entry = entry.unwrap();
            let path = entry.file_name().into_string().unwrap();

            if !path.starts_with(".") {
                println!("{}", path);
            }
        }
    }
}
