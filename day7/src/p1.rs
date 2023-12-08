use std::fs::read_to_string;

type Bid = usize;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
enum Class {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Hand {
    class: Class,
    orig_hand: [u8; 5],
}

fn map_char(c: char) -> usize {
    match c {
        '2' => 0,
        '3' => 1,
        '4' => 2,
        '5' => 3,
        '6' => 4,
        '7' => 5,
        '8' => 6,
        '9' => 7,
        'T' => 8,
        'J' => 9,
        'Q' => 10,
        'K' => 11,
        'A' => 12,
        _ => panic!(),
    }
}

impl Hand {
    fn parse(cards: &str) -> Hand {
        let mut class = None;
        let mut orig_hand = [0; 5];
        let mut counts = [0; 13];
        let mut num_pairs = 0;
        let mut num_trips = 0;
        let mut num_quads = 0;
        let mut num_pents = 0;

        for (index, nxt_char) in cards.chars().enumerate() {
            // Higher cards are stronger, so we invert their value
            orig_hand[index] = 12 - map_char(nxt_char) as u8;
            counts[map_char(nxt_char)] += 1;
            match counts[map_char(nxt_char)] {
                2 => num_pairs += 1,
                3 => num_trips += 1,
                4 => num_quads += 1,
                5 => num_pents += 1,
                _ => (),
            };
        }

        if num_pents == 1 {
            class = Some(Class::FiveOfAKind);
        } else if num_quads == 1 {
            class = Some(Class::FourOfAKind);
        } else if num_trips == 1 && num_pairs == 2 { // Need to 2 pairs, one is the trip
            class = Some(Class::FullHouse);
        } else if num_trips == 1 {
            class = Some(Class::ThreeOfAKind);
        } else if num_pairs == 2 {
            class = Some(Class::TwoPair);
        } else if num_pairs == 1 {
            class = Some(Class::OnePair);
        } else {
            class = Some(Class::HighCard);
        }

        return Hand {
            class: class.unwrap(),
            orig_hand,
        };
    }
}

fn parse_hand_and_bid(s: &str) -> (Hand, Bid) {
    let h = Hand::parse(&s[0..5]);
    let b = s[6..].parse().unwrap();
    return (h, b);
}

pub fn main() {
    let mut hands: Vec<_> = read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(parse_hand_and_bid)
        .collect();

    hands.sort_by(|(x, _), (y, _)| y.cmp(x));

    let mut total_winnings = 0;
    for (idx, hand) in hands.iter().enumerate() {
        total_winnings += hand.1 * (idx + 1);
    }

    println!("P1: {total_winnings}");
}
