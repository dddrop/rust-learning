fn main() {
    // Loop over the high values running from 10 to 20
    let mut high = 10;
    while high <= 20 {
        // Each time we have a new high value,
        // we start a new sum. Start the total
        // at 0, and start adding from 1.
        let mut i = 1;
        let mut total = 0;

        // Now, get all the numbers from 1 to
        // the high number and add them to total.
        while i <= high {
            total += i;
            i += 1;
        }

        // Print out the result...
        println!("sum(1, {}) == {}", high, total);

        // And go on to the next high
        high += 1;
    }
}
