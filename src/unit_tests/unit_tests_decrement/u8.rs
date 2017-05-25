use std;
use super::super::Decrement;

//
//Invalid decrement operations
//
#[test]
#[should_panic(expected = "overflow")]
fn u8_pre_decrement_panics_from_min()
{
    let mut result = std::u8::MIN;
    assert!(result._dec() == result);
}

#[test]
#[should_panic(expected = "overflow")]
fn u8_pre_decrement_by_4_panics_from_min()
{
    let mut result = std::u8::MIN;
    let delta = 4;
    assert!(result._dec_by(delta) == result);
}

#[test]
#[should_panic(expected = "overflow")]
fn u8_post_decrement_panics_from_min()
{
    let mut result = std::u8::MIN;
    assert!(result.dec_() == result);
}

#[test]
#[should_panic(expected = "overflow")]
fn u8_post_decrement_by_5_panics_from_min()
{
    let mut result = std::u8::MIN;
    let delta = 5;
    assert!(result.dec_by_(delta) == result);
}

