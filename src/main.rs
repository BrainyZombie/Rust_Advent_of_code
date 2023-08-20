mod mods;
use mods::mains;
use std::env;

fn get_fn_number<T>(args: &mut Box<T>) -> Result<usize, String>
where
    T: Iterator<Item = String>,
{
    args.next()
        .map_or(Err(String::from("Specify function number to run")), |s| {
            s.parse::<usize>()
                .map_err(|e| format!("Unable to parse integer argument: {e}"))
        })
        .map_or_else(Err, |i| {
            if i >= 100 {
                Err(String::from("The number must be between 0 and 99."))
            } else {
                Ok(i)
            }
        })
}

fn main() {
    let mut args = Box::from(env::args());
    args.next();
    let i = match get_fn_number(&mut args) {
        Ok(i) => i,
        Err(str) => {
            println!("{str}");
            return;
        }
    };

    (mains[i])(args);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_args() {
        let mut args = Box::from(vec![].into_iter());
        let result = get_fn_number(&mut args);
        assert_eq!(result, Err(String::from("Specify function number to run")));
    }
    #[test]
    fn invalid_int_1() {
        let mut args = Box::from(vec![String::from("a")].into_iter());
        let result = get_fn_number(&mut args);
        assert!(result.is_err_and(|s| s.starts_with("Unable to parse integer argument")));
    }
    #[test]
    fn invalid_int_2() {
        let mut args = Box::from(vec![String::from("1a")].into_iter());
        let result = get_fn_number(&mut args);
        assert!(result.is_err_and(|s| s.starts_with("Unable to parse integer argument")));
    }
    #[test]
    fn invalid_int_4() {
        let mut args = Box::from(vec![String::from("100")].into_iter());
        let result = get_fn_number(&mut args);
        assert_eq!(
            result,
            Err(String::from("The number must be between 0 and 99."))
        );
    }
    #[test]
    fn valid_int_1() {
        let mut args = Box::from(vec![String::from("99")].into_iter());
        let result = get_fn_number(&mut args);
        assert_eq!(result, Ok(99));
    }
    #[test]
    fn valid_int_2() {
        let mut args = Box::from(vec![String::from("2")].into_iter());
        let result = get_fn_number(&mut args);
        assert_eq!(result, Ok(2));
    }
}
