use super::super::Incrementable;

//
//Valid increment operations
//
#[test]
fn i128_pre_increment_works_from_min()
{
    let mut result = std::i128::MIN;
    let delta = 1;
    assert!(result._inc() == std::i128::MIN + delta);
    assert!(result == std::i128::MIN + delta);
}

#[test]
fn i128_pre_increment_by_2_works_from_min()
{
    let mut result = std::i128::MIN;
    let delta = 2;
    assert!(result._inc_by(delta) == std::i128::MIN + delta);
    assert!(result == std::i128::MIN + delta);
}

#[test]
fn i128_post_increment_works_from_min()
{
    let mut result = std::i128::MIN;
    let delta = 1;
    assert!(result.inc_() == std::i128::MIN);
    assert!(result == std::i128::MIN + delta);
}

#[test]
fn i128_post_increment_by_3_works_from_min()
{
    let mut result = std::i128::MIN;
    let delta = 3;
    assert!(result.inc_by_(delta) == std::i128::MIN);
    assert!(result == std::i128::MIN + delta);
}

//
//Invalid increment operations
//
#[test]
#[should_panic(expected = "overflow")]
fn i128_pre_increment_panics_from_max()
{
    let mut result = std::i128::MAX;
    assert!(result._inc() == result);
}

#[test]
#[should_panic(expected = "overflow")]
fn i128_pre_increment_by_4_panics_from_max()
{
    let mut result = std::i128::MAX;
    let delta = 4;
    assert!(result._inc_by(delta) == result);
}

#[test]
#[should_panic(expected = "overflow")]
fn i128_post_increment_panics_from_max()
{
    let mut result = std::i128::MAX;
    assert!(result.inc_() == result);
}

#[test]
#[should_panic(expected = "overflow")]
fn i128_post_increment_by_5_panics_from_max()
{
    let mut result = std::i128::MAX;
    let delta = 5;
    assert!(result.inc_by_(delta) == result);
}
