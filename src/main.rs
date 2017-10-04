extern crate sorting;
use sorting::sorting::*;

fn main ()
{
    let mut xs = [90, 15, 7, 53, 21, 100, 1];
    let mut nums = vec![90, 15, 7, 53, 21, 100, 1];
    let mut name = ["T", "h", "i", "b", "a", "u", "l", "t"];

    // insertion_sort(&mut nums);
    // insertion_sort(&mut xs);
    insertion_sort(&mut name);

    heap_sort(&mut xs);

    println!("{:?}", nums);
    bubble_sort(&mut nums);

    println!("{:?}", nums);
    // println!("{:?}", xs);
    // println!("{:?}", name);
}