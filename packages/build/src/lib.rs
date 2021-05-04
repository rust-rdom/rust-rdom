use std::{fs, path::Path};

use generator::InjectedStructGenerator;
use sourcegen_cli::{run_sourcegen, SourceGenerator, SourcegenParameters};

pub(crate) mod behavior;
pub(crate) mod generator;
pub(crate) mod template;

// recursive function traverses `dir`
// it also adds prefixes with '-' if the file is in a folder
fn read_dir<'a>(dir: impl AsRef<Path>, prefix: String, collect: &mut Vec<(String, &mut Box<dyn SourceGenerator>)>) {
    for entry in fs::read_dir(dir).unwrap().map(Result::unwrap) {
        // srry, this code is kind of a mess
        //                        - maksimil
        if entry.file_type().unwrap().is_file() {
            let key = format!("{}{}", prefix, entry.file_name().to_str().unwrap());
            let key = key[0..key.len() - 5].to_string();
            let generator = InjectedStructGenerator::load(&entry.path());
            collect.push((key, &mut (Box::new(generator) as Box<dyn SourceGenerator>)));
        } else if entry.file_type().unwrap().is_dir() {
            let prefix = format!("{}{}-", prefix, entry.file_name().to_str().unwrap());
            read_dir(&entry.path(), prefix, collect);
        }
    }
}

pub fn main<'a>() {
    // data storage
    let mut data: Vec<(String, &mut Box<dyn SourceGenerator>)> = {
        let mut data = Vec::new();
        read_dir(
            Path::new(file!())
                .parent()
                .unwrap()
                .join("../../../packages/rdom/generators"),
            "".to_string(),
            &mut data,
        );
        data
    };

    // storage for references to that data (because sourcegen is built this way)
    let mut dataref_a = data
        .iter_mut()
        .map(|(s, g)| {
            let z: Box<dyn SourceGenerator> = **g;
            let m = Box::leak(z);
            let k = &*m;
            (s.as_str(), k)
        })
        .into_iter();

    let dataref: Vec<(&str, &dyn SourceGenerator)> = dataref_a.collect::<Vec<_>>();

    let parameters = SourcegenParameters {
        generators: &dataref[..],
        ..Default::default()
    };
    run_sourcegen(&parameters).unwrap();
}
