const INPUT_FILE: &'static str = "input/07.txt";

pub fn a() -> String {
    let input = std::fs::read_to_string(INPUT_FILE).expect("Input should exist");
    a_with_input(&input).to_string()
}

fn a_with_input(input: &str) -> usize {
    let mut hands: Vec<_> = input.lines().map(|line| part_a::parse_line(line)).collect();
    hands.sort();

    let total: usize = hands.iter().enumerate().map(|(rank, (_, bid))| (rank + 1) * bid).sum();

    total
}

pub fn b() -> String {
    let input = std::fs::read_to_string(INPUT_FILE).expect("Input should exist");
    b_with_input(&input).to_string()
}

fn b_with_input(input: &str) -> usize {
    let mut hands: Vec<_> = input.lines().map(|line| part_b::parse_line(line)).collect();
    hands.sort();

    let total: usize = hands.iter().enumerate().map(|(rank, (_, bid))| (rank + 1) * bid).sum();

    total
}


// honestly, not much can be shared between the two ...
mod part_a {
    use std::collections::HashMap;

    #[derive(Eq, PartialEq, Copy, Clone, Ord, PartialOrd, Hash)]
    pub struct TypedHand {
        pub kind: HandType,
        pub cards: [Card; 5]
    }

    #[derive(Eq, PartialEq, Copy, Clone, Ord, PartialOrd, Hash)]
    pub enum HandType {
        // least to highest
        HighCard,
        Pair,
        TwoPair,
        ThreeOfAKind,
        FullHouse,
        FourOfAKind,
        FiveOfAKind
    }

    #[derive(Eq, PartialEq, Copy, Clone, Ord, PartialOrd, Hash)]
    pub enum Card {
        // least to highest
        C2,
        C3,
        C4,
        C5,
        C6,
        C7,
        C8,
        C9,
        T,
        J,
        Q,
        K,
        A
    }

    impl TryFrom<char> for Card {
        type Error = char;

        fn try_from(value: char) -> Result<Self, Self::Error> {
            use Card::*;

            match value {
                '2' => Ok(C2),
                '3' => Ok(C3),
                '4' => Ok(C4),
                '5' => Ok(C5),
                '6' => Ok(C6),
                '7' => Ok(C7),
                '8' => Ok(C8),
                '9' => Ok(C9),
                'T' => Ok(T),
                'J' => Ok(J),
                'Q' => Ok(Q),
                'K' => Ok(K),
                'A' => Ok(A),
                other => Err(other),
            }
        }
    }

    fn mult_map(cards: &[Card]) -> HashMap<Card, usize> {
        let mut out = HashMap::new();

        for c in cards.iter().copied() {
            *out.entry(c).or_insert(0) += 1;
        }

        out
    }

    fn hand_type(cards: [Card; 5]) -> HandType {
        let card_mults = mult_map(&cards);
        let mut orders: Vec<usize> = card_mults.values().copied().collect();
        orders.sort();
        orders.reverse();

        if orders[0] == 5 {
            HandType::FiveOfAKind
        } else if orders[0] == 4 {
            HandType::FourOfAKind
        } else if orders[0] == 3 {
            if orders[1] == 2 {
                HandType::FullHouse
            } else {
                HandType::ThreeOfAKind
            }
        } else if orders[0] == 2 {
            if orders[1] == 2 {
                HandType::TwoPair
            } else {
                HandType::Pair
            }
        } else {
            HandType::HighCard
        }
    }

    pub fn parse_line(input: &str) -> (TypedHand, usize) {
        let mut chars = input.chars();

        fn next_card<T: Iterator<Item=char>>(chars: &mut T) -> Card {
            chars.next().unwrap().try_into().unwrap()
        }

        let cards: [Card; 5] = [next_card(&mut chars), next_card(&mut chars), next_card(&mut chars), next_card(&mut chars), next_card(&mut chars), ];
        let kind: HandType = hand_type(cards);

        assert_eq!(chars.next(), Some(' '));

        let remainder: String = chars.collect();

        let bid = remainder.parse::<usize>().unwrap();

        (TypedHand { kind, cards }, bid)
    }
}


mod part_b {
    use std::collections::HashMap;

    #[derive(Eq, PartialEq, Copy, Clone, Ord, PartialOrd, Hash)]
    pub struct TypedHand {
        pub kind: HandType,
        pub cards: [Card; 5]
    }

    #[derive(Eq, PartialEq, Copy, Clone, Ord, PartialOrd, Hash, Debug)]
    pub enum HandType {
        // least to highest
        HighCard,
        Pair,
        TwoPair,
        ThreeOfAKind,
        FullHouse,
        FourOfAKind,
        FiveOfAKind
    }

    #[derive(Eq, PartialEq, Copy, Clone, Ord, PartialOrd, Hash)]
    pub enum Card {
        // least to highest; note J is lowest
        // note this is a re-definition because the sort order changed
        J,
        C2,
        C3,
        C4,
        C5,
        C6,
        C7,
        C8,
        C9,
        T,
        Q,
        K,
        A
    }

    impl TryFrom<char> for Card {
        type Error = char;

        fn try_from(value: char) -> Result<Self, Self::Error> {
            match value {
                '2' => Ok(Card::C2),
                '3' => Ok(Card::C3),
                '4' => Ok(Card::C4),
                '5' => Ok(Card::C5),
                '6' => Ok(Card::C6),
                '7' => Ok(Card::C7),
                '8' => Ok(Card::C8),
                '9' => Ok(Card::C9),
                'T' => Ok(Card::T),
                'J' => Ok(Card::J),
                'Q' => Ok(Card::Q),
                'K' => Ok(Card::K),
                'A' => Ok(Card::A),
                other => Err(other),
            }
        }
    }

    fn mult_map(cards: &[Card]) -> HashMap<Card, usize> {
        let mut out = HashMap::new();

        for c in cards.iter().copied() {
            *out.entry(c).or_insert(0) += 1;
        }

        out
    }

    fn hand_type(cards: [Card; 5]) -> HandType {
        let card_mults = mult_map(&cards);
        let mut orders: Vec<usize> = card_mults.iter().filter(|(&c, _)| c != Card::J).map(|(_c, num)| *num).collect();
        orders.sort();
        orders.reverse();

        let num_jokers = card_mults.get(&Card::J).copied().unwrap_or(0);

        if num_jokers == 5 || num_jokers == 4 {
            // if we have 4 or 5 wilds, it's a 5 of a kind, simple
            HandType::FiveOfAKind
        } else if num_jokers == 3 {
            // if we have 3 wilds, and a pair, then it's a five of a kind; otherwise a four of a kind
            if orders[0] ==2 {
                HandType::FiveOfAKind
            } else {
                HandType::FourOfAKind
            }
        } else if num_jokers == 2 {
            if orders[0] == 3 {
                // if we have 3 anyway, we can wild to 5
                HandType::FiveOfAKind
            } else if orders[0] == 2 {
                // if we have a pair, we can wild to 4, but can't get to 5, so four is best
                HandType::FourOfAKind
            } else {
                // if our regular cards are unmatched, we can two 2 pair or three of a kind;
                // the latter is better
                HandType::ThreeOfAKind
            }
        } else if num_jokers == 1 {
            if orders[0] >= 4 {
                HandType::FiveOfAKind
            } else if orders[0] == 3 {
                HandType::FourOfAKind
            } else if orders[0] == 2 {
                if orders[1] == 2 {
                    HandType::FullHouse
                } else {
                    HandType::ThreeOfAKind
                }
            } else {
                HandType::Pair
            }
        } else {
            // otherwise we can just do the regular thing with no wilds
            if orders[0] == 5 {
                HandType::FiveOfAKind
            } else if orders[0] == 4 {
                HandType::FourOfAKind
            } else if orders[0] == 3 {
                if orders[1] == 2 {
                    HandType::FullHouse
                } else {
                    HandType::ThreeOfAKind
                }
            } else if orders[0] == 2 {
                if orders[1] == 2 {
                    HandType::TwoPair
                } else {
                    HandType::Pair
                }
            } else {
                HandType::HighCard
            }
        }
    }

    pub fn parse_hand(chars: [char; 5]) -> TypedHand {
        let cards: [Card; 5] = chars.map(|c| c.try_into().unwrap());
        let kind = hand_type(cards);

        TypedHand { kind, cards }
    }

    pub fn parse_line(input: &str) -> (TypedHand, usize) {
        let mut chars = input.chars();

        fn next_card<T: Iterator<Item=char>>(chars: &mut T) -> Card {
            chars.next().unwrap().try_into().unwrap()
        }

        let hand_chars = [chars.next().unwrap(), chars.next().unwrap(), chars.next().unwrap(), chars.next().unwrap(), chars.next().unwrap(), ];
        let typed_hand = parse_hand(hand_chars);

        assert_eq!(chars.next(), Some(' '));

        let remainder: String = chars.collect();

        let bid = remainder.parse::<usize>().unwrap();

        (typed_hand, bid)
    }
}

#[cfg(test)]
mod tests {
    use super::part_b::HandType as HandTypeB;
    use super::part_b::parse_hand as parse_b;
    use super::*;

    const SAMPLE_A: &'static str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn sample_a() {
        assert_eq!(a_with_input(SAMPLE_A), 6440);
    }

    #[test]
    fn sample_b() {
        assert_eq!(b_with_input(SAMPLE_A), 5905);
    }

    fn hand_type_test_b(input: &str, exp: HandTypeB) {
        let chars: Vec<char> = input.chars().collect();
        let chars: [char; 5] = chars.try_into().unwrap();

        let act = parse_b(chars).kind;

        assert_eq!(exp, act, "Input {input} should yield {exp:?} but got {act:?}");
    }

    #[test]
    fn hand_type_tests() {
        use HandTypeB::*;

        for (input, exp) in [
            ("7J53Q", Pair),
            ("92JTJ", ThreeOfAKind),
            ("AAAAK", FourOfAKind),
            ("J8JJJ", FiveOfAKind),
            ("666JJ", FiveOfAKind),
            ("656JJ", FourOfAKind),
            ("JQK38", Pair),
            ("JKQ2Q", ThreeOfAKind)
        ] {
            hand_type_test_b(input, exp)
        }
    }
}
