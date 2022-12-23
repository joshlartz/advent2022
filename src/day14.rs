use grid::*;
use pathfinding::num_traits::ToPrimitive;
use std::ops::ControlFlow;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Coord {
    x: usize,
    y: usize,
}
pub struct Input {
    lines: Lines,
    x: usize,
    y: usize,
}
type Lines = Vec<Vec<Coord>>;

const SOURCE: Coord = Coord { x: 500, y: 0 };

pub fn generator(input: &str) -> Input {
    let mut max_y: usize = SOURCE.y;
    let mut max_x: usize = SOURCE.x;
    let lines = input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|each| {
                    let coord = each.split_once(',').unwrap();
                    let x: usize = coord.0.parse().unwrap();
                    let y: usize = coord.1.parse().unwrap();
                    if x > max_x {
                        max_x = x;
                    }
                    if y > max_y {
                        max_y = y;
                    }

                    Coord { x, y }
                })
                .collect()
        })
        .collect();

    Input {
        lines,
        x: max_x,
        y: max_y,
    }
}

pub fn part1(input: &Input) -> usize {
    let mut grid: Grid<char> = Grid::init(input.y + 1, input.x + 1, '.');

    draw_rocks(&mut grid, &input.lines);

    // print_grid(&grid);

    let mut count = 0;
    loop {
        if sand(&mut grid, SOURCE).is_break() {
            // print_grid(&grid);
            return count;
        }
        count += 1;
    }
}

pub fn part2(input: &Input) -> usize {
    let mut grid: Grid<char> = Grid::init(input.y + 2, input.x + input.y, '.');

    draw_rocks(&mut grid, &input.lines);

    draw_floor(&mut grid);

    // print_grid(&grid);

    let mut count = 0;
    loop {
        if sand(&mut grid, SOURCE).is_break() {
            // print_grid(&grid);
            return count + 1;
        }
        count += 1;
    }
}

fn draw_rocks(grid: &mut Grid<char>, lines: &Lines) {
    for points in lines {
        for line in points.windows(2) {
            if line[0].y.to_i32().unwrap() - line[1].y.to_i32().unwrap() > 0 {
                for y in line[1].y..=line[0].y {
                    grid[y][line[0].x] = '#'; // down
                }
            }
            if line[0].y.to_i32().unwrap() - line[1].y.to_i32().unwrap() < 0 {
                for y in line[0].y..=line[1].y {
                    grid[y][line[0].x] = '#'; // up
                }
            }
            if line[0].x.to_i32().unwrap() - line[1].x.to_i32().unwrap() > 0 {
                for x in line[1].x..=line[0].x {
                    grid[line[0].y][x] = '#'; // left
                }
            }
            if line[0].x.to_i32().unwrap() - line[1].x.to_i32().unwrap() < 0 {
                for x in line[0].x..=line[1].x {
                    grid[line[0].y][x] = '#'; // right
                }
            }
        }
    }
}

fn sand(grid: &mut Grid<char>, mut grain: Coord) -> ControlFlow<Coord> {
    // down
    while grid.get(grain.y + 1, grain.x) == Some(&'.') {
        grain.y += 1;
    }

    // down & left
    if grid.get(grain.y + 1, grain.x - 1) == Some(&'.') {
        grain.y += 1;
        grain.x -= 1;
        return sand(grid, grain);
    }
    // down & right
    else if grid.get(grain.y + 1, grain.x + 1) == Some(&'.') {
        grain.y += 1;
        grain.x += 1;
        return sand(grid, grain);
    }

    grid[grain.y][grain.x] = 'o';

    if grid.get(grain.y + 1, grain.x).is_none() || grain == SOURCE {
        return ControlFlow::Break(grain);
    }

    ControlFlow::Continue(())
}

#[allow(dead_code)]
fn print_grid(grid: &Grid<char>) {
    for row in 0..grid.rows() {
        println!("{:?}", grid[row].iter().collect::<String>());
    }
}

fn draw_floor(grid: &mut Grid<char>) {
    grid.push_row(vec!['#'; grid.cols()]);
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&generator(SAMPLE)), 24);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&generator(SAMPLE)), 93);
    }
}
