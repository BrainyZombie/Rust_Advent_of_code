pub fn run<T>(input: &str, _: T) -> Result<String, String>
where
    T: Iterator<Item = String>,
{
    let mut overlap_count = 0;
    input.lines().for_each(|line| {
        let ranges: Vec<Vec<i32>> = line
            .split(',')
            .map(|range| {
                range
                    .split('-')
                    .map(|num_str| num_str.parse::<i32>().unwrap_or(0))
                    .collect()
            })
            .collect();
        if (ranges[0][0] >= ranges[1][0] && ranges[0][1] <= ranges[1][1])
            || (ranges[1][0] >= ranges[0][0] && ranges[1][1] <= ranges[0][1])
        {
            overlap_count += 1;
        }
    });

    Ok(overlap_count.to_string())
}
