use std::fs::{self, OpenOptions};
use std::io::{self, prelude::*};
use std::path::Path;

pub fn process_line(line: &str, input_root: &str, output_root: &str) -> String {
    if !line.starts_with("#") {
        Path::new(output_root)
            .join(Path::new(&line).strip_prefix(input_root).unwrap())
            .to_str()
            .unwrap()
            .to_string()
    } else {
        line.to_string()
    }
}

pub fn process_playlists(input_dir: &str, input_root: &str, output_dir: &str, output_root: &str) {
    let playlist_paths = Path::new(input_dir).read_dir().unwrap();
    playlist_paths.for_each(|entry| {
        let entry = entry.unwrap();
        println!("Processing {:?}", entry.path());

        let reader = io::BufReader::new(fs::File::open(entry.path()).unwrap());
        let mut writer = io::BufWriter::new(
            OpenOptions::new()
                .write(true)
                .create(true)
                .open(Path::new(output_dir).join(&entry.file_name()))
                .unwrap(),
        );

        reader.lines().for_each(|line| {
            writeln!(
                writer,
                "{}",
                process_line(&line.unwrap(), input_root, output_root)
            )
            .unwrap();
        });
    });
}
