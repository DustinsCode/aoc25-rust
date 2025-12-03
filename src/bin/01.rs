advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let turns = input.lines().collect::<Vec<&str>>();

    let mut dial: i32 = 50;
    let mut hit_zero: u64 = 0;

    for turn in turns {
        let mut new_turn = String::from(turn);
        let direction = new_turn.remove(0);
        let count = new_turn.parse::<i32>().unwrap();

        if direction == 'R' {
            dial += count;
        } else {
            dial -= count;
        }

        dial %= 100;
        if dial == 0 {
            hit_zero += 1;
        }
    }

    Some(hit_zero)
}

pub fn part_two(input: &str) -> Option<u64> {
    let turns = input.lines().collect::<Vec<&str>>();

    let mut dial: i32 = 50;
    let mut hit_zero = 0;

    for turn in turns {
        let mut new_turn = String::from(turn);
        let direction = new_turn.remove(0);
        let mut count = new_turn.parse::<i32>().unwrap();

        if direction == 'L' {
            count = -count;
        }
        if dial == 0 && count < 0 {
            hit_zero -= 1;
        }

        dial += count;
        hit_zero += dial.div_euclid(100).abs();
        dial = dial.rem_euclid(100);

        if dial == 0 && count < 0 {
            hit_zero += 1;
        }
    }

    Some(hit_zero as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
