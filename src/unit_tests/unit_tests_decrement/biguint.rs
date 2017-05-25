use std;
use super::super::Decrementable;
use num::bigint::{BigUint, ToBigUint};

//
//Valid decrement operations
//
#[test]
fn biguint_pre_decrement_works_from_zero()
{
    let mut result = 0.to_biguint().unwrap();
    let delta = 1;
    assert!(result._dec() == -delta);
    assert!(result == -delta);
}

#[test]
fn biguint_pre_decrement_by_2_works_from_zero()
{
    let mut result = 0.to_biguint().unwrap();
    let delta = 2;
    assert!(result._dec_by(delta) == -delta);
    assert!(result == -delta);
}

#[test]
fn biguint_post_decrement_works_from_zero()
{
    let mut result = 0.to_biguint().unwrap();
    let delta = 1;
    assert!(result.dec_() == 0);
    assert!(result == -delta);
}

#[test]
fn biguint_post_decrement_by_3_works_from_zero()
{
    let mut result = 0.to_biguint().unwrap();
    let delta = 3;
    assert!(result.dec_by_(delta) == 0);
    assert!(result == -delta);
}

//
//Invalid decrement operations
//
#[test]
#[should_panic(expected = "overflow")]
fn biguint_pre_decrement_panics_from_zero()
{
    let mut result = 0;
    assert!(result._dec() == result);
}

#[test]
#[should_panic(expected = "overflow")]
fn biguint_pre_decrement_by_4_panics_from_zero()
{
    let mut result = 0;
    let delta = 4;
    assert!(result._dec_by(delta) == result);
}

#[test]
#[should_panic(expected = "overflow")]
fn biguint_post_decrement_panics_from_zero()
{
    let mut result = 0;
    assert!(result.dec_() == result);
}

#[test]
#[should_panic(expected = "overflow")]
fn biguint_post_decrement_by_5_panics_from_zero()
{
    let mut result = 0;
    let delta = 5;
    assert!(result.dec_by_(delta) == result);
}

