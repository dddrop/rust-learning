use std::fmt::Display;

fn print_them_all<I>(iter: I)
where
    I: Iterator,
    I::Item: Display,
{
    for x in iter {
        println!("{}", x);
    }
}

fn main() {
    print_them_all(1..11);
}
