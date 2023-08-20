pub fn run<T>(input: &str, _: T) -> Result<String, String>
where
    T: Iterator<Item = String>,
{
    let mut curr_sum = 0;
    let mut calorieList: Vec<i32> = input
        .lines()
        .map(|l| l.parse::<i32>().ok())
        .filter_map(|num| match num {
            Some(n) => {
                curr_sum += n;
                None
            }
            None => {
                let res = Some(curr_sum);
                curr_sum = 0;
                res
            }
        })
        .collect();
    calorieList.sort_unstable();
    Ok(calorieList.iter().rev().take(3).sum::<i32>().to_string())
}
