pub fn run<T>(input: &str, _: T) -> Result<String, String>
where
    T: Iterator<Item = String>,
{
    let mut overlap_count = 0;
    input.lines().for_each(|line| {
        let mut ranges: Vec<Vec<i32>> = line
            .split(',')
            .map(|range| {
                range
                    .split('-')
                    .map(|num_str| num_str.parse::<i32>().unwrap_or(0))
                    .collect()
            })
            .collect();
        ranges.sort_by(|range1, range2| range1[0].cmp(&range2[0]));
        if ranges[0][1] >= ranges[1][0] {
            overlap_count += 1;
        }
    });

    Ok(overlap_count.to_string())
}
