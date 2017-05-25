use std;
use super::super::Decrement;

//
//Valid decrement operations
//
#[test]
fn f64_pre_decrement_works_from_max()
{
    let mut result = std::f64::MAX;
    let delta = 1.0;
    assert!(result._dec() == std::f64::MAX - delta);
    assert!(result == std::f64::MAX - delta);
}

#[test]
fn f64_pre_decrement_by_2_works_from_max()
{
    let mut result = std::f64::MAX;
    let delta = 2.0;
    assert!(result._dec_by(delta) == std::f64::MAX - delta);
    assert!(result == std::f64::MAX - delta);
}

#[test]
fn f64_post_decrement_works_from_max()
{
    let mut result = std::f64::MAX;
    let delta = 1.0;
    assert!(result.dec_() == std::f64::MAX);
    assert!(result == std::f64::MAX - delta);
}

#[test]
fn f64_post_decrement_by_3_works_from_max()
{
    let mut result = std::f64::MAX;
    let delta = 3.0;
    assert!(result.dec_by_(delta) == std::f64::MAX);
    assert!(result == std::f64::MAX - delta);
}

