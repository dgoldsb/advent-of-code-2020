use aoc::parse_blocks;
use queue::Queue;
use std::str::FromStr;

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

fn part_a(deck_a: &Deck, deck_b: &Deck) -> usize {
    let mut winner: Deck = play_game(deck_a, deck_b);
    let mut counter = 0;
    loop {
        let multiplier = winner.cards.len();
        counter += winner.cards.dequeue().unwrap() * multiplier;

        if winner.cards.is_empty() {
            break;
        }
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
        part_a(&inputs.get(0).unwrap(), &inputs.get(1).unwrap())
    );
    //println!("B: {}", part_b(&inputs));
}
