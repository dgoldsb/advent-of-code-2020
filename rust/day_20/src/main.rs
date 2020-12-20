use aoc::parse_blocks;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

const PIXELS: usize = 10;
const TREE: char = '#';

#[derive(Debug, Eq, Hash, PartialEq)]
struct Tree {
    x: usize,
    y: usize,
}

#[derive(Debug, Eq)]
struct Tile {
    id: usize,
    trees: HashSet<Tree>,
}

impl Hash for Tile {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Tile {
    fn flip_border(border: &HashSet<usize>) -> HashSet<usize> {
        return border.iter().map(|v| PIXELS - 1 - *v).collect();
    }

    fn get_borders(&self) -> Vec<HashSet<usize>> {
        return vec![
            self.trees
                .iter()
                .filter(|&t| t.x == 0)
                .map(|t| t.y)
                .collect(),
            self.trees
                .iter()
                .filter(|&t| t.x == (PIXELS - 1))
                .map(|t| t.y)
                .collect(),
            self.trees
                .iter()
                .filter(|&t| t.y == 0)
                .map(|t| t.x)
                .collect(),
            self.trees
                .iter()
                .filter(|&t| t.y == (PIXELS - 1))
                .map(|t| t.x)
                .collect(),
        ];
    }

    fn neighbours(&self, other: &Tile) -> bool {
        // Self is not a neighbor.
        if self == other {
            return false;
        }

        for sb in self.get_borders() {
            for ob in other.get_borders() {
                if (sb == ob) || (sb == Tile::flip_border(&ob)) {
                    return true;
                }
            }
        }

        return false;
    }
}

fn parse_inputs() -> HashSet<Tile> {
    let mut tiles = HashSet::new();

    for block in parse_blocks() {
        let mut block_iter = block.iter();
        let id: usize = match block_iter
            .next()
            .unwrap()
            .replace("Tile ", "")
            .replace(":", "")
            .parse()
        {
            Ok(i) => i,
            Err(_) => continue,
        };

        let mut trees = HashSet::new();
        for (x, line) in block_iter.enumerate() {
            for (y, chr) in line.chars().enumerate() {
                if chr == TREE {
                    trees.insert(Tree { x, y });
                }
            }
        }
        tiles.insert(Tile { id, trees });
    }
    return tiles;
}

fn total_neighbours(tile: &Tile, others: &HashSet<Tile>) -> usize {
    return others.iter().filter(|&o| tile.neighbours(o)).count();
}

fn part_a(tiles: &HashSet<Tile>) -> usize {
    return tiles
        .iter()
        .filter(|&t| total_neighbours(t, tiles) == 2)
        .map(|t| t.id)
        .product();
}

fn main() {
    let inputs = parse_inputs();
    println!("Loaded {} tiles", inputs.len());
    println!("A: {}", part_a(&inputs));
}
