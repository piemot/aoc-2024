use std::collections::HashSet;

use aoc_2024::*;
use itertools::Itertools;

enum Tile {
    None,
    Node(char),
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            'a'..='z' => Self::Node(value),
            'A'..='Z' => Self::Node(value),
            '0'..='9' => Self::Node(value),
            _ => Self::None,
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

    pub fn data(&self) -> &Vec<T> {
        &self.data
    }

    pub fn width(&self) -> isize {
        self.width as isize
    }

    pub fn height(&self) -> isize {
        self.data.len() as isize / self.width()
    }

    #[allow(dead_code)]
    pub fn at_index(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }

    #[allow(dead_code)]
    pub fn at(&self, pos: (isize, isize)) -> Option<&T> {
        self.at_index(self.coord_to_index(pos))
    }

    #[allow(dead_code)]
    pub fn set(&mut self, pos: (isize, isize), value: T) {
        let ind = self.coord_to_index(pos);
        self.data[ind] = value;
    }

    #[allow(dead_code)]
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

day!(part1, part2);

fn part1(input: &'static str) -> usize {
    let mut lines = input.lines().peekable();
    let width = lines.peek().unwrap().len();
    let data: Vec<Tile> = lines
        .map(|l| l.chars())
        .flat_map(|chars| chars.map(Into::into))
        .collect();

    let grid = Grid::new(width, data);

    let unique_tiles: HashSet<&char> = grid
        .data()
        .iter()
        .filter_map(|tile| match tile {
            Tile::Node(ch) => Some(ch),
            _ => None,
        })
        .collect();

    let mut global_antinodes = HashSet::new();

    for c in unique_tiles {
        let positions = grid
            .data()
            .iter()
            .enumerate()
            .filter(|(_, value)| matches!(value, Tile::Node(ch) if ch == c))
            .map(|(index, _)| index);

        // positions.count() == count

        let ops = positions.combinations(2);
        for op in ops {
            let [a, b] = op[..] else {
                unreachable!();
            };
            let (a, b) = (grid.index_to_coord(a), grid.index_to_coord(b));

            let vector = (a.0 - b.0, a.1 - b.1);
            let antinodes = [
                (a.0 + vector.0, a.1 + vector.1),
                (b.0 - vector.0, b.1 - vector.1),
            ];
            for node in antinodes {
                if (0..grid.width()).contains(&node.0) && (0..grid.height()).contains(&node.1) {
                    global_antinodes.insert(node);
                }
            }
        }
    }

    global_antinodes.len()
}

fn part2(input: &'static str) -> usize {
    let mut lines = input.lines().peekable();
    let width = lines.peek().unwrap().len();
    let data: Vec<Tile> = lines
        .map(|l| l.chars())
        .flat_map(|chars| chars.map(Into::into))
        .collect();

    let grid = Grid::new(width, data);

    let unique_tiles: HashSet<&char> = grid
        .data()
        .iter()
        .filter_map(|tile| match tile {
            Tile::Node(ch) => Some(ch),
            _ => None,
        })
        .collect();

    let mut global_antinodes = HashSet::new();

    for c in unique_tiles {
        println!("=> {:?}", c);
        let positions = grid
            .data()
            .iter()
            .enumerate()
            .filter(|(_, value)| matches!(value, Tile::Node(ch) if ch == c))
            .map(|(index, _)| index);

        let ops = positions.combinations(2);
        for op in ops {
            let [a, b] = op[..] else {
                unreachable!();
            };
            let (a, b) = (grid.index_to_coord(a), grid.index_to_coord(b));

            let vector = (a.0 - b.0, a.1 - b.1);
            let mut i = 0;
            // Proceeding both back and forth across the vector starting at `a`,
            // periodically add an antinode until we leave the map
            loop {
                let antinode = (a.0 + vector.0 * i, a.1 + vector.1 * i);
                if (0..grid.width()).contains(&antinode.0)
                    && (0..grid.height()).contains(&antinode.1)
                {
                    global_antinodes.insert(antinode);
                    i += 1;
                } else {
                    break;
                }
            }
            i = -1;
            loop {
                let antinode = (a.0 + vector.0 * i, a.1 + vector.1 * i);
                if (0..grid.width()).contains(&antinode.0)
                    && (0..grid.height()).contains(&antinode.1)
                {
                    global_antinodes.insert(antinode);
                    i -= 1;
                } else {
                    break;
                }
            }
        }
    }

    global_antinodes.len()
}

#[cfg(test)]
mod test {
    use crate::*;

    /*
    ............
    ........0...
    .....0......
    .......0....
    ....0.......
    ......A.....
    ............
    ............
    ........A...
    .........A..
    ............
    ............
    */

    const SAMPLE_INPUT: &'static str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    test_day!(test_part1 -> part1(SAMPLE_INPUT), 14);
    test_day!(test_part2 -> part2(SAMPLE_INPUT), 34);
}
