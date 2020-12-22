use aoc::parse_blocks;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

const PIXEL: char = '#';

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up(),
    Down(),
    Left(),
    Right(),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Pixel {
    x: isize,
    y: isize,
}

#[derive(Clone, Debug, Eq)]
struct Picture {
    id: Option<usize>,
    pixels: HashSet<Pixel>,
}

impl PartialEq for Picture {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Picture {
    fn get_min_x(&self) -> isize {
        return self.pixels.iter().map(|t| t.x).min().unwrap();
    }

    fn get_min_y(&self) -> isize {
        return self.pixels.iter().map(|t| t.y).min().unwrap();
    }

    fn get_max_x(&self) -> isize {
        return self.pixels.iter().map(|t| t.x).max().unwrap();
    }

    fn get_max_y(&self) -> isize {
        return self.pixels.iter().map(|t| t.y).max().unwrap();
    }

    fn flip_x(&self) -> Picture {
        let max_x = self.get_max_x();
        return Picture {
            id: self.id,
            pixels: self
                .pixels
                .iter()
                .map(|p| Pixel {
                    x: max_x - p.x,
                    y: p.y,
                })
                .collect(),
        };
    }

    fn flip_y(&self) -> Picture {
        let max_y = self.get_max_y();
        return Picture {
            id: self.id,
            pixels: self
                .pixels
                .iter()
                .map(|p| Pixel {
                    x: p.x,
                    y: max_y - p.y,
                })
                .collect(),
        };
    }

    fn rotate(&self) -> Picture {
        let max_x = self.get_max_x();
        return Picture {
            id: self.id,
            pixels: self
                .pixels
                .iter()
                .map(|p| Pixel {
                    x: p.y,
                    y: max_x - p.x,
                })
                .collect(),
        };
    }

    fn get_border(&self, direction: Direction) -> HashSet<isize> {
        let max_x = self.get_max_x();
        let max_y = self.get_max_y();

        match direction {
            Direction::Up() => self
                .pixels
                .iter()
                .filter(|&t| t.y == 0)
                .map(|t| t.x)
                .collect(),
            Direction::Down() => self
                .pixels
                .iter()
                .filter(|&t| t.y == max_y)
                .map(|t| t.x)
                .collect(),
            Direction::Left() => self
                .pixels
                .iter()
                .filter(|&t| t.x == 0)
                .map(|t| t.y)
                .collect(),
            Direction::Right() => self
                .pixels
                .iter()
                .filter(|&t| t.x == max_x)
                .map(|t| t.y)
                .collect(),
        }
    }

    fn get_permutations(&self) -> Vec<Picture> {
        vec![
            self.clone(),
            self.flip_x(),
            self.flip_y(),
            self.rotate(),
            self.flip_x().flip_y(),
            self.flip_x().rotate(),
            self.flip_y().rotate(),
            self.flip_x().flip_y().rotate(),
        ]
    }

    fn is_neighbour(&self, other: &Picture, direction: Direction) -> bool {
        match direction {
            Direction::Up() => {
                self.get_border(Direction::Up()) == other.get_border(Direction::Down())
            }
            Direction::Down() => {
                self.get_border(Direction::Down()) == other.get_border(Direction::Up())
            }
            Direction::Left() => {
                self.get_border(Direction::Left()) == other.get_border(Direction::Right())
            }
            Direction::Right() => {
                self.get_border(Direction::Right()) == other.get_border(Direction::Left())
            }
        }
    }

    fn to_string(&self) -> String {
        let mut string = "".to_string();
        for y in self.get_min_y()..=self.get_max_y() {
            for x in self.get_min_x()..=self.get_max_x() {
                if self.pixels.contains(&Pixel { x, y }) {
                    string += &"#";
                } else {
                    string += &".";
                }
            }
            string += &"\n";
        }
        return string;
    }
}

impl FromStr for Picture {
    type Err = ();

    fn from_str(input: &str) -> Result<Picture, Self::Err> {
        let input_string = input.to_string();
        let mut input_iter = input_string.split("\n");

        // Parse the identifier.
        let id: usize;
        match input_iter
            .next()
            .unwrap()
            .replace("Tile ", "")
            .replace(":", "")
            .parse()
        {
            Ok(i) => id = i,
            Err(_) => return Err(()),
        };

        // Parse the pixels.
        let mut pixels = HashSet::new();
        for (y, line) in input_iter.enumerate() {
            for (x, chr) in line.chars().enumerate() {
                if chr == PIXEL {
                    pixels.insert(Pixel {
                        x: x as isize,
                        y: y as isize,
                    });
                }
            }
        }

        return Ok(Picture {
            id: Some(id),
            pixels,
        });
    }
}

fn count_neighbours(tile: &Picture, others: &Vec<Picture>) -> usize {
    let mut count = 0;
    for other in others {
        if other.id == tile.id {
            continue;
        }

        let mut found = false;
        for permutation in other.get_permutations() {
            for direction in vec![
                Direction::Down(),
                Direction::Up(),
                Direction::Right(),
                Direction::Left(),
            ] {
                if tile.is_neighbour(&permutation, direction) && !found {
                    count += 1;
                    found = true;
                }
            }
        }
    }
    return count;
}

fn part_a(tiles: &Vec<Picture>) -> usize {
    return tiles
        .iter()
        .filter(|&t| count_neighbours(t, tiles) == 2)
        .map(|t| t.id.unwrap())
        .product();
}

fn create_composite(tiles: &Vec<Picture>) -> Picture {
    // Keep the placed tiles in a hashmap.
    let mut placed: HashMap<(isize, isize), Picture> = HashMap::new();
    let mut unplaced = tiles.clone();

    // Insert a starting piece.
    placed.insert((0, 0), tiles.iter().next().unwrap().clone());
    unplaced.retain(|x| x != placed.values().next().unwrap());

    loop {
        for t in unplaced.clone() {
            for (k, v) in placed.clone().iter() {
                for candidate in t.get_permutations() {
                    for border in vec![
                        Direction::Down(),
                        Direction::Up(),
                        Direction::Right(),
                        Direction::Left(),
                    ] {
                        if v.is_neighbour(&candidate, border) {
                            let dx;
                            let dy;

                            match border {
                                Direction::Down() => {
                                    dx = 0;
                                    dy = 1;
                                }
                                Direction::Up() => {
                                    dx = 0;
                                    dy = -1;
                                }
                                Direction::Left() => {
                                    dx = -1;
                                    dy = 0;
                                }
                                Direction::Right() => {
                                    dx = 1;
                                    dy = 0;
                                }
                            }

                            placed.insert((k.0 + dx, k.1 + dy), candidate.clone());
                            unplaced.retain(|x| x != &t);
                            break;
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
    let mut composite: HashSet<Pixel> = HashSet::new();

    for (k, v) in placed.iter() {
        let max_x = v.get_max_x();
        let max_y = v.get_max_y();
        for t in v.pixels.clone() {
            if (t.x != 0) && (t.y != 0) && (t.x != max_x) && (t.y != max_y) {
                composite.insert(Pixel {
                    x: (k.0 * (max_x - 1)) + t.x,
                    y: (k.1 * (max_y - 1)) + t.y,
                });
            }
        }
    }

    return Picture {
        id: None,
        pixels: composite,
    };
}

fn get_seamonster(dx: isize, dy: isize) -> Picture {
    let mut monster = HashSet::new();
    let monster_strings = vec![
        "                  # ".to_string(),
        "#    ##    ##    ###".to_string(),
        " #  #  #  #  #  #   ".to_string(),
    ];

    for (y, line) in monster_strings.iter().enumerate() {
        for (x, chr) in line.chars().enumerate() {
            if chr == PIXEL {
                monster.insert(Pixel {
                    x: x as isize + dx,
                    y: y as isize + dy - 1,
                });
            }
        }
    }

    return Picture { id: None, pixels: monster };
}

fn count_seamonsters(composite: &Picture) -> usize {
    let mut counter = 0;
    for permutation in composite.get_permutations() {
        let mut found = false;
        for pixel in &permutation.pixels {
            let monster = get_seamonster(pixel.x, pixel.y);

            if monster.pixels.is_subset(&permutation.pixels) {
                counter += 1;
                found = true;
            }
        }

        if found {
            println!(
                "Found some in this orientation:\n\n{}",
                &permutation.to_string()
            );
        }
    }
    return counter;
}

fn part_b(tiles: &Vec<Picture>) -> usize {
    // Get the composite image from the tiles.
    let composite = create_composite(tiles);
    println!("There are {} waves/monster bits", composite.pixels.len());

    // Count the number of monsters in the composite image.
    let monster_count = count_seamonsters(&composite);
    println!("Found {} monsters", monster_count);

    // Subtract this number of monsters from the length of the composite image.
    return composite.pixels.len() - (monster_count * get_seamonster(0, 0).pixels.len());
}

fn main() {
    let inputs: Vec<Picture> = parse_blocks()
        .iter()
        .map(|s| Picture::from_str(s))
        .filter(|o| match o {
            Ok(_) => true,
            Err(_) => false,
        })
        .map(|o| o.unwrap())
        .collect();
    println!("Loaded {} tiles", inputs.len());
    println!("A: {}", part_a(&inputs));
    println!("B: {}", part_b(&inputs));
}
