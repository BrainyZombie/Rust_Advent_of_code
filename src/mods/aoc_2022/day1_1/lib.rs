pub fn run<T>(input: &str, _: T) -> Result<String, String>
where
    T: Iterator<Item = String>,
{
    let mut curr_sum = 0;
    let max_calories = input
        .lines()
        .map(|l| match l.parse::<i32>() {
            Ok(i) => Some(i),
            Err(_) => None,
        })
        .filter_map(|num| match num {
            Some(n) => {
                curr_sum += n;
                None
            }
            None => {
                let a = Some(curr_sum);
                curr_sum = 0;
                a
            }
        })
        .max();
    match max_calories {
        Some(max_calories) => Ok(max_calories.to_string()),
        None => Err(String::from("No max calories returned")),
    }
}
