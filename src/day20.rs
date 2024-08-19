use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display};
use std::fs::read_to_string;
use std::io::{self, Write};

#[allow(unused_imports)]
use itertools::Itertools;

pub struct Solution {
    content: String,
    tiles: Option<HashMap<usize, Image>>,
    corners: Option<Vec<usize>>,
}

const TILE_SIZE: usize = 10;

impl Solution {
    pub fn init() -> Self {
        let content = read_to_string("inputs/day20.txt").unwrap();

        Self {
            content,
            tiles: None,
            corners: None,
        }
    }

    fn part1(&mut self) -> usize {
        // Parse the input into tiles
        let mut tiles = HashMap::new();
        for block in self.content.split("\r\n\r\n") {
            let (id, tile) = Image::parse(&block.lines().collect::<Vec<_>>());
            tiles.insert(id, tile);
        }

        // Find the adjacents for each tile
        let static_tiles = tiles.clone();
        for (_, tile) in tiles.iter_mut() {
            tile.compute_adjacents(&static_tiles);
        }

        // Find the corners, ie tiles with only 2 adjacents
        let corners: Vec<_> = tiles.iter()
            .filter(|(_, tile)| tile.adjacents.len() == 2)
            .map(|(id, _)| *id)
            .collect();
        let result = corners.iter().product::<usize>();

        // Save the results for part 2
        self.tiles = Some(tiles);
        self.corners = Some(corners);

        result
    }

    fn part2(&mut self) -> usize {
        let tiles = self.tiles.as_mut().unwrap();
        let corners = self.corners.as_mut().unwrap();

        // Rebuild the image
        // 1) Pick a corner tile
        let mut image = HashMap::new();
        let mut queue = vec![(0, 0)];
        let mut placed_tiles = HashSet::new();
        image.insert((0, 0), tiles.get(&corners[0]).unwrap().clone());
        placed_tiles.insert(corners[0]);

        // 2) Place the other tiles
        while let Some((x,y)) = queue.pop() {
            let tile = image.get(&(x, y)).unwrap().clone();
            for adjacent_id in &tile.adjacents {
                // This tile is already placed, skip it
                if placed_tiles.contains(adjacent_id) {
                    continue;
                }

                // Find the shared border (s because one is also flipped)
                let mut adjacent_tile = tiles.get(adjacent_id).unwrap().clone();

                // Find the border that is not flipped, and the direction
                let shared_border = tile.common_border(&adjacent_tile).unwrap();
                let (direction, _false) = tile.identify_border(&shared_border);
                // println!("Shared border: {}->{} {:?} {:?} / {:?} {}", tile_id, adjacent_id, shared_border, direction, adjacent_direction, flipped);

                let needs_flip = loop {
                    // Rotate the adjacent tile until the shared border is on the right side
                    let (adjacent_direction, flipped) = adjacent_tile.identify_border(&shared_border);
                    if adjacent_direction == direction.opposite() {
                        break !flipped;
                    }
                    adjacent_tile.rotate();
                };

                // Flip if needed
                match direction {
                    Direction::Top | Direction::Bottom if needs_flip => adjacent_tile.flip_horizontal(),
                    Direction::Right | Direction::Left if needs_flip => adjacent_tile.flip_vertical(),
                    _ => (),
                }
                
                // Place the tile
                let (dx, dy) = direction.to_delta();
                let (nx, ny) = (x + dx, y + dy);

                image.insert((nx, ny), adjacent_tile);
                placed_tiles.insert(*adjacent_id);
                queue.push((nx, ny));
            }
        }

        // 4) Find the min and max coordinates
        let (min_x, max_x) = image.keys().map(|(x, _)| *x).minmax().into_option().unwrap();
        let (min_y, max_y) = image.keys().map(|(_, y)| *y).minmax().into_option().unwrap();
        let size = (max_x - min_x + 1) as usize;

        // 5) Build the final image
        let mut final_image = vec![vec![Pixel::Dot; size * (TILE_SIZE - 2)]; size * (TILE_SIZE - 2)];
        for y in min_y..=max_y {
            for x in min_x..= max_x {
                let tile = image.get(&(x, y)).unwrap();
                for (i, tile_row) in tile.contents.iter().enumerate() {
                    for (j, pixel) in tile_row.iter().enumerate() {
                        if i == 0 || i == TILE_SIZE - 1 || j == 0 || j == TILE_SIZE - 1 {
                            continue;
                        }

                        let px = (x - min_x) as usize * (TILE_SIZE - 2) + j - 1;
                        let py = (y - min_y) as usize * (TILE_SIZE - 2) + i - 1;
                        final_image[py][px] = *pixel;
                    }
                }
            }
        }
        let mut image = Image::raw(final_image);

        // 6) Print the final image
        // println!("Final image:");
        // println!("{}", image);
        
        // 7) Setup the sea monster
        let sea_monster = vec![
            "..................#.",
            "#....##....##....###",
            ".#..#..#..#..#..#...",
        ].iter().map(|line| line.chars().map(|c| Pixel::from(c)).collect::<Vec<_>>()).collect::<Vec<_>>();
        let sea_monster_coordinates = sea_monster.iter().enumerate().flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, pixel)| {
                if *pixel == Pixel::Hash {
                    Some((x as i32, y as i32))
                } else {
                    None
                }
            })
        }).collect::<HashSet<_>>();

        // 8) Find the sea monsters without rotating the image
        for _ in 0..4 {
            if let Some(num_non_monster_tiles) = image.count_non_monster_tiles(&sea_monster_coordinates) {
                return num_non_monster_tiles;
            }

            let mut image_copy = image.clone();
            image_copy.flip_horizontal();
            if let Some(num_non_monster_tiles) = image_copy.count_non_monster_tiles(&sea_monster_coordinates) {
                return num_non_monster_tiles;
            }

            image_copy = image.clone();
            image_copy.flip_vertical();
            if let Some(num_non_monster_tiles) = image_copy.count_non_monster_tiles(&sea_monster_coordinates) {
                return num_non_monster_tiles;
            }

            image.rotate();
        }

        0
    }

    pub fn solve(&mut self) {
        println!("========= DAY 20 ========");
        print!("Solving part 1: ");
        io::stdout().flush().unwrap();

        let start = std::time::Instant::now();
        let part1 = self.part1();
        let part1_time = start.elapsed();
        println!("{:?} (took {:?})", part1, part1_time);

        print!("Solving part 2: ");
        io::stdout().flush().unwrap();
        let start = std::time::Instant::now();
        let part2 = self.part2();
        let part2_time = start.elapsed();
        println!("{:?} (took {:?})", part2, part2_time);
        println!();
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]

enum Pixel {
    Dot,
    Hash,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Direction {
    Top,
    Right,
    Bottom,
    Left,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::Top => Direction::Bottom,
            Direction::Right => Direction::Left,
            Direction::Bottom => Direction::Top,
            Direction::Left => Direction::Right,
        }
    }

    fn to_delta(&self) -> (i32, i32) {
        match self {
            Direction::Top => (0, -1),
            Direction::Right => (1, 0),
            Direction::Bottom => (0, 1),
            Direction::Left => (-1, 0),
        }
    }
}

impl Display for Pixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pixel::Dot => write!(f, "."),
            Pixel::Hash => write!(f, "#"),
        }
    }
}

impl From<char> for Pixel {
    fn from(c: char) -> Self {
        match c {
            '.' => Pixel::Dot,
            '#' => Pixel::Hash,
            _ => panic!("Invalid tile character {}", c),
        }
    }
}

#[derive(Clone)]
struct Image {
    id: usize,
    contents: Vec<Vec<Pixel>>,
    borders: HashMap<Vec<Pixel>, (Direction, bool)>, // (Direction, flipped)
    adjacents: Vec<usize>,
}

impl Image {
    fn parse(lines: &[&str]) -> (usize, Self) {
        // Parse the tile ID 
        let tile_id = lines[0]
            .strip_prefix("Tile ")
            .unwrap()
            .strip_suffix(':')
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let mut contents = Vec::with_capacity(TILE_SIZE);
        for line in lines[1..].iter() {
            let mut row = Vec::with_capacity(TILE_SIZE);
            for c in line.chars() {
                row.push(Pixel::from(c));
            }
            contents.push(row);
        }

        // Generate the borders
        let borders = Self::generate_borders(&contents);

        (tile_id, Self {
            id: tile_id,
            contents,
            borders,
            adjacents: vec![],
        })
    }

    fn raw(contents: Vec<Vec<Pixel>>) -> Self {
        Self {
            id: 0,
            contents,
            borders: HashMap::new(),
            adjacents: vec![],
        }
    }

    fn get_hash_coordinates(&self) -> HashSet<(i32, i32)> {
        self.contents.iter().enumerate().flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, pixel)| {
                if *pixel == Pixel::Hash {
                    Some((x as i32, y as i32))
                } else {
                    None
                }
            })
        }).collect()
    }

    fn count_non_monster_tiles(&self, sea_monster: &HashSet<(i32, i32)>) -> Option<usize> {
        let image_hash = self.get_hash_coordinates();
        let size = self.contents.len();
        let num_sea_monster_pixels = sea_monster.len();
        let mut sea_monster_coordinates = HashSet::new();

        for x in 0..size - 20 {
            for y in 0..size - 3 {
                let sea_monster_offseted_coordinates = sea_monster.iter().map(|(dx, dy)| (x as i32 + dx, y as i32 + dy)).collect::<HashSet<_>>();
                if sea_monster_offseted_coordinates.intersection(&image_hash).count() == num_sea_monster_pixels {
                    sea_monster_coordinates.extend(sea_monster_offseted_coordinates);
                }
            }
        }

        if sea_monster_coordinates.is_empty() {
            None
        } else {
            let total_hashes = image_hash.len();
            Some(total_hashes - sea_monster_coordinates.len())
        }
    }

    fn generate_borders(contents: &Vec<Vec<Pixel>>) -> HashMap<Vec<Pixel>, (Direction, bool)> {
        let mut borders = HashMap::new();
        
        // The order of the items in the border must be so that only by rotating the tile we can get a "direct" border to match
        let top_border = contents[0].clone();
        let right_border: Vec<_> = contents.iter().map(|row| row[contents.len() - 1]).collect();
        let bottom_border = contents[contents.len() - 1].iter().rev().cloned().collect::<Vec<_>>();
        let left_border: Vec<_> = contents.iter().map(|row| row[0]).rev().collect::<Vec<_>>();

        borders.insert(top_border.clone(), (Direction::Top, false));
        borders.insert(top_border.iter().rev().cloned().collect(), (Direction::Top, true));
        borders.insert(bottom_border.clone(), (Direction::Bottom, false));
        borders.insert(bottom_border.iter().rev().cloned().collect(), (Direction::Bottom, true));
        borders.insert(left_border.clone(), (Direction::Left, false));
        borders.insert(left_border.iter().rev().cloned().collect(), (Direction::Left, true));
        borders.insert(right_border.clone(), (Direction::Right, false));
        borders.insert(right_border.iter().rev().cloned().collect(), (Direction::Right, true));

        borders
    }

    fn compute_adjacents(&mut self, tiles: &HashMap<usize, Image>) {
        for (_, tile) in tiles.iter() {
            if self.id == tile.id {
                continue;
            }

            if self.common_border(tile).is_some() {
                self.adjacents.push(tile.id);
            }
        }
    }

    fn common_border(&self, other: &Image) -> Option<Vec<Pixel>> {
        for (border, _) in other.borders.iter() {
            if let Some((_, false)) = self.borders.get(border) {
                return Some(border.clone());
            }
        }
        None
    }

    fn identify_border(&self, border: &Vec<Pixel>) -> (Direction, bool) {
        *self.borders.get(border).unwrap()
    }

    fn rotate(&mut self) {
        // Reverse the matrix
        let n = self.contents.len();
        for i in 0..n {
            self.contents[i].reverse();
        }

        // Transpose the matrix
        for i in 0..n {
            for j in i..n {
                let temp = self.contents[i][j];
                self.contents[i][j] = self.contents[j][i];
                self.contents[j][i] = temp;
            }
        }

        // Update the borders
        self.borders = Self::generate_borders(&self.contents);
    }

    fn flip_horizontal(&mut self) {
        for row in self.contents.iter_mut() {
            row.reverse();
        }

        // Update the borders
        self.borders = Self::generate_borders(&self.contents);
    }

    fn flip_vertical(&mut self) {
        self.contents.reverse();

        // Update the borders
        self.borders = Self::generate_borders(&self.contents);
    }

}

impl Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // writeln!(f, "Tile {}", self.id)?;
        for row in self.contents.iter() {
            for pixel in row.iter() {
                write!(f, "{}", pixel)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Debug for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Tile {}:", self.id)?;
        writeln!(f, "Adjacents: {:?}", self.adjacents)?;
        for row in self.contents.iter() {
            for pixel in row.iter() {
                write!(f, "{}", pixel)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}