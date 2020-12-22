use aoc::parse_blocks;
use lazy_static::lazy_static;
use queue::Queue;
use std::collections::HashSet;
use std::str::FromStr;
use std::sync::Mutex;

lazy_static! {
    static ref MEMORY: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
}

#[derive(Clone, Debug)]
struct Deck {
    player: String,
    cards: Queue<usize>,
}

impl FromStr for Deck {
    type Err = ();

    fn from_str(input: &str) -> Result<Deck, Self::Err> {
        let input_string = input.to_string();
        let mut input_iter = input_string.split("\n");

        // Parse the player.
        let player: String = input_iter.next().unwrap().replace(":", "").to_string();

        // Parse the cards.
        let mut cards: Queue<usize> = Queue::new();
        for c in input_iter {
            let card: usize;
            match c.parse::<usize>() {
                Ok(u) => card = u,
                Err(_) => continue,
            }
            cards.queue(card).unwrap();
        }

        return Ok(Deck { player, cards });
    }
}

fn play_game(deck_a: &Deck, deck_b: &Deck) -> Deck {
    let mut a = deck_a.clone();
    let mut b = deck_b.clone();

    loop {
        let card_a = a.cards.dequeue().unwrap();
        let card_b = b.cards.dequeue().unwrap();

        if card_a > card_b {
            a.cards.queue(card_a).unwrap();
            a.cards.queue(card_b).unwrap();
        } else if card_a < card_b {
            b.cards.queue(card_b).unwrap();
            b.cards.queue(card_a).unwrap();
        }

        if a.cards.is_empty() {
            return b;
        }

        if b.cards.is_empty() {
            return a;
        }
    }
}

fn stringify_state(deck_a: &Deck, deck_b: &Deck) -> String {
    let mut state = "".to_string();

    state += &deck_a.player;

    let mut cards = deck_a.cards.clone();
    while let Some(card) = cards.dequeue() {
        state += ",";
        state += &card.to_string();
    }

    state += &deck_b.player;

    let mut cards = deck_b.cards.clone();
    while let Some(card) = cards.dequeue() {
        state += ",";
        state += &card.to_string();
    }

    return state;
}

fn play_recursive_game(deck_a: &Deck, deck_b: &Deck) -> Result<Deck, Deck> {
    let mut a = deck_a.clone();
    let mut b = deck_b.clone();

    loop {
        // Check for instant player 1 win if we are in a loop.
        let state = stringify_state(&a, &b);
        match MEMORY.lock() {
            Ok(mut m) => {
                if m.contains(&state) {
                    println!("Found state {} before!", &state);
                    return Err(a);
                }
                m.insert(state);
            }
            Err(_) => panic!("Could not lock memory..."),
        }

        let card_a = a.cards.dequeue().unwrap();
        let card_b = b.cards.dequeue().unwrap();

        let winner: String;
        if (card_a <= a.cards.len()) && (card_b <= b.cards.len()) {
            match play_recursive_game(&a, &b) {
                Ok(d) => winner = d.player,
                Err(d) => {
                    return Err(d);
                },
            }
        } else {
            if card_a > card_b {
                winner = a.player.clone();
            } else {
                winner = b.player.clone();
            }
        }

        if winner == a.player {
            a.cards.queue(card_a).unwrap();
            a.cards.queue(card_b).unwrap();
        } else if winner == b.player {
            b.cards.queue(card_b).unwrap();
            b.cards.queue(card_a).unwrap();
        }

        if a.cards.is_empty() {
            return Ok(b);
        }

        if b.cards.is_empty() {
            return Ok(a);
        }
    }
}

fn solve(deck_a: &Deck, deck_b: &Deck, part_a: &bool) -> usize {
    let mut winner: Deck;
    if *part_a {
        winner = play_game(deck_a, deck_b);
    } else {
        winner = match play_recursive_game(deck_a, deck_b) {
            Ok(d) => d,
            Err(d) => d,
        };
    }

    let mut counter = 0;
    while let Some(card) = winner.cards.dequeue() {
        let multiplier = winner.cards.len() + 1;
        counter += card * multiplier;
    }
    return counter;
}

fn main() {
    let inputs: Vec<Deck> = parse_blocks()
        .iter()
        .map(|l| Deck::from_str(l).unwrap())
        .collect();

    println!(
        "B: {}",
        solve(&inputs.get(0).unwrap(), &inputs.get(1).unwrap(), &false)
    );
    println!(
        "A: {}",
        solve(&inputs.get(0).unwrap(), &inputs.get(1).unwrap(), &true)
    );
}

// 7248 is too low!
