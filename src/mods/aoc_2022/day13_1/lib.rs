use std::cmp::Ordering;

#[derive(Debug)]
enum Item {
    Num(usize),
    List(Vec<Item>),
}

fn create_item(input: &str) -> (Item, usize) {
    let mut vec = Vec::<Item>::new();
    let mut idx = 0;
    let bytes = input.as_bytes();
    while idx < bytes.len() {
        match bytes[idx] {
            b'[' => {
                let (new_item, idx_offset) = create_item(&input[idx + 1..]);
                idx += idx_offset;
                vec.push(new_item);
            }
            b']' => {
                return (Item::List(vec), idx + 2);
            }
            b',' => {
                idx += 1;
            }
            _ => {
                let input = &input[idx..];
                let idx_offset = input.find(|c| c == ',' || c == ']').unwrap();
                vec.push(Item::Num(input[..idx_offset].parse().unwrap()));
                idx += idx_offset;
            }
        }
    }
    (Item::List(vec), idx)
}

fn compare_pair(list1: &[Item], list2: &[Item]) -> Ordering {
    let (mut iter1, mut iter2) = (list1.iter(), list2.iter());
    loop {
        match (iter1.next(), iter2.next()) {
            (None, None) => {
                return Ordering::Equal;
            }
            (None, _) => {
                return Ordering::Less;
            }
            (_, None) => {
                return Ordering::Greater;
            }
            (Some(Item::Num(item1)), Some(Item::Num(item2))) => {
                let res = item1.cmp(item2);
                if !Ordering::is_eq(res) {
                    return res;
                }
            }
            (Some(Item::List(item1)), Some(Item::List(item2))) => {
                let result = compare_pair(item1, item2);
                if !Ordering::is_eq(result) {
                    return result;
                }
            }
            (Some(Item::Num(item1)), Some(Item::List(item2))) => {
                let v = vec![Item::Num(*item1)];
                let result = compare_pair(&v, item2);
                if !Ordering::is_eq(result) {
                    return result;
                }
            }
            (Some(Item::List(item1)), Some(Item::Num(item2))) => {
                let v = vec![Item::Num(*item2)];
                let result = compare_pair(item1, &v);
                if !Ordering::is_eq(result) {
                    return result;
                }
            }
        }
    }
}

pub fn run<T>(input: &str, _: T) -> Result<String, String>
where
    T: Iterator<Item = String>,
{
    let matching = input
        .split("\n\n")
        .map(|pair| {
            let pair: Vec<Item> = pair.split('\n').map(|expr| create_item(expr).0).collect();
            let Item::List(l1) = &pair[0] else {panic!()};
            let Item::List(l2) = &pair[1] else {panic!()};
            Ordering::is_lt(compare_pair(l1, l2))
        })
        .enumerate()
        .fold(
            0,
            |acc, (idx, correct)| if correct { acc + idx + 1 } else { acc },
        );
    Ok(matching.to_string())
}
