use std::fs::{File, OpenOptions};
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

pub fn process_line(line: &str, input_root: &str, output_root: &str) -> String {
    if !line.starts_with("#") {
        return Path::new(output_root)
            .join(Path::new(&line).strip_prefix(input_root).unwrap())
            .to_str()
            .unwrap()
            .to_string();
    }
    line.to_string()
}

pub fn process_playlists(input_dir: &str, input_root: &str, output_dir: &str, output_root: &str) {
    let playlist_paths = Path::new(input_dir).read_dir().unwrap();
    playlist_paths.for_each(|entry| {
        let entry = entry.unwrap();
        println!("Processing {:?}", entry.path());

        let file = File::open(entry.path()).unwrap();
        let reader = BufReader::new(file);

        let dest_path = Path::new(output_dir).join(&entry.file_name());
        let mut dest_file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(dest_path)
            .unwrap();

        reader.lines().for_each(|line: Result<String, io::Error>| {
            let mut line = line.unwrap();
            line = process_line(&line, input_root, output_root);
            writeln!(dest_file, "{}", line).unwrap();
        });
    });
}
