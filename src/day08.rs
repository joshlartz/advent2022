use std::{
    borrow::Borrow,
    collections::{HashMap, HashSet},
};

type Input = Vec<Vec<u32>>;
type Seen = HashSet<(usize, usize)>;
type Scores = HashMap<(usize, usize), u32>;

pub fn generator(input: &str) -> Input {
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| char.to_digit(10).unwrap())
                .collect()
        })
        .collect();

    grid
}

pub fn part1(input: &Input) -> usize {
    let row_max = input.len() - 1;
    let col_max = input[0].len() - 1;

    // length + width + 2 for indexes * 2 - 4 corners
    let edges = (col_max + row_max + 2) * 2 - 4;

    let rows = make_rows(input);
    let cols = make_cols(input, row_max, col_max);

    // track what has been deemed visible already
    let mut seen: Seen = HashSet::new();

    // left & right
    find_visible(&rows, row_max, col_max, &mut seen, false);
    // up & down
    find_visible(&cols, col_max, row_max, &mut seen, true);

    edges + seen.len()
}

pub fn part2(input: &Input) -> u32 {
    let row_max = input.len() - 1;
    let col_max = input[0].len() - 1;

    let rows = make_rows(input);
    let cols = make_cols(input, row_max, col_max);

    // track what has been deemed visible already
    let mut scores: Scores = HashMap::new();

    // left *& right
    find_score(&rows, row_max, col_max, &mut scores, false);
    // up & down
    find_score(&cols, col_max, row_max, &mut scores, true);

    *scores.values().max().unwrap()
}

fn find_visible(
    grid: &Vec<Vec<&u32>>,
    row_max: usize,
    col_max: usize,
    seen: &mut Seen,
    rotated: bool,
) {
    for (mut i, row) in grid[1..row_max].iter().enumerate() {
        i += 1; // intentionally clipping edges with above range, so shift this index accordingly
        for (mut j, col) in row[1..col_max].iter().enumerate() {
            j += 1; // same index shift here

            let index = if rotated { (j, i) } else { (i, j) };

            if seen.contains(&index) {
                continue;
            }

            if grid[i][0..j].iter().all(|tree| tree < col) {
                seen.insert(index); // left
            } else if grid[i][j + 1..=col_max].iter().all(|tree| tree < col) {
                seen.insert(index); // right
            }
        }
    }
}

fn find_score(
    grid: &Vec<Vec<&u32>>,
    row_max: usize,
    col_max: usize,
    scores: &mut Scores,
    rotated: bool,
) {
    for (mut i, row) in grid[1..row_max].iter().enumerate() {
        i += 1; // intentionally clipping edges with above range, so shift this index accordingly
        for (mut j, col) in row[1..col_max].iter().enumerate() {
            j += 1; // same index shift here

            let index = if rotated { (j, i) } else { (i, j) };

            let mut left = 0;
            for tree in grid[i][0..j].iter().rev() {
                left += 1;
                if tree >= col {
                    break;
                }
            }

            let mut right = 0;
            for tree in grid[i][j + 1..=col_max].iter() {
                right += 1;
                if tree >= col {
                    break;
                }
            }

            *scores.entry(index).or_insert(1) *= left * right;
        }
    }
}

/** borrow cells to be compatible with find_visible signature TODO better way */
fn make_rows(input: &Input) -> Vec<Vec<&u32>> {
    input
        .iter()
        .map(|row| row.iter().map(|col| col.borrow()).collect())
        .collect()
}

/** rotate grid counter clockwise 90 so columns can also be sliced for searching */
fn make_cols(input: &Input, row_max: usize, col_max: usize) -> Vec<Vec<&u32>> {
    let mut cols: Vec<Vec<&u32>> = Vec::new();
    for col in 0..=col_max {
        let mut rotated: Vec<&u32> = Vec::new();
        for row in 0..=row_max {
            rotated.push(&input[row][col]);
        }
        cols.push(rotated);
    }
    cols
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&generator(SAMPLE)), 21);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&generator(SAMPLE)), 8);
    }
}
