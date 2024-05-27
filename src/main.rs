fn tell_temperature(temp: i32) {
    let message = if temp <= 10 {
        "It's cold!"
    } else if temp <= 25 {
        "It's nice"
    } else if temp <= 30 {
        "It's warm"
    } else {
        "It's hot!"
    };

    println!("{}", message);
}

fn main() {
    tell_temperature(15);
}
