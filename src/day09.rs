use std::collections::HashSet;

type Input<'a> = Vec<Motion<'a>>;
/** x, y */
type Coord = (i32, i32);
type Rope = Vec<Coord>;

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
    let mut rope: Rope = vec![(0, 0); 2];
    let mut visited: HashSet<Coord> = HashSet::new();

    for motion in input.iter() {
        move_rope(&motion, &mut rope, &mut visited);
    }

    visited.len()
}

pub fn part2(input: &Input) -> usize {
    let mut rope: Rope = vec![(0, 0); 10];
    let mut visited: HashSet<Coord> = HashSet::new();

    for motion in input.iter() {
        move_rope(&motion, &mut rope, &mut visited);
    }

    visited.len()
}

fn move_rope(motion: &Motion, rope: &mut Rope, visited: &mut HashSet<Coord>) {
    let tail = rope.len() - 1;
    for _ in 0..motion.steps {
        move_head(motion.direction, &mut rope[0]);

        for head in 0..tail {
            move_knot(rope, head, head + 1)
        }

        visited.insert(rope[tail]);
    }
}

fn move_head(direction: &str, head: &mut Coord) {
    match direction {
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
}

fn move_knot(rope: &mut Rope, head: usize, tail: usize) {
    let distance: Coord = (rope[head].0 - rope[tail].0, rope[head].1 - rope[tail].1);
    let tail = &mut rope[tail];

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

    #[test]
    fn test_part2() {
        assert_eq!(part2(&generator(SAMPLE1)), 1);
        assert_eq!(part2(&generator(SAMPLE2)), 36);
    }
}
