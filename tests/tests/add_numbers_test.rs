use tests::add_numbers;

#[test]
fn test_add_positive_numbers() {
    assert_eq!(add_numbers(2, 3), 5);
}

#[test]
fn test_add_negative_numbers() {
    assert_eq!(add_numbers(-2, -3), -5);
}

#[test]
fn test_add_zero() {
    assert_eq!(add_numbers(0, 0), 0);
}
