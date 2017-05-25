use std;
use super::super::Increment;

//
//Valid increment operations
//
#[test]
fn usize_pre_increment_works_from_min()
{
    let mut result = std::usize::MIN;
    let delta = 1;
    assert!(result._inc() == std::usize::MIN + delta);
    assert!(result == std::usize::MIN + delta);
}

#[test]
fn usize_pre_increment_by_2_works_from_min()
{
    let mut result = std::usize::MIN;
    let delta = 2;
    assert!(result._inc_by(delta) == std::usize::MIN + delta);
    assert!(result == std::usize::MIN + delta);
}

#[test]
fn usize_post_increment_works_from_min()
{
    let mut result = std::usize::MIN;
    let delta = 1;
    assert!(result.inc_() == std::usize::MIN);
    assert!(result == std::usize::MIN + delta);
}

#[test]
fn usize_post_increment_by_3_works_from_min()
{
    let mut result = std::usize::MIN;
    let delta = 3;
    assert!(result.inc_by_(delta) == std::usize::MIN);
    assert!(result == std::usize::MIN + delta);
}

//
//Invalid increment operations
//
#[test]
#[should_panic(expected = "overflow")]
fn usize_pre_increment_panics_from_max()
{
    let mut result = std::usize::MAX;
    assert!(result._inc() == result);
}

#[test]
#[should_panic(expected = "overflow")]
fn usize_pre_increment_by_4_panics_from_max()
{
    let mut result = std::usize::MAX;
    let delta = 4;
    assert!(result._inc_by(delta) == result);
}

#[test]
#[should_panic(expected = "overflow")]
fn usize_post_increment_panics_from_max()
{
    let mut result = std::usize::MAX;
    assert!(result.inc_() == result);
}

#[test]
#[should_panic(expected = "overflow")]
fn usize_post_increment_by_5_panics_from_max()
{
    let mut result = std::usize::MAX;
    let delta = 5;
    assert!(result.inc_by_(delta) == result);
}
