fn main() {
    // These two things are basically the same
    assert!("apple" != "banana");
    assert_ne!("apple", "banana");

    assert!(5 == 5);
    assert_eq!(5, 5);

    assert!(() == ());

    // Oh, did I mention that we can use true and false like this?
    // They're also literals!
    assert!(true == true);
    assert!(true != false);

    assert!(6 != 3);
    assert!(6 > 3);
    assert!(3 < 6);
    assert!(6 >= 3);

    println!("Success!");
}