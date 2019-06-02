use writing_tests;

mod common;

#[test]
fn int_testing_for_super_heavy_calcs() {
    common::setup();

    let mut num = 10;

    num = writing_tests::really_heavy_calc(num);
    num = writing_tests::super_heavy_calc(num);
    num = writing_tests::extreamly_heavy_calc(num);
    assert_eq!(num, 20)
}
