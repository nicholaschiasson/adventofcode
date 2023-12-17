use std::{collections::HashMap, fmt::Display, str::FromStr};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
enum Card {
	Joker,
	Two,
	Three,
	Four,
	Five,
	Six,
	Seven,
	Eight,
	Nine,
	Ten,
	Jack,
	Queen,
	King,
	Ace,
}

impl Display for Card {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		use Card::*;
		write!(
			f,
			"{}",
			match self {
				Joker => 'J',
				Two => '2',
				Three => '3',
				Four => '4',
				Five => '5',
				Six => '6',
				Seven => '7',
				Eight => '8',
				Nine => '9',
				Ten => 'T',
				Jack => 'J',
				Queen => 'Q',
				King => 'K',
				Ace => 'A',
			}
		)
	}
}

impl From<char> for Card {
	fn from(value: char) -> Self {
		use Card::*;
		match value {
			'2' => Two,
			'3' => Three,
			'4' => Four,
			'5' => Five,
			'6' => Six,
			'7' => Seven,
			'8' => Eight,
			'9' => Nine,
			'T' => Ten,
			'J' => Jack,
			'Q' => Queen,
			'K' => King,
			'A' => Ace,
			_ => panic!("uh oh"),
		}
	}
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
enum HandType {
	HighCard,
	OnePair,
	TwoPair,
	ThreeOfAKind,
	FullHouse,
	FourOfAKind,
	FiveOfAKind,
}

impl Display for HandType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		use HandType::*;
		write!(
			f,
			"{}",
			match self {
				HighCard => "HighCard",
				OnePair => "OnePair",
				TwoPair => "TwoPair",
				ThreeOfAKind => "ThreeOfAKind",
				FullHouse => "FullHouse",
				FourOfAKind => "FourOfAKind",
				FiveOfAKind => "FiveOfAKind",
			}
		)
	}
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
enum Rules {
	#[default]
	Normal,
	Joker,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
struct Hand {
	cards: [Card; 5],
}

impl Hand {
	fn with_rules(self, rules: Rules) -> Self {
		let mut cards = self.cards;
		for c in cards.as_mut() {
			match (*c, rules) {
				(Card::Jack | Card::Joker, Rules::Normal) => *c = Card::Jack,
				(Card::Jack | Card::Joker, Rules::Joker) => *c = Card::Joker,
				_ => (),
			}
		}
		Self { cards }
	}

	fn kind(&self) -> HandType {
		use HandType::*;
		let mut jokers = 0;
		let mut cards = self
			.cards
			.iter()
			.fold(HashMap::new(), |mut m, c| {
				if let Card::Joker = c {
					jokers += 1;
				} else {
					m.entry(c).and_modify(|e| *e += 1).or_insert(1);
				}
				m
			})
			.values()
			.copied()
			.collect::<Vec<_>>();

		cards.sort();
		if let Some(n) = cards.last_mut() {
			*n += jokers;
		}

		match cards.len() {
			0 | 1 => FiveOfAKind,
			2 if cards.contains(&4) => FourOfAKind,
			2 => FullHouse,
			3 if cards.contains(&3) => ThreeOfAKind,
			3 => TwoPair,
			4 => OnePair,
			_ => HighCard,
		}
	}
}
impl Display for Hand {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.cards.iter().fold(Ok(()), |_, c| write!(f, "{c}"))
	}
}

impl FromStr for Hand {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(Hand {
			cards: [
				s.chars().nth(0).map(|c| c.into()).ok_or("parse error")?,
				s.chars().nth(1).map(|c| c.into()).ok_or("parse error")?,
				s.chars().nth(2).map(|c| c.into()).ok_or("parse error")?,
				s.chars().nth(3).map(|c| c.into()).ok_or("parse error")?,
				s.chars().nth(4).map(|c| c.into()).ok_or("parse error")?,
			],
		})
	}
}

pub fn part_01(input: &str) -> u64 {
	let mut hands = input
		.lines()
		.flat_map(|l| l.split_once(' '))
		.map(|(hand, bid)| (hand.parse::<Hand>().unwrap(), bid.parse::<u64>().unwrap()))
		.map(|(hand, bid)| (hand.kind(), hand, bid))
		.collect::<Vec<_>>();
	hands.sort_by_key(|&(kind, hand, _)| (kind, hand));
	hands
		.iter()
		.enumerate()
		.fold(0, |sum, (i, &(_, _, bid))| sum + bid * (i as u64 + 1))
}

pub fn part_02(input: &str) -> u64 {
	let mut hands = input
		.lines()
		.flat_map(|l| l.split_once(' '))
		.map(|(hand, bid)| {
			(
				hand.parse::<Hand>().unwrap().with_rules(Rules::Joker),
				bid.parse::<u64>().unwrap(),
			)
		})
		.map(|(hand, bid)| (hand.kind(), hand, bid))
		.collect::<Vec<_>>();
	hands.sort_by_key(|&(kind, hand, _)| (kind, hand));
	hands
		.iter()
		.enumerate()
		.fold(0, |sum, (i, &(_, _, bid))| sum + bid * (i as u64 + 1))
}

#[cfg(test)]
mod tests {
	use crate::utils::{read_resource, relative_input_path};

	const INPUT_PATH: &str = module_path!();

	#[test]
	fn part_01() {
		assert_eq!(
			super::part_01(&read_resource(relative_input_path(&format!("{INPUT_PATH}::practice_01")))),
			6440
		);
		assert_eq!(
			super::part_01(&read_resource(relative_input_path(&format!("{INPUT_PATH}::final")))),
			251545216
		);
	}

	#[test]
	fn part_02() {
		assert_eq!(
			super::part_02(&read_resource(relative_input_path(&format!("{INPUT_PATH}::practice_01")))),
			5905
		);
		assert_eq!(
			super::part_02(&read_resource(relative_input_path(&format!("{INPUT_PATH}::final")))),
			250384185
		);
	}
}
