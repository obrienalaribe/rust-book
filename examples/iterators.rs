use std::ops::{Range, RangeBounds};
use std::vec::IntoIter;

fn main() {
    let mut iterator1 = (1..5).into_iter();
    iterate_all(&mut iterator1);

    let mut iterator2 = (1..10).into_iter();
    skip_items(&mut iterator2);

    let mut iterator3 = vec!["A", "B", "C", "D"].into_iter();
    enumarate_items(&mut iterator3);

    let mut iterator4 = (1..10).into_iter();
    collect_items(&mut iterator4);

}

fn iterate_all(iterator: &mut Range<i32>) {
    println!("start: {}", iterator.start);
    println!("end: {}", iterator.end);
    println!("endBound: {:?}", iterator.end_bound());
    println!("contains 5? : {:?}", iterator.contains(&5));
    println!("next : {:?}", iterator.next());
    println!("next : {:?}", iterator.next());
    println!("next : {:?}", iterator.next());
    println!("next : {:?}", iterator.next());
    println!("next : {:?}", iterator.next());
    println!("next : {:?}", iterator.next());
    println!("next : {:?}", iterator.next());
}

fn skip_items(iterator: &mut Range<i32>) {
    println!("----- From Skip Items -----");
    let mut skipped_collection = iterator.skip(2);
    println!("skip : {:?}", skipped_collection);
    println!("next : {:?}", skipped_collection.next());
    println!("next : {:?}", skipped_collection.next());
    println!("next : {:?}", skipped_collection.next());

    let t_factor = 2;
    let mut take_some = skipped_collection.take(t_factor);
    println!("--- Taking only next {} chunks---", t_factor);
    println!("next : {:?}", take_some.next());
    println!("next : {:?}", take_some.next());
    println!("next : {:?}", take_some.next());

}

fn enumarate_items(iterator: &mut IntoIter<&str>) {
    let mut enumerated = iterator.enumerate();
    println!("----- From Enumarted Items -----");
    println!("e_next : {:?}", enumerated.next());
    println!("e_next : {:?}", enumerated.next());
    println!("e_next : {:?}", enumerated.next());
    println!("e_next : {:?}", enumerated.next());
    println!("e_next : {:?}", enumerated.next());
}

fn collect_items(iterator: &mut Range<i32>) {
    println!("----- From Collect Items -----");
    let mut skipped_collection = iterator.skip(2);
    println!("skip : {:?}", skipped_collection);
    let taken = skipped_collection.take(4);
    let v: Vec<i32> = taken.collect();
    println!("Collect() in Vec {:?}", v);
}