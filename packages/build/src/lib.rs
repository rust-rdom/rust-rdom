use std::{fs, path::Path};

use generator::Generator;
use sourcegen_cli::{run_sourcegen, SourceGenerator, SourcegenParameters};

pub(crate) mod generator;
pub(crate) mod template;

// recursive function traverses `dir`
// it also adds prefixes with '-' if the file is in a folder
fn read_dir(dir: impl AsRef<Path>, prefix: String, collect: &mut Vec<(String, Generator)>) {
    for entry in fs::read_dir(dir).unwrap().map(Result::unwrap) {
        // srry, this code is kind of a mess
        //                        - maksimil
        if entry.file_type().unwrap().is_file() {
            let key = format!("{}{}", prefix, entry.file_name().to_str().unwrap());
            let key = key[0..key.len() - 5].to_string();
            let generator = Generator::load(&entry.path());
            collect.push((key, generator));
        } else if entry.file_type().unwrap().is_dir() {
            let prefix = format!("{}{}-", prefix, entry.file_name().to_str().unwrap());
            read_dir(&entry.path(), prefix, collect);
        }
    }
}

pub fn main() {
    // data storage
    let data = {
        let mut data = Vec::new();
        read_dir("generators", "".to_string(), &mut data);
        data
    };

    // storage for references to that data (because sourcegen is built this way)
    let dataref = data
        .iter()
        .map(|(s, g)| (s.as_str(), g as &dyn SourceGenerator))
        .collect::<Vec<_>>();

    eprintln!("{:?}", data);
    let parameters = SourcegenParameters {
        generators: &dataref[..],
        ..Default::default()
    };
    run_sourcegen(&parameters).unwrap();
}
