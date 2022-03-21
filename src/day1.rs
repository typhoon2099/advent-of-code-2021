pub fn part1() -> usize {
    let raw_depths = String::from_utf8(include_bytes!("../data/day1.txt").to_vec()).unwrap();
    let depth_strings: Vec<&str> = raw_depths.split("\n").collect();

    println!("Test");

    let depths: Vec<usize> = depth_strings
        .iter()
        .map(|x| x.parse::<usize>().unwrap_or(0))
        .collect();

    let mut increases = 0;

    let length = depths.len();

    for position in 1..length {
        if depths[position] > depths[position - 1] {
            increases += 1;
        }
    }

    increases
}
