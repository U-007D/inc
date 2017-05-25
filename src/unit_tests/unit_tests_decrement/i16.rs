use std;
use super::super::Decrement;

//
//Valid decrement operations
//
#[test]
fn i16_pre_decrement_works_from_max()
{
    let mut result = std::i16::MAX;
    let delta = 1;
    assert!(result._dec() == std::i16::MAX - delta);
    assert!(result == std::i16::MAX - delta);
}

#[test]
fn i16_pre_decrement_by_2_works_from_max()
{
    let mut result = std::i16::MAX;
    let delta = 2;
    assert!(result._dec_by(delta) == std::i16::MAX - delta);
    assert!(result == std::i16::MAX - delta);
}

#[test]
fn i16_post_decrement_works_from_max()
{
    let mut result = std::i16::MAX;
    let delta = 1;
    assert!(result.dec_() == std::i16::MAX);
    assert!(result == std::i16::MAX - delta);
}

#[test]
fn i16_post_decrement_by_3_works_from_max()
{
    let mut result = std::i16::MAX;
    let delta = 3;
    assert!(result.dec_by_(delta) == std::i16::MAX);
    assert!(result == std::i16::MAX - delta);
}

//
//Invalid decrement operations
//
#[test]
#[should_panic(expected = "overflow")]
fn i16_pre_decrement_panics_from_min()
{
    let mut result = std::i16::MIN;
    assert!(result._dec() == result);
}

#[test]
#[should_panic(expected = "overflow")]
fn i16_pre_decrement_by_4_panics_from_min()
{
    let mut result = std::i16::MIN;
    let delta = 4;
    assert!(result._dec_by(delta) == result);
}

#[test]
#[should_panic(expected = "overflow")]
fn i16_post_decrement_panics_from_min()
{
    let mut result = std::i16::MIN;
    assert!(result.dec_() == result);
}

#[test]
#[should_panic(expected = "overflow")]
fn i16_post_decrement_by_5_panics_from_min()
{
    let mut result = std::i16::MIN;
    let delta = 5;
    assert!(result.dec_by_(delta) == result);
}

