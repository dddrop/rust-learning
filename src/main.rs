struct VecIter {
    vec: Vec<u32>,
    index: usize,
}

impl Iterator for VecIter {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        if self.index >= self.vec.len() {
            None
        } else {
            let res = Some(self.vec[self.index]);
            self.index += 1;
            res
        }
    }
}

fn main() {
    let fibs: Vec<u32> = vec![1, 1, 2, 3, 5, 8, 13];
    let iter = VecIter {
        vec: fibs,
        index: 0,
    };

    for x in iter {
        println!("{}", x);
    }
}
