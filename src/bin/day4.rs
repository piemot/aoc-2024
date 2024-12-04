use aoc_2024::*;

day!(part1, part2);

#[derive(Debug, PartialEq, Eq, Clone)]
enum Tile {
    X,
    M,
    A,
    S,
    Other,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            'X' => Tile::X,
            'M' => Tile::M,
            'A' => Tile::A,
            'S' => Tile::S,
            _ => Tile::Other,
        }
    }
}

type Point = (usize, usize);

#[derive(Debug, PartialEq, Eq, Clone)]
struct Grid(pub usize, pub Vec<Tile>);

impl Grid {
    pub fn get(&self, pt: Point) -> &Tile {
        &self.1[pt.1 * self.0 + pt.0]
    }
    pub fn get_checked(&self, pt: (isize, isize)) -> Option<&Tile> {
        if pt.0 < 0 || pt.1 < 0 || pt.0 >= self.width() as isize || pt.1 >= self.height() as isize {
            return None;
        }

        Some(&self.1[pt.1 as usize * self.0 + pt.0 as usize])
    }

    pub fn ranges(&self) -> Vec<(usize, usize)> {
        let mut pts = vec![];
        for x in 0..(self.width() - 2) {
            for y in 0..(self.height() - 2) {
                pts.push((x, y));
            }
        }
        pts
    }

    pub fn width(&self) -> usize {
        self.0
    }

    pub fn height(&self) -> usize {
        self.1.len() / self.0
    }

    pub fn surrounding(&self, pt: Point) -> Vec<(usize, usize)> {
        let mut surrounding_pts = vec![];
        let pt = (pt.0 as isize, pt.1 as isize);
        println!("lookup xy [{:?}]", pt);
        let translations = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        for t in translations {
            let new_pt = (pt.0 + t.0, pt.1 + t.1);
            if new_pt.0 < 0
                || new_pt.1 < 0
                || new_pt.0 >= self.width() as isize
                || new_pt.1 >= self.height() as isize
            {
                continue;
            }
            surrounding_pts.push((new_pt.0 as usize, new_pt.1 as usize));
        }
        surrounding_pts
    }

    pub fn translations(&self, pt: Point) -> Vec<(isize, isize)> {
        let mut surrounding_ts = vec![];
        let pt = (pt.0 as isize, pt.1 as isize);
        let translations = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        for t in translations {
            let new_pt = (pt.0 + t.0, pt.1 + t.1);
            if new_pt.0 < 0
                || new_pt.1 < 0
                || new_pt.0 >= self.width() as isize
                || new_pt.1 >= self.height() as isize
            {
                continue;
            }
            surrounding_ts.push(t);
        }
        surrounding_ts
    }
}

fn part1(input: &'static str) -> usize {
    let mut lines = input.lines().peekable();
    let width = lines.peek().unwrap().len();
    let mut tiles = Grid(width, vec![]);
    for line in lines {
        tiles.1.extend(line.chars().map(|c| c.into()));
    }

    let mut matches = 0;

    let starting_chars = tiles
        .1
        .iter()
        .enumerate()
        .filter(|(_ind, t)| t == &&Tile::X)
        .map(|(ind, _t)| ind);

    for index in starting_chars {
        let pt = (index % tiles.width(), index / tiles.width());
        for t in tiles.translations(pt) {
            let next_tile = add_points(pt, t);
            if let Some(tile) = tiles.get_checked(next_tile) {
                if tile == &Tile::M {
                    let next_tile = add_points_i(next_tile, t);
                    if let Some(tile) = tiles.get_checked(next_tile) {
                        if tile == &Tile::A {
                            let next_tile = add_points_i(next_tile, t);
                            if let Some(tile) = tiles.get_checked(next_tile) {
                                if tile == &Tile::S {
                                    matches += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    matches
}

fn part2(input: &'static str) -> usize {
    let mut lines = input.lines().peekable();
    let width = lines.peek().unwrap().len();
    let mut tiles = Grid(width, vec![]);
    for line in lines {
        tiles.1.extend(line.chars().map(|c| c.into()));
    }

    tiles
        .ranges()
        .iter()
        .filter(|top_pt| {
            let row_one = [
                tiles.get(**top_pt),
                tiles.get((top_pt.0 + 1, top_pt.1 + 1)),
                tiles.get((top_pt.0 + 2, top_pt.1 + 2)),
            ];
            if row_one != [&Tile::M, &Tile::A, &Tile::S]
                && row_one != [&Tile::S, &Tile::A, &Tile::M]
            {
                return false;
            }
            let row_two = [
                tiles.get((top_pt.0 + 2, top_pt.1)),
                tiles.get((top_pt.0 + 1, top_pt.1 + 1)),
                tiles.get((top_pt.0, top_pt.1 + 2)),
            ];
            if row_two != [&Tile::M, &Tile::A, &Tile::S]
                && row_two != [&Tile::S, &Tile::A, &Tile::M]
            {
                return false;
            }
            true
        })
        .count()
}

fn add_points(pt: (usize, usize), t: (isize, isize)) -> (isize, isize) {
    let pt = (pt.0 as isize, pt.1 as isize);
    (pt.0 + t.0, pt.1 + t.1)
}
fn add_points_i(pt: (isize, isize), t: (isize, isize)) -> (isize, isize) {
    (pt.0 + t.0, pt.1 + t.1)
}

/*

Lets it go in any direction

for index in starting_chars {
        for surrounding in tiles
            .surrounding((index % tiles.width(), index / tiles.width()))
            .iter()
            .filter(|ind| *tiles.get(**ind) == Tile::M)
        {
            for surrounding in tiles
                .surrounding(*surrounding)
                .iter()
                .filter(|ind| *tiles.get(**ind) == Tile::A)
            {
                for _ in tiles
                    .surrounding(*surrounding)
                    .iter()
                    .filter(|ind| *tiles.get(**ind) == Tile::S)
                {
                    matches += 1;
                }
            }
        }
    }

*/

#[cfg(test)]
mod test {
    use crate::*;

    /*
            ....XXMAS.
            .SAMXMS...
            ...S..A...
            ..A.A.MS.X
            XMASAMX.MM
            X.....XA.A
            S.S.S.S.SS
            .A.A.A.A.A
            ..M.M.M.MM
            .X.X.XMASX
    */
    const SAMPLE_INPUT: &'static str = "....XXMAS.
.SAMXMS...
...S..A...
..A.A.MS.X
XMASAMX.MM
X.....XA.A
S.S.S.S.SS
.A.A.A.A.A
..M.M.M.MM
.X.X.XMASX";

    test_day!(test_part1 -> part1(SAMPLE_INPUT), 18);
    test_day!(test_part2 -> part2(SAMPLE_INPUT), 3);
}
