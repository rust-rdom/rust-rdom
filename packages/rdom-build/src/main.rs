use behave::BehaviorGenerator;
use inject::InjectionGenerator;
use sourcegen_cli::{run_sourcegen, SourcegenParameters};

pub mod behave;
pub mod inject;

pub fn main() {
    let parameters = SourcegenParameters {
        generators: &[
            ("behave", &BehaviorGenerator),
            ("inject", &InjectionGenerator),
        ],
        ..Default::default()
    };
    run_sourcegen(&parameters).unwrap();
}
