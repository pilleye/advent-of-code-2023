use once_cell::sync::Lazy;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let first = get_number(line, 0..line.len(), false);
                let last = get_number(line, (0..line.len()).rev(), false);
                first * 10 + last
            })
            .sum(),
    )
}

static NUMBER_MAPPING: Lazy<Vec<(&str, u32)>> = Lazy::new(|| {
    vec![
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]
});

fn get_number(s: &str, range: impl Iterator<Item = usize>, allow_text: bool) -> u32 {
    range
        .map(|i| s.get(i..).unwrap())
        .map(|s| get_value(s, allow_text))
        .find(|v| v.is_some())
        .unwrap()
        .unwrap()
}

fn get_value(s: &str, allow_text: bool) -> Option<u32> {
    s.chars().next().and_then(|c| c.to_digit(10)).or_else(|| {
        allow_text
            .then(|| {
                NUMBER_MAPPING
                    .iter()
                    .find_map(|(k, v)| s.starts_with(k).then(|| *v))
            })
            .flatten()
    })
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let first = get_number(line, 0..line.len(), true);
                let last = get_number(line, (0..line.len()).rev(), true);
                first * 10 + last
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(281));
    }
}
