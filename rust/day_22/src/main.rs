use aoc::parse_blocks;
use std::collections::{HashSet, VecDeque};
use std::str::FromStr;

#[derive(Clone, Debug)]
struct Deck {
    player: String,
    cards: VecDeque<usize>,
}

impl FromStr for Deck {
    type Err = ();

    fn from_str(input: &str) -> Result<Deck, Self::Err> {
        let input_string = input.to_string();
        let mut input_iter = input_string.split("\n");

        // Parse the player.
        let player: String = input_iter.next().unwrap().replace(":", "").to_string();

        // Parse the cards.
        let mut cards: VecDeque<usize> = VecDeque::new();
        for c in input_iter {
            let card: usize;
            match c.parse::<usize>() {
                Ok(u) => card = u,
                Err(_) => continue,
            }
            cards.push_back(card);
        }

        return Ok(Deck { player, cards });
    }
}

fn play_game(deck_a: &Deck, deck_b: &Deck) -> Deck {
    let mut a = deck_a.clone();
    let mut b = deck_b.clone();

    loop {
        let card_a = a.cards.pop_front().unwrap();
        let card_b = b.cards.pop_front().unwrap();

        if card_a > card_b {
            a.cards.push_back(card_a);
            a.cards.push_back(card_b);
        } else if card_a < card_b {
            b.cards.push_back(card_b);
            b.cards.push_back(card_a);
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
    while let Some(card) = cards.pop_front() {
        state += ",";
        state += &card.to_string();
    }

    state += &deck_b.player;

    let mut cards = deck_b.cards.clone();
    while let Some(card) = cards.pop_front() {
        state += ",";
        state += &card.to_string();
    }

    return state;
}

fn play_recursive_game(deck_a: &Deck, deck_b: &Deck) -> Deck {
    let mut a = deck_a.clone();
    let mut b = deck_b.clone();
    let mut h = HashSet::new();

    loop {
        // Check for instant player 1 win if we are in a loop.
        let state = stringify_state(&a, &b);
        if h.contains(&state) {
            return a.clone();
        }
        h.insert(state);

        // Draw cards.
        let card_a = a.cards.pop_front().unwrap();
        let card_b = b.cards.pop_front().unwrap();

        // Determine the winner.
        let winner: String;
        if (card_a <= a.cards.len()) && (card_b <= b.cards.len()) {
            let mut a_trunc = a.clone();
            for _ in card_a..a.cards.len() {
                a_trunc.cards.pop_back();
            }

            let mut b_trunc = b.clone();
            for _ in card_b..b.cards.len() {
                b_trunc.cards.pop_back();
            }

            winner = play_recursive_game(&a_trunc, &b_trunc).player
        } else {
            if card_a > card_b {
                winner = a.player.clone();
            } else {
                winner = b.player.clone();
            }
        }

        // Assign cards.
        if winner == a.player {
            a.cards.push_back(card_a);
            a.cards.push_back(card_b);
        } else {
            b.cards.push_back(card_b);
            b.cards.push_back(card_a);
        }

        // Check for game end.
        if a.cards.is_empty() {
            return b.clone();
        }
        if b.cards.is_empty() {
            return a.clone();
        }
    }
}

fn solve(deck_a: &Deck, deck_b: &Deck, part_a: &bool) -> usize {
    let mut winner: Deck;
    if *part_a {
        winner = play_game(deck_a, deck_b);
    } else {
        winner = play_recursive_game(deck_a, deck_b);
    }

    let mut counter = 0;
    while let Some(card) = winner.cards.pop_front() {
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
        "A: {}",
        solve(&inputs.get(0).unwrap(), &inputs.get(1).unwrap(), &true)
    );
    println!(
        "B: {}",
        solve(&inputs.get(0).unwrap(), &inputs.get(1).unwrap(), &false)
    );
}
