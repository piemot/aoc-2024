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

#[derive(Debug, Clone)]
struct Grid<T> {
    width: usize,
    items: Vec<T>,
}

const ALL_TRANSLATIONS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

impl<T> Grid<T> {
    // Create a new grid of a given width.
    pub fn new(width: usize) -> Self {
        Self {
            width,
            items: vec![],
        }
    }

    #[allow(dead_code)]
    // Create a new grid of a given width, with space for at least `capacity` elements before reallocating.
    pub fn with_capacity(width: usize, capacity: usize) -> Self {
        Self {
            width,
            items: Vec::with_capacity(capacity),
        }
    }

    /// Get the value of a single point in the grid.
    /// If it is outside the grid, `None` will be returned.
    pub fn get(&self, pt: Point) -> Option<&T> {
        if pt.0 >= self.width() || pt.1 >= self.height() {
            return None;
        }

        Some(&self.items[pt.1 as usize * self.width + pt.0 as usize])
    }

    /// Get the value of a single point in the grid, using `isize`s as indices.
    /// If it is outside the grid, `None` will be returned.
    pub fn get_isize(&self, pt: (isize, isize)) -> Option<&T> {
        if pt.0 < 0 || pt.1 < 0 {
            return None;
        }

        self.get((pt.0 as usize, pt.1 as usize))
    }

    /// Get all points in the grid, except those covered by an offset of the given parameters
    /// ## Example:
    /// ```ex
    ///     y1      y2
    ///    +--+----+--+
    /// x1 |  |    |  |
    ///    +--+----+--+
    ///    |  |0000|  |
    ///    |  |0000|  |
    ///    +--+----+--+
    /// x2 |  |    |  |
    ///    +--+----+--+
    /// ```
    /// Points marked with `0` in this example would be returned.
    pub fn range_offset(&self, x1: usize, y1: usize, x2: usize, y2: usize) -> Vec<Point> {
        let mut pts = vec![];
        for x in x1..(self.width() - x2) {
            for y in y1..(self.height() - y2) {
                pts.push((x, y));
            }
        }
        pts
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.items.len() / self.width()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.items.iter()
    }

    #[allow(dead_code)]
    pub fn surrounding(&self, pt: Point) -> Vec<Point> {
        let mut surrounding_pts = vec![];
        let pt = (pt.0 as isize, pt.1 as isize);

        for t in ALL_TRANSLATIONS {
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

    pub fn translations(&self, pt: Point) -> impl Iterator<Item = &(isize, isize)> {
        let pt = (pt.0 as isize, pt.1 as isize);
        ALL_TRANSLATIONS.iter().filter(move |t| {
            let new_pt = (pt.0 + t.0, pt.1 + t.1);
            !(new_pt.0 < 0
                || new_pt.1 < 0
                || new_pt.0 >= self.width() as isize
                || new_pt.1 >= self.height() as isize)
        })
    }
}

impl<A> Extend<A> for Grid<A> {
    fn extend<T: IntoIterator<Item = A>>(&mut self, iter: T) {
        self.items.extend(iter);
    }
}

fn part1(input: &'static str) -> usize {
    let mut lines = input.lines().peekable();
    let width = lines.peek().unwrap().len();
    let mut tiles: Grid<Tile> = Grid::new(width);
    for line in lines {
        tiles.extend(line.chars().map(|c| c.into()));
    }

    let mut matches = 0;

    let starting_chars = tiles
        .iter()
        .enumerate()
        .filter(|(_ind, t)| t == &&Tile::X)
        .map(|(ind, _t)| ind);

    for index in starting_chars {
        let pt = (index % tiles.width(), index / tiles.width());
        for t in tiles.translations(pt) {
            let next_tile = add_points(pt, *t);
            if let Some(tile) = tiles.get_isize(next_tile) {
                if tile == &Tile::M {
                    let next_tile = add_points_i(next_tile, *t);
                    if let Some(tile) = tiles.get_isize(next_tile) {
                        if tile == &Tile::A {
                            let next_tile = add_points_i(next_tile, *t);
                            if let Some(tile) = tiles.get_isize(next_tile) {
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
    let mut tiles: Grid<Tile> = Grid::new(width);
    for line in lines {
        tiles.extend(line.chars().map(|c| c.into()));
    }

    tiles
        .range_offset(0, 0, 2, 2)
        .iter()
        .filter(|top_pt| {
            let row_one = [
                tiles.get(**top_pt).unwrap(),
                tiles.get((top_pt.0 + 1, top_pt.1 + 1)).unwrap(),
                tiles.get((top_pt.0 + 2, top_pt.1 + 2)).unwrap(),
            ];
            if row_one != [&Tile::M, &Tile::A, &Tile::S]
                && row_one != [&Tile::S, &Tile::A, &Tile::M]
            {
                return false;
            }
            let row_two = [
                tiles.get((top_pt.0 + 2, top_pt.1)).unwrap(),
                tiles.get((top_pt.0 + 1, top_pt.1 + 1)).unwrap(),
                tiles.get((top_pt.0, top_pt.1 + 2)).unwrap(),
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

#[cfg(test)]
mod test {
    use crate::*;

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
