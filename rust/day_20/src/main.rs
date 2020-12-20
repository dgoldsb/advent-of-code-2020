use aoc::parse_blocks;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

const PIXELS: isize = 10;
const TREE: char = '#';

#[derive(Clone, Copy, Debug)]
enum Border {
    Up(),
    Down(),
    Left(),
    Right(),
}

// I thought that the pictures were of trees in part A, so henceforth these will be tree monsters.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Tree {
    x: isize,
    y: isize,
}

#[derive(Clone, Debug, Eq)]
struct Tile {
    id: isize,
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
    fn fits(&self, other: &Tile, border: Border) -> bool {
        match border {
            Border::Down() => return self.get_borders()[2] == other.get_borders()[3],
            Border::Up() => return self.get_borders()[3] == other.get_borders()[2],
            Border::Left() => return self.get_borders()[0] == other.get_borders()[1],
            Border::Right() => return self.get_borders()[1] == other.get_borders()[0],
        }
    }

    fn flip_border(border: &HashSet<isize>) -> HashSet<isize> {
        return border.iter().map(|v| PIXELS - 1 - *v).collect();
    }

    fn get_all_rotation(&self) -> Vec<Tile> {
        return vec![
            Tile { id: self.id, trees: flip_and_rotate(&self.trees, 0, true) },
            Tile { id: self.id, trees: flip_and_rotate(&self.trees, 1, true) },
            Tile { id: self.id, trees: flip_and_rotate(&self.trees, 2, true) },
            Tile { id: self.id, trees: flip_and_rotate(&self.trees, 3, true) },
            Tile { id: self.id, trees: flip_and_rotate(&self.trees, 0, false) },
            Tile { id: self.id, trees: flip_and_rotate(&self.trees, 1, false) },
            Tile { id: self.id, trees: flip_and_rotate(&self.trees, 2, false) },
            Tile { id: self.id, trees: flip_and_rotate(&self.trees, 3, false) },
        ];
    }

    fn get_borders(&self) -> Vec<HashSet<isize>> {
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
        let id: isize = match block_iter
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
                    trees.insert(Tree {
                        x: x as isize,
                        y: y as isize,
                    });
                }
            }
        }
        tiles.insert(Tile { id, trees });
    }
    return tiles;
}

fn total_neighbours(tile: &Tile, others: &HashSet<Tile>) -> isize {
    return others.iter().filter(|&o| tile.neighbours(o)).count() as isize;
}

fn flip_and_rotate(original: &HashSet<Tree>, ticks: isize, flip: bool) -> HashSet<Tree> {
    let mut new = HashSet::new();

    for tree in original {
        let mut x: isize = tree.x;
        let mut y: isize = tree.y;

        if flip {
            let old_x = x;
            x = y;
            y = old_x;
        }

        // Rotate!
        match ticks {
            0 => {}
            1 => {
                let old_x = x;
                x = y;
                y = -1 * old_x;
            }
            2 => {
                x = -x;
                y = -y;
            }
            3 => {
                let old_x = x;
                x = -1 * y;
                y = old_x;
            }
            _ => panic!("Invalid rotation!"),
        }

        new.insert(Tree { x, y });
    }

    return new;
}

fn create_composite(tiles: &HashSet<Tile>) -> HashSet<Tree> {
    // Keep the placed tiles in a hashmap.
    let mut placed: HashMap<(isize, isize), Tile> = HashMap::new();
    let mut unplaced = tiles.clone();

    // Insert a starting piece.
    placed.insert((0, 0), tiles.iter().next().unwrap().clone());

    loop {
        for t in unplaced.clone() {
            for (k, v) in placed.clone().iter() {
                for candidate in t.get_all_rotation() {
                    for border in vec![
                        Border::Down(),
                        Border::Up(),
                        Border::Right(),
                        Border::Left(),
                    ] {
                        if v.fits(&candidate, border) {
                            let dx;
                            let dy;

                            match border {
                                Border::Down() => {
                                    dx = 0;
                                    dy = -1;
                                }
                                Border::Up() => {
                                    dx = 0;
                                    dy = 1;
                                }
                                Border::Left() => {
                                    dx = -1;
                                    dy = 0;
                                }
                                Border::Right() => {
                                    dx = 1;
                                    dy = 0;
                                }
                            }

                            placed.insert((k.0 + dx, k.1 + dy), candidate.clone());
                            unplaced.remove(&t);
                        }
                    }
                }
            }
        }

        if unplaced.len() == 0 {
            break;
        }
    }

    // Add each tiles to the composite respecting the x and y offset.
    let mut composite: HashSet<Tree> = HashSet::new();

    for (k, v) in placed.iter() {
        for t in v.trees.clone() {
            if (t.x != 0) && (t.y != 0) && (t.x != PIXELS - 1) && (t.y != PIXELS - 1) {
                composite.insert(Tree {
                    x: (k.0 * (PIXELS - 2)) + t.x,
                    y: (k.1 * (PIXELS - 2)) + t.y,
                });
            }
        }
    }

    return composite;
}

fn get_seamonster_hashset(dx: isize, dy: isize) -> HashSet<Tree> {
    let mut monster = HashSet::new();
    let monster_strings = vec![
        "                  # ".to_string(),
        "#    ##    ##    ###".to_string(),
        " #  #  #  #  #  #   ".to_string(),
    ];

    for (x, line) in monster_strings.iter().enumerate() {
        for (y, chr) in line.chars().enumerate() {
            if chr == TREE {
                monster.insert(Tree {
                    x: x as isize + dx,
                    y: y as isize + dy,
                });
            }
        }
    }

    return monster;
}

fn count_seamonsters(composite: &HashSet<Tree>) -> usize {
    // Get all monsters we could reasonably have.
    let mut monsters: Vec<HashSet<Tree>> = Vec::new();
    for dx in -90..90 {
        for dy in -90..90 {
            let monster = get_seamonster_hashset(dx, dy);

            // Add all rotations and flips.
            monsters.push(flip_and_rotate(&monster, 0, true));
            monsters.push(flip_and_rotate(&monster, 1, true));
            monsters.push(flip_and_rotate(&monster, 2, true));
            monsters.push(flip_and_rotate(&monster, 3, true));
            monsters.push(flip_and_rotate(&monster, 0, false));
            monsters.push(flip_and_rotate(&monster, 1, false));
            monsters.push(flip_and_rotate(&monster, 2, false));
            monsters.push(flip_and_rotate(&monster, 3, false));
        }
    }

    // For each starting position, check if there is a seamonster using the subset function.
    let mut counter = 0;
    for monster in monsters {
        if monster.is_subset(composite) {
            counter += 1;
        }
    }

    return counter;
}

fn part_b(tiles: &HashSet<Tile>) -> usize {
    // Get the composite image from the tiles.
    let composite = create_composite(tiles);

    // Count the number of monsters in the composite image.
    let monster_count = count_seamonsters(&composite);

    // Subtract this number of monsters from the length of the composite image.
    return composite.len() - (monster_count * get_seamonster_hashset(0, 0).len());
}

fn part_a(tiles: &HashSet<Tile>) -> isize {
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
    println!("B: {}", part_b(&inputs));
}
