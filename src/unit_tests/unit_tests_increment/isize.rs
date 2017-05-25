use std;
use super::super::Increment;
//
//Valid increment operations
//
#[test]
fn isize_pre_increment_works_from_min()
{
    let mut result = std::isize::MIN;
    let delta = 1;
    assert!(result._inc() == std::isize::MIN + delta);
    assert!(result == std::isize::MIN + delta);
}

#[test]
fn isize_pre_increment_by_2_works_from_min()
{
    let mut result = std::isize::MIN;
    let delta = 2;
    assert!(result._inc_by(delta) == std::isize::MIN + delta);
    assert!(result == std::isize::MIN + delta);
}

#[test]
fn isize_post_increment_works_from_min()
{
    let mut result = std::isize::MIN;
    let delta = 1;
    assert!(result.inc_() == std::isize::MIN);
    assert!(result == std::isize::MIN + delta);
}

#[test]
fn isize_post_increment_by_3_works_from_min()
{
    let mut result = std::isize::MIN;
    let delta = 3;
    assert!(result.inc_by_(delta) == std::isize::MIN);
    assert!(result == std::isize::MIN + delta);
}

//
//Invalid increment operations
//
#[test]
#[should_panic(expected = "overflow")]
fn isize_pre_increment_panics_from_max()
{
    let mut result = std::isize::MAX;
    assert!(result._inc() == result);
}

#[test]
#[should_panic(expected = "overflow")]
fn isize_pre_increment_by_4_panics_from_max()
{
    let mut result = std::isize::MAX;
    let delta = 4;
    assert!(result._inc_by(delta) == result);
}

#[test]
#[should_panic(expected = "overflow")]
fn isize_post_increment_panics_from_max()
{
    let mut result = std::isize::MAX;
    assert!(result.inc_() == result);
}

#[test]
#[should_panic(expected = "overflow")]
fn isize_post_increment_by_5_panics_from_max()
{
    let mut result = std::isize::MAX;
    let delta = 5;
    assert!(result.inc_by_(delta) == result);
}
