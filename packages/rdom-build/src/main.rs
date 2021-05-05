use behave::BehaviorGenerator;
use sourcegen_cli::{run_sourcegen, SourcegenParameters};

pub mod behave;

pub fn main() {
    let parameters = SourcegenParameters {
        generators: &[("behave", &BehaviorGenerator)],
        ..Default::default()
    };
    run_sourcegen(&parameters).unwrap();
}
