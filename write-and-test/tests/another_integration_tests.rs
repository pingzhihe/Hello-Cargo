use write_and_test;

#[test]
fn it_really_adds_two() {
    assert_eq!(5, write_and_test::add_two(3));
}