fn main ()
{
    let mut xs = [90, 15, 7, 53, 21, 100, 1];
    let mut nums = vec![90, 15, 7, 53, 21, 100, 1];
    let mut name = ["T", "h", "i", "b", "a", "u", "l", "t"];

    insertion_sort(&mut nums);
    insertion_sort(&mut xs);
    insertion_sort(&mut name);

    println!("{:?}", nums);
    println!("{:?}", xs);
    println!("{:?}", name);
}

// O(n^2)
fn insertion_sort<T: PartialOrd>(arr: &mut [T])
{
    // When doing the let (mut i, j) format
    // j is not mutable
    let (mut j, size) = (0, arr.len());
    while j < size
    {
        let mut i = j;
        while i > 0 && arr[i - 1] > arr[i]
        {
            arr.swap(i, i - 1);
            i = i - 1;
        }
    j += 1;
    }
}