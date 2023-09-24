use std::collections::HashSet;
use uuid::Uuid;

fn main() {
    

    let sum = sum_one;

    println!("{}", sum(2));

    let sum_closure = |n:i32| -> i32 {
        n + 1
    };

    println!("{}", sum_closure(5));

    let mut hs:HashSet<Uuid> = HashSet::new();
    hs.insert(Uuid::new_v4());
    hs.insert(Uuid::new_v4());

    let mut hsbck:HashSet<Uuid> = HashSet::new();
    hsbck.insert(Uuid::new_v4());
    hsbck.insert(Uuid::new_v4());

    for guid in hs.union(&hsbck) {
        println!("{}", guid)
    }
}

fn sum_one(nmb: i32) -> i32 {
    nmb + 1
}