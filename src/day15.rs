type Input = Vec<Pair>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Coord {
    x: i32,
    y: i32,
}
impl Coord {
    fn within(&self, max: &i32) -> bool {
        self.x >= 0 && self.x <= *max && self.y >= 0 && self.y <= *max
    }
}

#[derive(Debug)]
pub struct Pair {
    beacon: Coord,
    sensor: Coord,
    distance: i32,
}

pub fn generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let pair = line[10..].split_once(": closest beacon is at ").unwrap();
            let sensor = pair.0[2..].split_once(", y=").unwrap();
            let beacon = pair.1[2..].split_once(", y=").unwrap();
            let mut pair = Pair {
                sensor: Coord {
                    x: sensor.0.parse().unwrap(),
                    y: sensor.1.parse().unwrap(),
                },
                beacon: Coord {
                    x: beacon.0.parse().unwrap(),
                    y: beacon.1.parse().unwrap(),
                },
                distance: 0,
            };
            pair.distance = manhattan_distance(&pair.beacon, &pair.sensor);
            pair
        })
        .collect()
}
pub fn part1(input: &Input) -> usize {
    part1_row(input, None)
}

fn part1_row(input: &Input, row: Option<i32>) -> usize {
    let y = row.unwrap_or(2_000_000);

    let mut count = 0;
    for x in -6_000_000..6_000_000 {
        if check_coverage(input, y, x) {
            count += 1;
        }
    }

    count
}

pub fn part2(input: &Input) -> i64 {
    part2_max(input, None)
}

fn part2_max(input: &Input, max: Option<i32>) -> i64 {
    let max = max.unwrap_or(4_000_000);

    for edge in input.iter().flat_map(|pair| perimeter(pair, &max)) {
        if !check_coverage(input, edge.y, edge.x) && !input.iter().any(|pair| pair.beacon == edge) {
            return (edge.x as i64) * 4_000_000 + (edge.y as i64);
        }
    }

    panic!("no value found")
}

fn manhattan_distance(a: &Coord, b: &Coord) -> i32 {
    (a.x.abs_diff(b.x) + a.y.abs_diff(b.y)).try_into().unwrap()
}

fn check_coverage(input: &Input, y: i32, x: i32) -> bool {
    let coord = Coord { x, y };
    input
        .iter()
        .any(|pair| manhattan_distance(&pair.sensor, &coord) <= pair.distance)
        && input.iter().all(|pair| pair.beacon != coord)
}

/** coords just past sensor edge */
fn perimeter(pair: &Pair, max: &i32) -> Vec<Coord> {
    let mut edges: Vec<Coord> = Vec::new();

    let up = Coord {
        x: pair.sensor.x,
        y: pair.sensor.y - pair.distance - 1,
    };
    let down = Coord {
        x: pair.sensor.x,
        y: pair.sensor.y + pair.distance + 1,
    };
    let left = Coord {
        x: pair.sensor.x - pair.distance - 1,
        y: pair.sensor.y,
    };
    let right = Coord {
        x: pair.sensor.x + pair.distance + 1,
        y: pair.sensor.y,
    };

    for i in 0..=pair.distance {
        let ne = Coord {
            x: up.x + i,
            y: up.y + i,
        };
        let se = Coord {
            x: right.x - i,
            y: right.y + i,
        };
        let sw = Coord {
            x: down.x - i,
            y: up.y - i,
        };
        let nw = Coord {
            x: left.x + i,
            y: up.y - i,
        };
        if ne.within(max) {
            edges.push(ne);
        }
        if se.within(max) {
            edges.push(se);
        }
        if sw.within(max) {
            edges.push(sw);
        }
        if nw.within(max) {
            edges.push(nw);
        }
    }

    edges
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn test_part1() {
        assert_eq!(part1_row(&generator(SAMPLE), Some(10)), 26);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2_max(&generator(SAMPLE), Some(20)), 56000011);
    }
}
