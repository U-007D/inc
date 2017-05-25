use std;
use super::super::Decrement;

//
//Valid decrement operations
//
#[test]
fn usize_pre_decrement_works_from_max()
{
    let mut result = std::usize::MAX;
    let delta = 1;
    assert!(result._dec() == std::usize::MAX - delta);
    assert!(result == std::usize::MAX - delta);
}

#[test]
fn usize_pre_decrement_by_2_works_from_max()
{
    let mut result = std::usize::MAX;
    let delta = 2;
    assert!(result._dec_by(delta) == std::usize::MAX - delta);
    assert!(result == std::usize::MAX - delta);
}

#[test]
fn usize_post_decrement_works_from_max()
{
    let mut result = std::usize::MAX;
    let delta = 1;
    assert!(result.dec_() == std::usize::MAX);
    assert!(result == std::usize::MAX - delta);
}

#[test]
fn usize_post_decrement_by_3_works_from_max()
{
    let mut result = std::usize::MAX;
    let delta = 3;
    assert!(result.dec_by_(delta) == std::usize::MAX);
    assert!(result == std::usize::MAX - delta);
}

//
//Invalid decrement operations
//
#[test]
#[should_panic(expected = "overflow")]
fn usize_pre_decrement_panics_from_min()
{
    let mut result = std::usize::MIN;
    assert!(result._dec() == result);
}

#[test]
#[should_panic(expected = "overflow")]
fn usize_pre_decrement_by_4_panics_from_min()
{
    let mut result = std::usize::MIN;
    let delta = 4;
    assert!(result._dec_by(delta) == result);
}

#[test]
#[should_panic(expected = "overflow")]
fn usize_post_decrement_panics_from_min()
{
    let mut result = std::usize::MIN;
    assert!(result.dec_() == result);
}

#[test]
#[should_panic(expected = "overflow")]
fn usize_post_decrement_by_5_panics_from_min()
{
    let mut result = std::usize::MIN;
    let delta = 5;
    assert!(result.dec_by_(delta) == result);
}

