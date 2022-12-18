use grid::*;
use itertools::Itertools;
use pathfinding::{prelude::{astar, bfs, dijkstra}, num_traits::ToPrimitive};

type Input = Grid<i16>;
type BfsNeighbor = Coord;
type DijkstraNeighbor = (Coord, i16);

pub trait Neighbor<T> {
    fn neighbor(&self) -> Vec<T>;
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Coord {
    x: usize,
    y: usize,
}

struct Map {
    grid: Grid<i16>,
    start: Coord,
    end: Coord,
}
impl Map {
    fn new(input: &Input) -> Self {
        let find = |char: char| -> Option<Coord> {
            for y in 0..input.rows() {
                let find = input.iter_row(y).position(|&x| x == char as i16);
                if let Some(x) = find {
                    return Some(Coord { x, y });
                }
            }
            None
        };

        let mut grid = input.clone();
        let start = find('S').unwrap();
        let end = find('E').unwrap();
        grid[start.y][start.x] = (b'a') as i16;
        grid[end.y][end.x] = (b'z') as i16;

        Self { grid, start, end }
    }

    fn bfs_neighbors(&self, position: &Coord) -> Vec<BfsNeighbor> {
        let neighbors: Vec<BfsNeighbor> = Vec::new();
        let store = |coord: Coord| coord;
        self.eligible_neighbors(position, neighbors, store)
    }

    fn dijkstra_neighbors(&self, position: &Coord) -> Vec<DijkstraNeighbor> {
        let neighbors: Vec<DijkstraNeighbor> = Vec::new();
        let store = |coord: Coord| (coord, 1);
        self.eligible_neighbors(position, neighbors, store)
    }

    fn eligible_neighbors<T>(
        &self,
        position: &Coord,
        mut neighbors: Vec<T>,
        store: fn(Coord) -> T,
    ) -> Vec<T> {
        let current_elevation = self.grid.get(position.y, position.x).unwrap();

        let mut check_neighbor = |coord: Coord| {
            if let Some(elevation) = self.grid.get(coord.y, coord.x) {
                if elevation - current_elevation < 2 {
                    neighbors.push(store(coord))
                }
            }
        };

        if position.y > 0 {
            check_neighbor(Coord {
                y: position.y - 1, // up
                x: position.x,
            });
        }
        if position.y < self.grid.rows() - 1 {
            check_neighbor(Coord {
                y: position.y + 1, // down
                x: position.x,
            });
        }
        if position.x > 0 {
            check_neighbor(Coord {
                y: position.y,
                x: position.x - 1, // left
            });
        }
        if position.x < self.grid.cols() - 1 {
            check_neighbor(Coord {
                y: position.y,
                x: position.x + 1, // right
            });
        }

        neighbors
    }

    fn heuristic(&self, position: &Coord) -> i16 {
        let a = self.end.y.abs_diff(position.y);
        let b = self.end.x.abs_diff(position.x);
        // an actual use for the pythagorean theory lol
        f32::sqrt((a.pow(2) + b.pow(2)) as f32).to_i16().unwrap()
    }
}

pub fn generator(input: &str) -> Input {
    let mut grid = Grid::new(0, 0);

    for line in input.lines() {
        grid.push_row(line.chars().map(|char| char as i16).collect_vec());
    }

    grid
}

pub fn part1_bfs(input: &Input) -> usize {
    let map = Map::new(input);

    let path = bfs(
        &map.start,
        |coord| map.bfs_neighbors(coord),
        |coord| map.end.eq(coord),
    )
    .unwrap_or_else(|| panic!("no path found"));

    path.len() - 1
}

pub fn part1_dijkstra(input: &Input) -> i16 {
    let map = Map::new(input);

    let path = dijkstra(
        &map.start,
        |coord| map.dijkstra_neighbors(coord),
        |coord| map.end.eq(coord),
    )
    .unwrap_or_else(|| panic!("no path found"));

    path.1
}

pub fn part1_astar(input: &Input) -> i16 {
    let map = Map::new(input);

    let path = astar(
        &map.start,
        |coord| map.dijkstra_neighbors(coord),
        |coord| map.heuristic(coord),
        |coord| map.end.eq(coord),
    )
    .unwrap_or_else(|| panic!("no path found"));

    path.1
}

// pub fn part2(input: &[Input]) -> u64 {
//     rounds(input.iter().map(Monkey::new).collect(), 10_000, true)
// }

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn test_part1_bfs() {
        assert_eq!(part1_bfs(&generator(SAMPLE)), 31);
    }

    #[test]
    fn test_part1_dijkstra() {
        assert_eq!(part1_dijkstra(&generator(SAMPLE)), 31);
    }

    #[test]
    fn test_part1_astar() {
        assert_eq!(part1_astar(&generator(SAMPLE)), 31);
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(&generator(SAMPLE)), 2713310158);
    // }
}
