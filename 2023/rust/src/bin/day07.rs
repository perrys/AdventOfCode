use std::{cmp::Ordering, fs};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("USAGE: {} <filename>", args[0]);
        return;
    }
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Couldn't read file {filename}");

    println!("part1 total is {}", part1(contents.as_str()));
    //    println!("part2 total is {}", part2(&contents));
}

fn part1(content: &str) -> usize {
    let mut hands = content
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(Hand::parse)
        .collect::<Vec<_>>();
    hands.sort();
    hands
        .into_iter()
        .enumerate()
        .map(|(idx, hand)| (1 + idx) * hand.bid)
        .sum()
}

#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Debug)]
enum Card {
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    T,
    J,
    Q,
    K,
    A,
}

impl Card {
    fn new(c: char) -> Self {
        match c {
            '2' => Card::N2,
            '3' => Card::N3,
            '4' => Card::N4,
            '5' => Card::N5,
            '6' => Card::N6,
            '7' => Card::N7,
            '8' => Card::N8,
            '9' => Card::N9,
            'T' => Card::T,
            'J' => Card::J,
            'Q' => Card::Q,
            'K' => Card::K,
            'A' => Card::A,
            _ => panic!("unrecognised card {c}"),
        }
    }
}

#[derive(Ord, Eq, PartialOrd, PartialEq, Debug)]
enum HandType {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    fn new(mut cards: [Card; 5]) -> Self {
        cards.sort();
        let mut groups = Vec::<u8>::new();
        let mut size = 1u8;
        for (idx, card) in cards.into_iter().enumerate().skip(1) {
            if card == cards[idx - 1] {
                size += 1;
            } else {
                groups.push(size);
                size = 1;
            }
        }
        groups.push(size);
        groups.sort();
        match groups {
            _ if groups == vec![5] => Self::FiveOfAKind,
            _ if groups == vec![1, 4] => Self::FourOfAKind,
            _ if groups == vec![2, 3] => Self::FullHouse,
            _ if groups == vec![1, 1, 3] => Self::ThreeOfAKind,
            _ if groups == vec![1, 2, 2] => Self::TwoPair,
            _ if groups == vec![1, 1, 1, 2] => Self::Pair,
            _ if groups == vec![1, 1, 1, 1, 1] => Self::HighCard,
            _ => panic!("unrecognized grouping {groups:?}"),
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Hand {
    cards: [Card; 5],
    hand_type: HandType,
    bid: usize,
}

impl Hand {
    fn new(cards: [Card; 5], bid: usize) -> Self {
        let hand_type = HandType::new(cards);
        Self {
            cards,
            hand_type,
            bid,
        }
    }
    fn parse(line: &str) -> Self {
        let (cards, rank) = parse_line(line);
        Self::new(cards, rank)
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let c = self.hand_type.cmp(&other.hand_type);
        match c {
            Ordering::Equal => {
                for (idx, card) in self.cards.iter().enumerate() {
                    let o = card.cmp(&other.cards[idx]);
                    match o {
                        Ordering::Equal => continue,
                        _ => return o,
                    }
                }
                Ordering::Equal
            }
            _ => c,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_line(line: &str) -> ([Card; 5], usize) {
    let mut iter = line.split(' ');
    let cards = iter
        .next()
        .unwrap()
        .chars()
        .map(Card::new)
        .collect::<Vec<_>>()
        .try_into()
        .unwrap_or_else(|_| panic!("didn't find 5 cards in line {line}"));
    let bid = iter
        .next()
        .expect("no bid in line")
        .parse::<usize>()
        .expect("unparsable number");
    (cards, bid)
}

#[allow(non_snake_case)]
#[cfg(test)]
mod test07 {
    use super::*;

    #[test]
    fn GIVEN_pairs_of_cards_WHEN_comparing_THEN_correct_ordering_produced() {
        assert!(Card::N2 < Card::N3);
        assert!(Card::N2 == Card::N2);
        assert!(Card::N3 > Card::N2);
        assert!(Card::N9 < Card::T);
        assert!(Card::K < Card::A);
    }
    #[test]
    fn GIVEN_valid_line_WHEN_parsing_THEN_expected_output_produced() {
        assert_eq!(
            parse_line("32T3K 765"),
            ([Card::N3, Card::N2, Card::T, Card::N3, Card::K], 765)
        );
    }

    #[test]
    fn GIVEN_five_cards_WHEN_constructing_hand_THEN_correct_handtype_assigned() {
        let dotest = |line, expected| {
            let (cards, score) = parse_line(line);
            let hand = Hand::new(cards, score);
            assert_eq!(hand.hand_type, expected);
        };
        dotest("AAAAA 1", HandType::FiveOfAKind);
        dotest("A2AAA 1", HandType::FourOfAKind);
        dotest("2AA2A 1", HandType::FullHouse);
        dotest("AQAKA 1", HandType::ThreeOfAKind);
        dotest("2A52A 1", HandType::TwoPair);
        dotest("2AJ2T 1", HandType::Pair);
        dotest("76543 1", HandType::HighCard);
    }

    #[test]
    fn GIVEN_several_hands_WHEN_ordering_THEN_correct_rules_followed() {
        let dotest = |hand1, hand2, expected_ordering| {
            let hand1 = Hand::parse(hand1);
            let hand2 = Hand::parse(hand2);
            assert_eq!(hand1.cmp(&hand2), expected_ordering);
        };
        dotest("AAAAA 1", "AAAAK 1", Ordering::Greater);
        dotest("AAAAA 1", "AAAAA 1", Ordering::Equal);
        dotest("AAAAK 1", "AAAAA 1", Ordering::Less);
        dotest("AAAAK 1", "AAAAQ 1", Ordering::Greater);
        dotest("KAAAA 1", "KAAAA 1", Ordering::Equal);
        dotest("AA2AA 1", "AATAA 1", Ordering::Less);
    }

    static EXAMPLE_INPUT: &str = r#"
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
"#;

    #[test]
    fn GIVEN_aoc_example_input_WHEN_part1_run_THEN_expected_total_returned() {
        assert_eq!(6440, part1(EXAMPLE_INPUT));
    }
}
