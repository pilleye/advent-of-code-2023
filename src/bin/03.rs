advent_of_code::solution!(3);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Data {
    Symbol(char),
    Number(u32),
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum VisitStatus {
    /// Currently visiting a number, the number is the x-coordinate of the start and the current value of the number
    Visiting(usize, u32),

    /// Not visiting anythng
    Empty,
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = get_map(input);

    Some(
        map.iter()
            .enumerate()
            .map(|(y, line)| traverse_line(y, line, &map))
            .sum(),
    )
}

fn traverse_line(y: usize, line: &Vec<Data>, map: &Vec<Vec<Data>>) -> u32 {
    line.iter()
        .enumerate()
        .fold((0, VisitStatus::Empty), |(sum, status), (x, data)| {
            match (status, data) {
                (VisitStatus::Visiting(start_x, visit_sum), Data::None | Data::Symbol(..)) => {
                    for x in start_x - 1..x + 1 {
                        for y in y - 1..y + 2 {
                            if matches!(map[y][x], Data::Symbol(..)) {
                                return (sum + visit_sum, VisitStatus::Empty);
                            }
                        }
                    }

                    (sum, VisitStatus::Empty)
                }
                (VisitStatus::Visiting(start_x, visit_sum), Data::Number(u)) => {
                    (sum, VisitStatus::Visiting(start_x, visit_sum * 10 + u))
                }
                (VisitStatus::Empty, Data::Number(n)) => (sum, VisitStatus::Visiting(x, *n)),
                (VisitStatus::Empty, Data::None | Data::Symbol(..)) => (sum, VisitStatus::Empty),
            }
        })
        .0
}

/// Returns a map surrounded by empty spacing to make it easier to traverse
fn get_map(input: &str) -> Vec<Vec<Data>> {
    let map: Vec<Vec<Data>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match (c, c.to_digit(10)) {
                    (_, Some(c)) => Data::Number(c),
                    ('.', _) => Data::None,
                    (c, _) => Data::Symbol(c),
                })
                .collect()
        })
        .collect();

    let mut big_map = vec![vec![Data::None; map[0].len() + 2]; map.len() + 2];
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            big_map[y + 1][x + 1] = map[y][x];
        }
    }

    big_map
}

fn traverse_line_gear(
    y: usize,
    line: &Vec<Data>,
    map: &Vec<Vec<Data>>,
    gears: &mut Vec<Vec<Option<Vec<u32>>>>,
) {
    line.iter()
        .enumerate()
        .fold(VisitStatus::Empty, |status, (x, data)| {
            match (status, data) {
                (VisitStatus::Visiting(start_x, visit_sum), Data::None | Data::Symbol(..)) => {
                    for x in start_x - 1..x + 1 {
                        for y in y - 1..y + 2 {
                            if map[y][x] == Data::Symbol('*') {
                                gears[y][x].get_or_insert_with(|| vec![]).push(visit_sum);
                            }
                        }
                    }

                    VisitStatus::Empty
                }
                (VisitStatus::Visiting(start_x, visit_sum), Data::Number(u)) => {
                    VisitStatus::Visiting(start_x, visit_sum * 10 + u)
                }
                (VisitStatus::Empty, Data::Number(n)) => VisitStatus::Visiting(x, *n),
                (VisitStatus::Empty, Data::None | Data::Symbol(..)) => VisitStatus::Empty,
            }
        });
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = get_map(input);

    let mut gear_map = map
        .iter()
        .map(|l| {
            l.iter()
                .map(|d| (d == &Data::Symbol('*')).then(|| vec![]))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    map.iter()
        .enumerate()
        .for_each(|(y, l)| traverse_line_gear(y, l, &map, &mut gear_map));

    Some(
        gear_map
            .into_iter()
            .map(|l| {
                l.into_iter()
                    .filter_map(|v| v)
                    .map(|v| match &v[..] {
                        &[a, b] => a * b,
                        _ => 0,
                    })
                    .sum::<u32>()
            })
            .sum::<u32>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
