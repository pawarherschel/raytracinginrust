use crate::prelude::*;
use crate::remap;
use rstest::rstest;
use rstest_reuse::{apply, template};

#[rstest]
#[case(0.0, - 1.0, 1.0, 0.0, 1.0, 0.5)]
#[case(0.5_f64, - 1_f64, 1_f64, 0_f64, 1_f64, 0.75)]
fn test_remap_f64(
    #[case] a: f64,
    #[case] b: f64,
    #[case] c: f64,
    #[case] d: f64,
    #[case] e: f64,
    #[case] f: f64,
) {
    assert_eq!(remap!(value: a, from: b, c, to: d, e), f);
}

#[rstest]
#[case(color ! (0), - 1.0, 1.0, 0.0, 1.0, color ! (0.5))]
#[case(color ! (0.5), - 1_f64, 1_f64, 0_f64, 1_f64, color ! (0.75))]
fn test_remap_color(
    #[case] a: Color,
    #[case] b: f64,
    #[case] c: f64,
    #[case] d: f64,
    #[case] e: f64,
    #[case] f: Color,
) {
    assert_eq!(remap!(value: a, from: b, c, to: d, e), f);
}

// #[template]
// #[rstest]
// #[case(2, 2)]
// #[case(4 / 2, 2)]
// fn base(#[case] a: u32, #[case] b: u32) {}
//
// // Here we add a new case and an argument in a value list:
// #[apply(base)]
// #[case(9 / 3, 3)]
// fn it_works(a: u32, b: u32, #[values("a", "b")] t: &str) {
//     assert_eq!(a, b);
//     assert!("abcd".contains(t))
// }
