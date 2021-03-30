#![cfg(test)]

use crate::config::ScreenMetrics;
use crate::sandbox::Sandbox;

#[test]
fn it_works() {
    let metrics: ScreenMetrics = Default::default();
    let sbox = Sandbox::new(metrics);
    let doc = sbox.window().document();

    assert_eq!(2 + 2, 4);
}
