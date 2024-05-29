fn main() {
    let mut fibs = vec![1, 1, 2, 3, 5, 8, 13];

    for x in fibs.iter_mut() {
        println!("{}", x);
        *x *= 2;
    }

    println!("{:?}", fibs);
}
