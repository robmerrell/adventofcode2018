use std::collections::HashSet;

pub fn run() {
    let data = include_str!("../data/day01.txt");

    // parse the input into a vector of ints
    let input: Vec<i32> = data.lines().map(|x| x.parse::<i32>().unwrap()).collect();

    // part 1 - find the frequency of the input by adding the numbers to 0
    println!("Frequency: {}", input.iter().sum::<i32>());

    // part 2 - continously find the frequency until we encounter a duplicate
    let mut frequency = 0;
    let mut seen = HashSet::new();
    let first_duplicate = input
        .iter()
        .cycle()
        .find_map(|x| {
            frequency += x;
            seen.replace(frequency)
        }).unwrap();

    println!("First duplicate frequency: {}", first_duplicate);
}
