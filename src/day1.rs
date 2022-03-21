pub fn part1() -> usize {
    let raw_depths = String::from_utf8(include_bytes!("../data/day1.txt").to_vec()).unwrap();
    let depth_strings: Vec<&str> = raw_depths.split("\n").collect();

    let depths: Vec<usize> = depth_strings
        .iter()
        .map(|x| x.parse::<usize>().unwrap_or(0))
        .collect();

    let windows = depths.windows(2);

    windows.filter(|window|
        window[1] > window[0]
    ).count()
}
