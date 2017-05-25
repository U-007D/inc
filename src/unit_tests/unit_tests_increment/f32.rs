use std;
use super::super::Increment;

//
//Valid increment operations
//
#[test]
fn f32_pre_increment_works_from_min()
{
    let mut result = std::f32::MIN;
    let delta = 1.0;
    assert!(result._inc() == std::f32::MIN + delta);
    assert!(result == std::f32::MIN + delta);
}

#[test]
fn f32_pre_increment_by_2_works_from_min()
{
    let mut result = std::f32::MIN;
    let delta = 2.0;
    assert!(result._inc_by(delta) == std::f32::MIN + delta);
    assert!(result == std::f32::MIN + delta);
}

#[test]
fn f32_post_increment_works_from_min()
{
    let mut result = std::f32::MIN;
    let delta = 1.0;
    assert!(result.inc_() == std::f32::MIN);
    assert!(result == std::f32::MIN + delta);
}

#[test]
fn f32_post_increment_by_3_works_from_min()
{
    let mut result = std::f32::MIN;
    let delta = 3.0;
    assert!(result.inc_by_(delta) == std::f32::MIN);
    assert!(result == std::f32::MIN + delta);
}
