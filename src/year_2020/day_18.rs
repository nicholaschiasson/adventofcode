use lexical::parse_partial;
use std::{fmt::Display, iter};

trait Evaluate {
	fn evaluate(&self) -> i64;
	fn advanced_evaluate(&self) -> i64;
}

#[derive(Clone, Debug)]
struct Expression {
	operands: Vec<Operand>,
	operators: Vec<Operator>,
}

impl Evaluate for Expression {
	fn evaluate(&self) -> i64 {
		self
			.operands
			.iter()
			.zip(iter::once(&Operator::Addition).chain(self.operators.iter()))
			.fold(0, |left, (right, op)| op.operate(left, right.evaluate()))
	}
	fn advanced_evaluate(&self) -> i64 {
		let mut values = vec![self.operands[0].advanced_evaluate()];
		for (i, op) in self.operators.iter().enumerate() {
			match op {
				Operator::Addition => {
					let last = values.pop().unwrap();
					values.push(last + self.operands[i + 1].advanced_evaluate());
				}
				Operator::Multiplication => values.push(self.operands[i + 1].advanced_evaluate()),
			}
		}
		values.iter().product()
	}
}

impl Display for Expression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.operands[0])?;
		for (operand, operator) in self.operands.iter().skip(1).zip(self.operators.iter()) {
			write!(f, " {} {}", operator, operand)?;
		}
		Ok(())
	}
}

// This is bad...
fn parse_expression(s: &str) -> (Expression, usize) {
	let mut operands = Vec::new();
	let mut operators = Vec::new();
	let mut i = 0;
	while i < s.len() {
		operands.push(if s[i..].starts_with('(') {
			let begin = i + 1;
			let (e, end) = parse_expression(&s[begin..]);
			i = begin + end + 1;
			Operand::Expression(e)
		} else {
			if let Ok((v, idx)) = parse_partial::<i64, _>(&s[i..]) {
				i += idx;
				Operand::Value(v)
			} else {
				panic!("Failed to parse operand")
			}
		});
		if s[i..].starts_with(')') {
			break;
		}
		if i < s.len() {
			operators.push(match &s[i + 1..i + 2] {
				"+" => Operator::Addition,
				"*" => Operator::Multiplication,
				_ => panic!("Failed to parse operator"),
			});
			i += 3;
		}
	}
	(Expression { operands, operators }, i)
}

#[derive(Clone, Debug)]
enum Operand {
	Expression(Expression),
	Value(i64),
}

impl Display for Operand {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Expression(e) => write!(f, "({})", e),
			Self::Value(v) => write!(f, "{}", v),
		}
	}
}

impl Evaluate for Operand {
	fn evaluate(&self) -> i64 {
		match self {
			Self::Expression(e) => e.evaluate(),
			Self::Value(v) => *v,
		}
	}
	fn advanced_evaluate(&self) -> i64 {
		match self {
			Self::Expression(e) => e.advanced_evaluate(),
			Self::Value(v) => *v,
		}
	}
}

#[derive(Clone, Copy, Debug)]
enum Operator {
	Addition,
	Multiplication,
}

impl Operator {
	fn operate(&self, a: i64, b: i64) -> i64 {
		match self {
			Self::Addition => a + b,
			Self::Multiplication => a * b,
		}
	}
}

impl Display for Operator {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			match self {
				Self::Addition => '+',
				Self::Multiplication => '*',
			}
		)
	}
}

pub fn part_01(input: &String) -> i64 {
	input.lines().fold(0, |sum, l| sum + parse_expression(l).0.evaluate())
}

pub fn part_02(input: &String) -> i64 {
	input
		.lines()
		.fold(0, |sum, l| sum + parse_expression(l).0.advanced_evaluate())
}
