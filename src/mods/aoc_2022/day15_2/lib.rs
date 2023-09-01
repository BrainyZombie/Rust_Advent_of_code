use std::{collections::HashSet, ops::Sub};

enum MergeInfo {
    ShiftAndTruncate { from: usize, to: usize },
    SortedInsert(usize),
}

fn intersection(range1: &(isize, isize), range2: &(isize, isize)) -> Option<(isize, isize)> {
    if range1.0 <= range2.1 && range2.0 <= range1.1 {
        Some((range1.0.min(range2.0), range1.1.max(range2.1)))
    } else {
        None
    }
}

fn add_ranges(ranges: &mut Vec<(isize, isize)>, range: (isize, isize)) {
    let mut range_to_add: (isize, isize) = range;
    let mut insert_idx: usize = 0;
    let mut idx = 0;

    let merge_info = loop {
        if idx == ranges.len() || ranges[idx].0 > range_to_add.1 {
            if insert_idx != idx {
                ranges[insert_idx] = range_to_add;
                break MergeInfo::ShiftAndTruncate {
                    from: insert_idx + 1,
                    to: idx,
                };
            }

            break MergeInfo::SortedInsert(idx);
        }
        if let Some(new_range) = intersection(&range_to_add, &ranges[idx]) {
            range_to_add = new_range;
        } else {
            insert_idx += 1;
        }
        idx += 1;
    };

    match merge_info {
        MergeInfo::ShiftAndTruncate { from, to } => {
            for idx in to..ranges.len() {
                ranges[idx + from - to] = ranges[idx];
            }
            ranges.truncate(ranges.len() + from - to);
        }
        MergeInfo::SortedInsert(idx) => {
            ranges.insert(idx, range_to_add);
        }
    }
}

pub fn run<T>(input: &str, _: T) -> Result<String, String>
where
    T: Iterator<Item = String>,
{
    for target_y in 0..4000000 {
        let mut ranges: Vec<(isize, isize)> = vec![];
        let mut existing_beacons: HashSet<isize> = HashSet::new();

        input.lines().for_each(|line| {
            let comma = line.find(',').unwrap();
            let colon = line.find(':').unwrap();
            let x_sensor: isize = line[12..comma].parse().unwrap();
            let y_sensor: isize = line[comma + 4..colon].parse().unwrap();
            let comma = line[colon + 25..].find(',').unwrap() + colon + 25;
            let x_beacon: isize = line[colon + 25..comma].parse().unwrap();
            let y_beacon: isize = line[comma + 4..].parse().unwrap();

            let manhattan_distance = x_sensor.sub(x_beacon).abs() + y_sensor.sub(y_beacon).abs();
            let used_distance = y_sensor.sub(target_y).abs();
            let available_distance = manhattan_distance - used_distance;

            if available_distance >= 0 {
                let edge1 = x_sensor + available_distance;
                let edge2 = x_sensor - available_distance;
                let range = (edge1.min(edge2), edge1.max(edge2));
                add_ranges(&mut ranges, range);
            }
            if y_beacon == target_y {
                existing_beacons.insert(x_beacon);
            }
        });

        if ranges.len() != 1 {
            let result = (ranges[1].0 - 1) * 4000000 + target_y;
            return Ok(result.to_string());
        }
    }
    panic!();
}
