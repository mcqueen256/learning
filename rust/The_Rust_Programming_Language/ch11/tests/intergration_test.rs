use ch11;

#[test]
fn it_adds_two_dude() {
    let a = ch11::add_two(40);
    assert_eq!(a, 42);
}
