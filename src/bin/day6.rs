use std::collections::HashSet;

use aoc_2024::*;

day!(part1, part2);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Guard,
    Obstacle,
    None,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub const fn rotate(self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }

    pub const fn translation(self) -> (isize, isize) {
        match self {
            Self::North => (0, -1),
            Self::East => (1, 0),
            Self::South => (0, 1),
            Self::West => (-1, 0),
        }
    }
}

impl TryFrom<char> for Tile {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(Self::Guard),
            '#' => Ok(Self::Obstacle),
            '.' => Ok(Self::None),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Grid<T> {
    pub width: usize,
    pub data: Vec<T>,
}

impl<T> Grid<T> {
    pub const fn new(width: usize, data: Vec<T>) -> Self {
        Self { width, data }
    }

    pub fn at_index(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }

    pub fn at(&self, pos: (isize, isize)) -> Option<&T> {
        self.at_index(self.coord_to_index(pos))
    }

    pub fn set(&mut self, pos: (isize, isize), value: T) {
        let ind = self.coord_to_index(pos);
        self.data[ind] = value;
    }

    pub fn coord_to_index(&self, pos: (isize, isize)) -> usize {
        assert!(
            pos.0 >= 0 && pos.1 >= 0,
            "Invalid values provided to coord_to_index"
        );
        pos.1 as usize * self.width + pos.0 as usize
    }

    #[allow(dead_code)]
    pub fn index_to_coord(&self, ind: usize) -> (isize, isize) {
        ((ind % self.width) as isize, (ind / self.width) as isize)
    }
}

fn part1(input: &'static str) -> usize {
    let mut lines = input.lines().peekable();
    let width = lines.peek().unwrap().len();
    let mut grid: Grid<Tile> = Grid::new(width, vec![]);

    for line in lines {
        for char in line.chars() {
            grid.data.push(char.try_into().unwrap());
        }
    }

    let mut visited_positions = HashSet::new();
    let mut direction = Direction::North;
    let guard_pos = grid.data.iter().position(|t| *t == Tile::Guard).unwrap();
    let mut guard_pos = ((guard_pos % width) as isize, (guard_pos / width) as isize);

    loop {
        let next_tile = (
            guard_pos.0 + direction.translation().0,
            guard_pos.1 + direction.translation().1,
        );
        // println!("{:?} => {:?}", guard_pos, next_tile);
        if next_tile.0 < 0
            || next_tile.1 < 0
            || next_tile.0 as usize >= width
            || next_tile.1 as usize >= grid.data.len() / width
        {
            break;
        }

        if *grid.at(next_tile).unwrap() == Tile::Obstacle {
            direction = direction.rotate();
            continue;
        }

        visited_positions.insert(guard_pos);
        guard_pos = next_tile;
    }

    visited_positions.len() + 1
}

fn escape(grid: &Grid<Tile>) -> bool {
    let mut count = 0;
    let mut direction = Direction::North;
    let guard_pos = grid.data.iter().position(|t| *t == Tile::Guard).unwrap();
    let mut guard_pos = (
        (guard_pos % grid.width) as isize,
        (guard_pos / grid.width) as isize,
    );

    loop {
        let next_tile = (
            guard_pos.0 + direction.translation().0,
            guard_pos.1 + direction.translation().1,
        );
        if next_tile.0 < 0
            || next_tile.1 < 0
            || next_tile.0 as usize >= grid.width
            || next_tile.1 as usize >= grid.data.len() / grid.width
        {
            return false;
        }
        // This value started at 1000, but the answer it gave
        // was extremely incorrrect: some paths are very long.
        if count >= 30_000 {
            return true;
        }

        if *grid.at(next_tile).unwrap() == Tile::Obstacle {
            direction = direction.rotate();
            continue;
        }

        count += 1;
        guard_pos = next_tile;
    }
}

fn part2(input: &'static str) -> i32 {
    let mut lines = input.lines().peekable();
    let width = lines.peek().unwrap().len();
    let mut grid: Grid<Tile> = Grid::new(width, vec![]);

    for line in lines {
        for char in line.chars() {
            grid.data.push(char.try_into().unwrap());
        }
    }

    let mut matches = 0;

    for x in 0..width {
        for y in 0..(grid.data.len() / width) {
            if *grid.at((x as isize, y as isize)).unwrap() == Tile::None {
                grid.set((x as isize, y as isize), Tile::Obstacle);
                if escape(&grid) {
                    matches += 1;
                }
                grid.set((x as isize, y as isize), Tile::None);
            }
        }
    }

    matches
}

#[cfg(test)]
mod test {
    use crate::*;

    const SAMPLE_INPUT: &'static str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    test_day!(test_part1 -> part1(SAMPLE_INPUT), 41);
    test_day!(test_part2 -> part2(SAMPLE_INPUT), 6);
}
