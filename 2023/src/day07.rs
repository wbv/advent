#![cfg(not(doctest))]

//! # Camel Cards
//!
//! Your all-expenses-paid trip turns out to be a one-way, five-minute ride in an airship. (At
//! least it's a cool airship!) It drops you off at the edge of a vast desert and descends back to
//! Island Island.
//!
//! "Did you bring the parts?"
//!
//! You turn around to see an Elf completely covered in white clothing, wearing goggles, and riding
//! a large camel.
//!
//! "Did you bring the parts?" she asks again, louder this time. You aren't sure what parts she's
//! looking for; you're here to figure out why the sand stopped.
//!
//! "The parts! For the sand, yes! Come with me; I will show you." She beckons you onto the camel.
//!
//! After riding a bit across the sands of Desert Island, you can see what look like very large
//! rocks covering half of the horizon. The Elf explains that the rocks are all along the part of
//! Desert Island that is directly above Island Island, making it hard to even get there. Normally,
//! they use big machines to move the rocks and filter the sand, but the machines have broken down
//! because Desert Island recently stopped receiving the parts they need to fix the machines.
//!
//! You've already assumed it'll be your job to figure out why the parts stopped when she asks if
//! you can help. You agree automatically.

use std::collections::BinaryHeap;

use super::*;

/// # Winnings
///
/// Because the journey will take a few days, she offers to teach you the game of Camel Cards.
/// Camel Cards is sort of similar to poker except it's designed to be easier to play while riding
/// a camel.
///
/// In Camel Cards, you get a list of hands, and your goal is to order them based on the strength
/// of each hand. A hand consists of five cards labeled one of A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3,
/// or 2. The relative strength of each card follows this order, where A is the highest and 2 is
/// the lowest.
///
/// Every hand is exactly one type. From strongest to weakest, they are:
///
/// - Five of a kind, where all five cards have the same label: AAAAA
/// - Four of a kind, where four cards have the same label and one card has a different label:
///   AA8AA
/// - Full house, where three cards have the same label, and the remaining two cards share a
///   different label: 23332
/// - Three of a kind, where three cards have the same label, and the remaining two cards are each
///   different from any other card in the hand: TTT98
/// - Two pair, where two cards share one label, two other cards share a second label, and the
///   remaining card has a third label: 23432
/// - One pair, where two cards share one label, and the other three cards have a different label
///   from the pair and each other: A23A4
/// - High card, where all cards' labels are distinct: 23456
///
/// Hands are primarily ordered based on type; for example, every full house is stronger than any
/// three of a kind.
///
/// If two hands have the same type, a second ordering rule takes effect. Start by comparing the
/// first card in each hand. If these cards are different, the hand with the stronger first card is
/// considered stronger. If the first card in each hand have the same label, however, then move on
/// to considering the second card in each hand. If they differ, the hand with the higher second
/// card wins; otherwise, continue with the third card in each hand, then the fourth, then the
/// fifth.
///
/// So, 33332 and 2AAAA are both four of a kind hands, but 33332 is stronger because its first card
/// is stronger. Similarly, 77888 and 77788 are both a full house, but 77888 is stronger because
/// its third card is stronger (and both hands have the same first and second card).
///
/// To play Camel Cards, you are given a list of hands and their corresponding bid (your puzzle
/// input). For example:
///
/// ```
/// 32T3K 765
/// T55J5 684
/// KK677 28
/// KTJJT 220
/// QQQJA 483
/// ```
///
/// This example shows five hands; each hand is followed by its bid amount. Each hand wins an
/// amount equal to its bid multiplied by its rank, where the weakest hand gets rank 1, the
/// second-weakest hand gets rank 2, and so on up to the strongest hand. Because there are five
/// hands in this example, the strongest hand will have rank 5 and its bid will be multiplied by 5.
///
/// So, the first step is to put the hands in order of strength:
///
/// - 32T3K is the only one pair and the other hands are all a stronger type, so it gets rank 1.
/// - KK677 and KTJJT are both two pair. Their first cards both have the same label, but the second
///   card of KK677 is stronger (K vs T), so KTJJT gets rank 2 and KK677 gets rank 3.
/// - T55J5 and QQQJA are both three of a kind. QQQJA has a stronger first card, so it gets rank 5
///   and T55J5 gets rank 4.
///
/// Now, you can determine the total winnings of this set of hands by adding up the result of
/// multiplying each hand's bid with its rank (765 * 1 + 220 * 2 + 28 * 3 + 684 * 4 + 483 * 5). So
/// the total winnings in this example are 6440.
///
/// Find the rank of every hand in your set. What are the total winnings?
pub fn solve_part1<B: BufRead>(input: B) -> std::io::Result<isize> {
    get_winnings(input, RuleSet::Standard)
}

/// # Jokers
///
/// To make things a little more interesting, the Elf introduces one additional rule. Now, J cards
/// are jokers - wildcards that can act like whatever card would make the hand the strongest type
/// possible.
///
/// To balance this, J cards are now the weakest individual cards, weaker even than 2. The other
/// cards stay in the same order: A, K, Q, T, 9, 8, 7, 6, 5, 4, 3, 2, J.
///
/// J cards can pretend to be whatever card is best for the purpose of determining hand type; for
/// example, QJJQ2 is now considered four of a kind. However, for the purpose of breaking ties
/// between two hands of the same type, J is always treated as J, not the card it's pretending to
/// be: JKKK2 is weaker than QQQQ2 because J is weaker than Q.
///
/// Now, the above example goes very differently:
///
/// ```
/// 32T3K 765
/// T55J5 684
/// KK677 28
/// KTJJT 220
/// QQQJA 483
/// ```
///
/// - 32T3K is still the only one pair; it doesn't contain any jokers, so its strength doesn't
///   increase.
/// - KK677 is now the only two pair, making it the second-weakest hand.
/// - T55J5, KTJJT, and QQQJA are now all four of a kind! T55J5 gets rank 3, QQQJA gets rank 4, and
///   KTJJT gets rank 5.
///
/// With the new joker rule, the total winnings in this example are 5905.
///
/// Using the new joker rule, find the rank of every hand in your set. What are the new total
/// winnings?
///
pub fn solve_part2<B: BufRead>(input: B) -> std::io::Result<isize> {
    get_winnings(input, RuleSet::Joker)
}

fn get_winnings<B: BufRead>(input: B, rules: RuleSet) -> std::io::Result<isize> {
    let hands = input.lines().into_iter()
        .map(|l| Hand::new(l.unwrap(), rules))
        .collect::<BinaryHeap<_>>()
        .into_sorted_vec();

    let winnings = hands.iter().enumerate()
        .map(|(rank, hand)| {
            let rank = rank as isize + 1;
            let cards = String::from_utf8(hand.cards.0.to_vec()).unwrap();
            debug!("{rank:4} : {} bets {:4} ({:?})", cards, hand.bet, hand.strength);
            rank * hand.bet
        })
        .sum();

    Ok(winnings)
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
enum Strength {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum RuleSet {
    Standard,
    Joker
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct Cards([u8; 5]);

impl TryFrom<&[u8]> for Cards {
    type Error = ();
    fn try_from(other: &[u8]) -> Result<Self, Self::Error> {
        match other.try_into() {
            Ok(cards) => Ok(Cards(cards)),
            Err(_) => Err(()),
        }
    }
}

impl TryFrom<&str> for Cards {
    type Error = ();
    fn try_from(other: &str) -> Result<Self, Self::Error> {
        Cards::try_from(other.as_bytes())
    }
}

struct Hand {
    cards: Cards,
    bet: isize,
    strength: Strength,
    rules: RuleSet,
}

impl Hand {
    pub fn new<L: AsRef<str>>(line: L, rules: RuleSet) -> Self {
        let line = line.as_ref()
            .split_whitespace()
            .collect::<Vec<_>>();
        let cards: Cards = line[0].try_into().unwrap();
        let bet = isize::from_str_radix(line[1], 10).unwrap();
        let strength = Hand::strength(&cards, rules);
        Hand { cards, bet, strength, rules }
    }

    pub fn strength(cards: &Cards, rules: RuleSet) -> Strength {
        debug!("HAND: {}", cards.0.iter().map(|&c| c as char).collect::<String>());
        let mut unique = cards.0.to_vec();
        unique.sort_by(|a, b| card_strength(a, rules).cmp(&card_strength(b, rules)));
        unique.dedup();
        debug!("unique: {}", unique.iter().map(|&c| c as char).collect::<String>());

        let mut counts = unique.iter()
            .filter_map(|&c| match cards.0.iter().filter(|&&ch| ch == c).count() {
                0 => None,
                x => Some((c, x)),
            })
            .collect::<Vec<_>>();
        debug!("counts: {:?}", counts.iter().map(|(ch, cnt)| format!("{}: {}", *ch as char, cnt)).collect::<Vec<_>>());

        if rules == RuleSet::Joker {
            let jokers = counts.iter()
                .find(|&&(c, _)| c == b'J')
                .map_or(0, |&(_, cnt)| cnt);
            counts.retain(|&(c, _)| c != b'J');
            debug!("*count: {:?} ({jokers} jokers)", counts.iter().map(|(ch, cnt)| format!("{}: {}", *ch as char, cnt)).collect::<Vec<_>>());

            // re-sort to obtain largest counts
            let mut counts = counts.iter()
                .map(|&(_, cnt)| cnt)
                .collect::<Vec<_>>();
            counts.sort();
            let most = counts.pop();
            let nextmost = counts.pop();
            hand_strength_joker(jokers, most, nextmost)
        } else {
            let mut counts = counts.iter()
                .map(|&(_, cnt)| cnt)
                .collect::<Vec<_>>();
            counts.sort();
            let most = counts.pop();
            let nextmost = counts.pop();
            hand_strength_standard(most, nextmost)
        }
    }
}

impl std::fmt::Debug for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Hand")
            .field("cards", &self.cards)
            .field("strength", &self.strength)
            .field("bet", &self.bet)
            .field("rules", &self.rules)
            .finish()
    }
}

impl std::fmt::Debug for Cards {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cards = String::from_utf8(self.0.to_vec()).unwrap();
        write!(f, "{}", cards.as_str())
    }
}

impl std::cmp::PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl std::cmp::Eq for Hand {}
impl std::cmp::PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

impl std::cmp::Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.strength.cmp(&other.strength) {
            std::cmp::Ordering::Equal => {
                let left = self.cards.0.iter().map(|c| card_strength(c, self.rules));
                let right = other.cards.0.iter().map(|c| card_strength(c, self.rules));
                left.cmp(right)
            }
            order => order
        }
    }
}

const fn card_strength(card: &u8, rules: RuleSet) -> isize {
    match card {
        b'A' => 14,
        b'K' => 13,
        b'Q' => 12,
        b'J' => match rules {
            RuleSet::Standard => 11,
            RuleSet::Joker => 1,
        },
        b'T' => 10,
        b'9' => 9,
        b'8' => 8,
        b'7' => 7,
        b'6' => 6,
        b'5' => 5,
        b'4' => 4,
        b'3' => 3,
        b'2' => 2,
        _ => 0,
    }
}

const fn hand_strength_standard(first: Option<usize>, second: Option<usize>) -> Strength {
    match (first, second) {
        (Some(5), _)       => Strength::FiveKind,
        (Some(4), _)       => Strength::FourKind,
        (Some(3), Some(2)) => Strength::FullHouse,
        (Some(3), _)       => Strength::ThreeKind,
        (Some(2), Some(2)) => Strength::TwoPair,
        (Some(2), _)       => Strength::OnePair,
        _                  => Strength::HighCard,
    }
}

const fn hand_strength_joker(jokers: usize, first: Option<usize>, second: Option<usize>) -> Strength {
    match (jokers, first, second) {
        (_, Some(5), _)       => Strength::FiveKind,
        (1, Some(4), _)       => Strength::FiveKind,
        (2, Some(3), _)       => Strength::FiveKind,
        (3, Some(2), _)       => Strength::FiveKind,
        (4, Some(1), _)       => Strength::FiveKind,
        (5, _, _)             => Strength::FiveKind,
        (0, Some(4), _)       => Strength::FourKind,
        (1, Some(3), _)       => Strength::FourKind,
        (2, Some(2), _)       => Strength::FourKind,
        (3, Some(1), _)       => Strength::FourKind,
        (4, _, _)             => Strength::FourKind,
        (0, Some(3), Some(2)) => Strength::FullHouse,
        (1, Some(2), Some(2)) => Strength::FullHouse,
        (3, _, _)             => Strength::FullHouse,
        (0, Some(3), _)       => Strength::ThreeKind,
        (1, Some(2), _)       => Strength::ThreeKind,
        (2, Some(1), _)       => Strength::ThreeKind,
        (0, Some(2), Some(2)) => Strength::TwoPair,
        (2, _, _)             => Strength::TwoPair,
        (0, Some(2), _)       => Strength::OnePair,
        (1, Some(1), _)       => Strength::OnePair,
        _                     => Strength::HighCard,
    }
}

