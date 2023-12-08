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
        'J' => 0,
        '2' => 1,
        '3' => 2,
        '4' => 3,
        '5' => 4,
        '6' => 5,
        '7' => 6,
        '8' => 7,
        '9' => 8,
        'T' => 9,
        'Q' => 10,
        'K' => 11,
        'A' => 12,
        _ => panic!(),
    }
}

impl Hand {
    fn parse(cards: &str) -> Hand {
        #[allow(unused_assignments)]
        let mut class = None;
        let mut orig_hand = [0; 5];
        let mut counts = [0; 13];
        let mut num_pairs = 0;
        let mut num_trips = 0;

        for (index, nxt_char) in cards.chars().enumerate() {
            // Higher cards are stronger, so we invert their value
            orig_hand[index] = match map_char(nxt_char) {
                n @ 1.. => 12 - n,
                0 => 13, // jokers are weak AF
                _ => panic!()
            } as u8;
            counts[map_char(nxt_char)] += 1;
            match counts[map_char(nxt_char)] {
                2 => num_pairs += 1,
                3 => num_trips += 1,
                _ => (),
            };
        }

        'finish: {
            for i in 1..13 {
                if counts[i] == 5 || counts[0] + counts[i] == 5 {
                    class = Some(Class::FiveOfAKind);
                    break 'finish;
                }
            }

            for i in 1..13 {
                if counts[i] == 4 || counts[0] + counts[i] == 4 {
                    class = Some(Class::FourOfAKind);
                    break 'finish;
                }
            }

            // Only full house cases:
            // - a. XXXYY
            // - b. XXJYY
            // - c. XJJYY
            // All other configurations would result in 4-of-a-kind or better.
            if num_pairs > 1 {
                // Case a
                if num_pairs == 2 && num_trips == 1 {
                    // Jokers must be gone here.
                    assert!(counts[0] == 0, "{}", cards);
                    class = Some(Class::FullHouse);
                    break 'finish;
                }

                // Case b
                if num_pairs == 2 && num_trips == 0 && counts[0] == 1 {
                    class = Some(Class::FullHouse);
                    break 'finish;
                }

                // Case c
                if num_pairs == 1 && counts[0] == 2 {
                    class = Some(Class::FullHouse);
                    break 'finish;
                }
            }

            for i in 1..13 {
                if counts[i] == 3 {
                    // Jokers must be gone here.
                    assert!(counts[0] == 0, "{}", cards);
                    class = Some(Class::ThreeOfAKind);
                    break 'finish;
                }

                if counts[i] + counts[0] == 3 {
                    class = Some(Class::ThreeOfAKind);
                    break 'finish;
                }
            }

            // Only two pair cases:
            // - a. XXYYZ
            // - b. XJYYZ
            // XJJYZ (is three of a kind)
            // XXYJZ (is same as a)
            // XXYYJ (is full house)

            // Exactly 2 here means two pair, any J would be full house
            // Case a
            if num_pairs == 2 {
                // Jokers must be gone here.
                assert!(counts[0] == 0, "{}", cards);
                class = Some(Class::TwoPair);
                break 'finish;
            }

            // Case b
            if num_pairs == 1 && counts[0] == 1 {
                class = Some(Class::TwoPair);
                break 'finish;
            }

            // No more two pair
            if num_pairs == 1 {
                // Jokers must be gone here.
                assert!(counts[0] == 0, "{}", cards);
                class = Some(Class::OnePair);
                break 'finish;
            }

            if counts[0] == 1 {
                class = Some(Class::OnePair);
                break 'finish;
            }

            // All jokers eliminated.
            assert!(counts[0] == 0, "{}", cards);
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

    println!("P2: {total_winnings}");
}
