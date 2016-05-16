#[test]
fn function_without_doing() {
}

#[test]
fn wrong_assert() {
    assert!(false);
}

#[test]
#[should_panic]
fn assert_should_panic() {
    assert!(false);
}

#[test]
#[should_panic(expected = "assertion failed")]
fn expected_panic_failed_assertion() {
    assert_eq!("Hello", "world");
}

#[test]
#[ignore]
fn ignored_test() {
    assert_eq!("Hello", "world");
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod test_uses_non_testing_function {
    use super::add_two;

    #[test]
    fn it_works() {
        assert_eq!(4, add_two(2));
    }
}
