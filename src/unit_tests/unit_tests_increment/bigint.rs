use std;
use super::super::Incrementable;
use num::bigint::{ToBigInt};

//
//Valid increment operations
//
#[test]
fn bigint_pre_increment_works_from_zero()
{
    let mut result = 0.to_bigint().unwrap();
    let delta = 1;
    assert!(result._inc() == delta);
    assert!(result == delta);
}

#[test]
fn bigint_pre_increment_by_2_works_from_zero()
{
    let mut result = 0.to_bigint().unwrap();
    let delta = 2;
    assert!(result._inc_by(delta) == delta);
    assert!(result == delta);
}

#[test]
fn bigint_post_increment_works_from_zero()
{
    let mut result = 0.to_bigint().unwrap();
    let delta = 1;
    assert!(result.inc_() == 0);
    assert!(result == delta);
}

#[test]
fn bigint_post_increment_by_3_works_from_zero()
{
    let mut result = 0.to_bigint().unwrap();
    let delta = 3;
    assert!(result.inc_by_(delta) == 0);
    assert!(result == delta);
}
