use itertools::Itertools;


fn main() {
    let nums: Vec<usize> = include_str!("../input.txt")
        .lines()
        .map(|i| i.parse::<usize>().unwrap())
        .collect();
    //Part 1
    let mut count: usize;

    count = nums
        .iter()
        .tuple_windows()
        .map(|(a, b)| (a > b) as usize)
        .sum();

    println!("Part 1: Number of increases:{}", count);

    //Part 2
    count = nums
        .iter()
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows()
        .map(|(a, b)| (b > a) as usize)
        .sum();

    println!("Part 2: Number of increases in a window of 3:{}", count);

 //TODO: Read more on ArrayWindows feature. tuple_windows may not work beyond window size 4.
//https://doc.rust-lang.org/nightly/std/slice/struct.ArrayWindows.html

}
