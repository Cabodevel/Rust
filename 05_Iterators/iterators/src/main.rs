fn main() {
    let s = [1, 2, 3];

    for x in s.iter(){
        println!("{}", x+1);
    }

    let mut counter = Counter::new();

    let next = counter.next();
    
    println!("{}", next.unwrap_or_default());
}

pub struct Counter{
    count:i32
}

impl Counter {
    fn new() -> Counter {
        Counter{
            count: 0
        }
    }
}

impl Iterator for Counter{
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        Some(self.count)
    }
}

