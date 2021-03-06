use aoc::parse_lines;
use std::collections::HashSet;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Block {
    x: isize,
    y: isize,
    z: isize,
    w: isize,
}

impl Block {
    fn neighbours(&self, part_a: &bool) -> HashSet<Block> {
        let mut set = HashSet::new();
        for dx in -1..=1 {
            for dy in -1..=1 {
                for dz in -1..=1 {
                    if *part_a {
                        set.insert(Block {
                            x: self.x + dx,
                            y: self.y + dy,
                            z: self.z + dz,
                            w: 0,
                        });
                    } else {
                        for dw in -1..=1 {
                            set.insert(Block {
                                x: self.x + dx,
                                y: self.y + dy,
                                z: self.z + dz,
                                w: self.w + dw,
                            });
                        }
                    }
                }
            }
        }
        return set;
    }

    fn becomes_active(&self, previous_state: &HashSet<Block>, part_a: &bool) -> bool {
        let active_neighbours = self
            .neighbours(part_a)
            .iter()
            .filter(|&b| b != self)
            .filter(|&b| previous_state.contains(b))
            .count();
        return (previous_state.contains(self)
            && (active_neighbours == 2 || active_neighbours == 3))
            || active_neighbours == 3;
    }
}

fn parse_inputs() -> HashSet<Block> {
    let mut starting_state = HashSet::new();

    for (x, line) in parse_lines().iter().enumerate() {
        for (y, char) in line.chars().enumerate() {
            if char == '#' {
                starting_state.insert(Block {
                    x: x as isize,
                    y: y as isize,
                    z: 0,
                    w: 0,
                });
            }
        }
    }

    return starting_state;
}

fn do_cycle(previous_state: &HashSet<Block>, part_a: &bool) -> HashSet<Block> {
    let mut new_state = HashSet::new();

    // Get all blocks that could be active next cycle.
    let mut under_consideration: HashSet<Block> = HashSet::new();
    for block in previous_state.iter() {
        under_consideration.extend(block.neighbours(part_a));
    }

    // For each, check if they will be active.
    for block in under_consideration {
        if block.becomes_active(previous_state, part_a) {
            new_state.insert(block);
        }
    }

    return new_state;
}

fn solve(inputs: &HashSet<Block>, part_a: &bool) -> usize {
    let mut current_state = inputs.clone();

    for _ in 0..6 {
        current_state = do_cycle(&current_state, part_a);
    }

    // State only contains active blocks, so return the length.
    return current_state.len();
}

fn main() {
    let inputs = parse_inputs();

    println!("A: {}", solve(&inputs, &true));
    println!("B: {}", solve(&inputs, &false));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let mut set = HashSet::new();
        set.insert(Block {
            x: 0,
            y: 1,
            z: 0,
            w: 0,
        });
        set.insert(Block {
            x: 1,
            y: 2,
            z: 0,
            w: 0,
        });
        set.insert(Block {
            x: 2,
            y: 0,
            z: 0,
            w: 0,
        });
        set.insert(Block {
            x: 2,
            y: 1,
            z: 0,
            w: 0,
        });
        set.insert(Block {
            x: 2,
            y: 2,
            z: 0,
            w: 0,
        });
        assert_eq!(solve(&set, &true), 112);
        assert_eq!(solve(&set, &false), 848);
    }
}
