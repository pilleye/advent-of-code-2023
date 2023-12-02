use std::cmp::max;

advent_of_code::solution!(2);

#[derive(Debug, Clone, Copy)]
struct DiceSet {
    red: u32,
    green: u32,
    blue: u32,
}

fn parse_line(line: &str) -> Vec<DiceSet> {
    let sets = line.split(": ").collect::<Vec<_>>()[1].split(";");
    sets.map(|s| {
        let mut dice_set = DiceSet {
            red: 0,
            green: 0,
            blue: 0,
        };

        s.trim().split(",").map(|s| s.trim()).for_each(|die| {
            let split = die.split(" ").collect::<Vec<_>>();
            let (number, color) = (split[0], split[1]);
            let number = number.parse::<u32>().unwrap();
            match color {
                "red" => dice_set.red = number,
                "green" => dice_set.green = number,
                "blue" => dice_set.blue = number,
                _ => unreachable!(),
            }
        });

        dice_set
    })
    .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| parse_line(line))
            .map(|ds| {
                ds.iter()
                    .map(|d| d.red <= 12 && d.green <= 13 && d.blue <= 14)
                    .find(|valid| !valid)
                    .is_none()
            })
            .enumerate()
            .map(|(i, valid)| if valid { i as u32 + 1 } else { 0 })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| parse_line(line))
            .map(|ds| {
                ds.iter().fold(
                    DiceSet {
                        red: 0,
                        green: 0,
                        blue: 0,
                    },
                    |acc, d| DiceSet {
                        red: max(acc.red, d.red),
                        green: max(acc.green, d.green),
                        blue: max(acc.blue, d.blue),
                    },
                )
            })
            .map(|d| d.red * d.green * d.blue)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
