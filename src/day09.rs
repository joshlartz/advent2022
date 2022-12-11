use std::collections::HashSet;

type Input<'a> = Vec<Motion<'a>>;
/** x, y */
type Coord = (i32, i32);

pub struct Motion<'a> {
    direction: &'a str,
    steps: usize,
}

pub fn generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let (direction, s) = line.split_once(" ").unwrap();
            Motion {
                direction,
                steps: s.parse().unwrap(),
            }
        })
        .collect()
}

pub fn part1(input: &Input) -> usize {
    let mut head: Coord = (0, 0);
    let mut tail: Coord = (0, 0);
    let mut visited: HashSet<Coord> = HashSet::new();

    for motion in input.iter() {
        move_rope(&motion, &mut head, &mut tail, &mut visited);
    }

    visited.len()
}

// pub fn part2(input: &Input) -> u32 {

// }

fn move_rope(motion: &Motion, head: &mut Coord, tail: &mut Coord, visited: &mut HashSet<Coord>) {
    for _ in 0..motion.steps {
        match motion.direction {
            "U" => {
                head.1 += 1;
            }
            "D" => {
                head.1 -= 1;
            }
            "L" => {
                head.0 -= 1;
            }
            "R" => {
                head.0 += 1;
            }
            _ => unreachable!(),
        }
        move_tail(head, tail, visited);
    }
}

fn move_tail(head: &Coord, tail: &mut Coord, visisted: &mut HashSet<Coord>) {
    let distance: Coord = (head.0 - tail.0, head.1 - tail.1);

    if distance.0 == 0 && distance.1 > 1 {
        tail.1 += 1; // up
    }
    if distance.0 == 0 && distance.1 < -1 {
        tail.1 -= 1; // down
    }
    if distance.0 < -1 && distance.1 == 0 {
        tail.0 -= 1; // left
    }
    if distance.0 > 1 && distance.1 == 0 {
        tail.0 += 1; // right
    }
    if (distance.0 <= -1 && distance.1 > 1) || (distance.0 < -1 && distance.1 >= 1) {
        *tail = (tail.0 - 1, tail.1 + 1); // up/left
    }
    if (distance.0 >= 1 && distance.1 > 1) || (distance.0 > 1 && distance.1 >= 1) {
        *tail = (tail.0 + 1, tail.1 + 1); // up/right
    }
    if (distance.0 <= -1 && distance.1 < -1) || (distance.0 < -1 && distance.1 <= -1) {
        *tail = (tail.0 - 1, tail.1 - 1); // down/left
    }
    if (distance.0 >= 1 && distance.1 < -1) || (distance.0 > 1 && distance.1 <= -1) {
        *tail = (tail.0 + 1, tail.1 - 1); // down/right
    }

    visisted.insert(*tail);
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE1: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    const SAMPLE2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&generator(SAMPLE1)), 13);
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(&generator(SAMPLE)), 36);
    // }
}
