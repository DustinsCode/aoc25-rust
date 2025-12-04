advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let ranges = input.trim_end().split(',').collect::<Vec<&str>>();

    let mut total: u64 = 0;

    for range_str in ranges {
        let (start_str, end_str) = range_str.split_once('-').unwrap();

        let start: u64 = start_str.parse().unwrap();
        let end: u64 = end_str.parse().unwrap();

        for i in start..end + 1 {
            let i_str = i.to_string();

            if (i_str.len() % 2) != 0 {
                continue;
            }

            let (left, right) = i_str.split_at(i_str.len() / 2);

            if left.eq(right) {
                total += i;
            }
        }
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let ranges = input.trim_end().split(',').collect::<Vec<&str>>();

    let mut total: u64 = 0;

    for range_str in ranges {
        let (start_str, end_str) = range_str.split_once('-').unwrap();

        let start: u64 = start_str.parse().unwrap();
        let end: u64 = end_str.parse().unwrap();

        for i in start..end + 1 {
            let i_str = i.to_string();

            if (i_str.len() % 2) != 0 {
                continue;
            }

            let (left, right) = i_str.split_at(i_str.len() / 2);

            if left.eq(right) {
                total += i;
            }
        }
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
