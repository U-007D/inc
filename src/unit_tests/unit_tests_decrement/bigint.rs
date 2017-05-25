use std;
use super::super::Decrementable;
use num::bigint::{ToBigInt};

//
//Valid decrement operations
//
#[test]
fn bigint_pre_decrement_works_from_zero()
{
    let mut result = 0.to_bigint().unwrap();
    let delta = 1;
    assert!(result._dec() == -delta);
    assert!(result == -delta);
}

#[test]
fn bigint_pre_decrement_by_2_works_from_zero()
{
    let mut result = 0.to_bigint().unwrap();
    let delta = 2;
    assert!(result._dec_by(delta) == -delta);
    assert!(result == -delta);
}

#[test]
fn bigint_post_decrement_works_from_zero()
{
    let mut result = 0.to_bigint().unwrap();
    let delta = 1;
    assert!(result.dec_() == 0);
    assert!(result == -delta);
}

#[test]
fn bigint_post_decrement_by_3_works_from_zero()
{
    let mut result = 0.to_bigint().unwrap();
    let delta = 3;
    assert!(result.dec_by_(delta) == 0);
    assert!(result == -delta);
}
