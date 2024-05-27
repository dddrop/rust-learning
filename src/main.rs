fn main() {
    let x = true;
    let y = !x; // not x
    let z = !y; // not y

    // All three of the following do the same thing
    assert!(z == true);
    assert_eq!(z, true);
    assert!(z);

    // Same for these three
    assert!(y == false);
    assert_eq!(y, false);
    assert!(!y);

    println!("Woohoo!");
}
