use std;
use super::super::Decrement;

//
//Valid decrement operations
//
#[test]
fn f32_pre_decrement_works_from_max()
{
    let mut result = std::f32::MAX;
    let delta = 1.0;
    assert!(result._dec() == std::f32::MAX - delta);
    assert!(result == std::f32::MAX - delta);
}

#[test]
fn f32_pre_decrement_by_2_works_from_max()
{
    let mut result = std::f32::MAX;
    let delta = 2.0;
    assert!(result._dec_by(delta) == std::f32::MAX - delta);
    assert!(result == std::f32::MAX - delta);
}

#[test]
fn f32_post_decrement_works_from_max()
{
    let mut result = std::f32::MAX;
    let delta = 1.0;
    assert!(result.dec_() == std::f32::MAX);
    assert!(result == std::f32::MAX - delta);
}

#[test]
fn f32_post_decrement_by_3_works_from_max()
{
    let mut result = std::f32::MAX;
    let delta = 3.0;
    assert!(result.dec_by_(delta) == std::f32::MAX);
    assert!(result == std::f32::MAX - delta);
}

