advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let lines = input.lines();
    let count = lines.count();
    let (mut v1, mut v2) = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|x| x.parse::<u64>().expect("Should parse"))
                .collect::<Vec<u64>>()
        })
        .fold(
            (Vec::with_capacity(count), Vec::with_capacity(count)),
            |(mut v1, mut v2), lv| {
                v1.push(lv[0]);
                v2.push(lv[1]);
                (v1, v2)
            },
        );
    v1.sort();
    v2.sort();
    Some(
        v1.into_iter()
            .zip(v2.into_iter())
            .map(|(x, y)| if x > y { x - y } else { y - x })
            .sum::<u64>(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = input.lines();
    let count = lines.count();
    let (v1, v2) = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|x| x.parse::<u64>().expect("Should parse"))
                .collect::<Vec<u64>>()
        })
        .fold(
            (Vec::with_capacity(count), Vec::with_capacity(count)),
            |(mut v1, mut v2), lv| {
                v1.push(lv[0]);
                v2.push(lv[1]);
                (v1, v2)
            },
        );
    let max = v2
        .iter()
        .max()
        .copied()
        .expect("There should be a max number") as usize;
    let counts = v2.into_iter().fold(vec![0; max + 1], |mut acc, x| {
        acc[x as usize] += 1;
        acc
    });
    Some(v1.into_iter().map(|x| x * counts[x as usize]).sum::<u64>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
