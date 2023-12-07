use std::{cmp::Ordering, str::FromStr};

use anyhow::{anyhow, bail, Context, Error, Result};

pub fn run(lines: &[String]) -> Result<usize> {
    let mut hands: Vec<Hand> = lines
        .iter()
        .map(|line| line.as_str())
        .map(Hand::from_str)
        .collect::<Result<Vec<_>, _>>()?;

    hands.sort();

    let sum: usize = hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.bid)
        .sum();

    Ok(sum)
}

#[derive(Debug, Clone)]
struct Hand {
    cards: [Card; 5],
    hand_type: Option<Type>,
    bid: usize,
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self.hand_type, other.hand_type) {
            (Some(self_type), Some(other_type)) => {
                let ordering = self_type.cmp(&other_type);
                if ordering == Ordering::Equal {
                    self.cards.cmp(&other.cards)
                } else {
                    ordering
                }
            }
            (Some(_), None) => Ordering::Greater,
            (None, Some(_)) => Ordering::Less,
            (None, None) => self.cards.cmp(&other.cards),
        }
    }
}

impl FromStr for Hand {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_ascii_whitespace();

        let cards = iter
            .next()
            .context("no cards in line?")?
            .as_bytes()
            .iter()
            .map(Card::try_from)
            .collect::<Result<Vec<Card>>>()?
            .try_into()
            .map_err(|_| anyhow!("Unable to map cards string to [Card; 5]"))?;

        let hand_type = Type::try_from(&cards).ok();

        let bid = iter.next().context("no bid in line?")?.parse()?;

        Ok(Self {
            cards,
            hand_type,
            bid,
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,
    T,
    J,
    Q,
    K,
    A,
}

impl TryFrom<&u8> for Card {
    type Error = Error;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        Ok(match value {
            b'2' => Card::_2,
            b'3' => Card::_3,
            b'4' => Card::_4,
            b'5' => Card::_5,
            b'6' => Card::_6,
            b'7' => Card::_7,
            b'8' => Card::_8,
            b'9' => Card::_9,
            b'T' => Card::T,
            b'J' => Card::J,
            b'Q' => Card::Q,
            b'K' => Card::K,
            b'A' => Card::A,
            _ => bail!("No Card matches this symbol"),
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Type {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl TryFrom<&[Card; 5]> for Type {
    type Error = Error;

    fn try_from(cards: &[Card; 5]) -> Result<Self, Self::Error> {
        fn count(cards: &[Card; 5]) -> [u32; 13] {
            let mut counts = [0; 13];
            for &card in cards {
                counts[card as usize] += 1;
            }
            counts
        }

        let count = count(cards);

        if count.contains(&5) {
            return Ok(Self::FiveOfAKind);
        }

        if count.contains(&4) {
            return Ok(Self::FourOfAKind);
        }

        if count.contains(&3) && count.contains(&2) {
            return Ok(Self::FullHouse);
        }

        if count.contains(&3) {
            return Ok(Self::ThreeOfAKind);
        }

        if let Some(idx) = count.iter().position(|&c| c == 2) {
            if count[..idx].contains(&2) || count[idx + 1..].contains(&2) {
                return Ok(Self::TwoPair);
            } else {
                return Ok(Self::OnePair);
            }
        }

        if count.into_iter().filter(|&c| c == 1).count() == 5 {
            return Ok(Self::HighCard);
        }

        bail!("cards don't have a type")
    }
}
