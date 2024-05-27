fn main() {
    let x = 5;
    {
        let mut x = 6;
        assert_eq!(x, 6);
        x += 1;
        assert_eq!(x, 7);
        x = 8;
        assert_eq!(x, 8);
    }
    assert_eq!(x, 5);
}