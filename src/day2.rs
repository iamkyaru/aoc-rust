use std::ops::Deref;

use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

pub struct PaperRockScissors {
    input: Vec<(Shape, Shape)>,
}

#[derive(PartialEq, Clone, Copy)]
pub enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Shape {
    pub fn losing(self) -> Shape {
        match self {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper,
        }
    }

    pub fn winning(self) -> Shape {
        match self {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock,
        }
    }
}

impl TryFrom<char> for Shape {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' | 'X' => Ok(Shape::Rock),
            'B' | 'Y' => Ok(Shape::Paper),
            'C' | 'Z' => Ok(Shape::Scissors),
            _ => Err("Unknown value "),
        }
    }
}

impl Deref for Shape {
    type Target = char;

    fn deref(&self) -> &Self::Target {
        match self {
            Shape::Rock => &'X',
            Shape::Paper => &'Y',
            Shape::Scissors => &'Z',
        }
    }
}

impl PaperRockScissors {
    pub fn solve(&self, opponent: &Shape, me: &Shape) -> usize {
        let mut score = 0;

        if self.won(opponent, me) {
            score += 6;
        } else if self.draw(opponent, me) {
            score += 3;
        }

        score += *me as usize;

        score
    }

    pub fn one(&self) -> usize {
        let mut score = 0;

        for (opponent, me) in &self.input {
            score += self.solve(opponent, me);
        }

        score
    }

    pub fn two(&self) -> usize {
        let mut score = 0;

        for (opponent, outcome) in &self.input {
            score += match outcome.deref() {
                'X' => self.solve(opponent, &opponent.losing()),
                'Y' => self.solve(opponent, opponent),
                'Z' => self.solve(opponent, &opponent.winning()),
                _ => continue,
            };
        }

        score
    }

    fn won(&self, opponent: &Shape, me: &Shape) -> bool {
        matches!(
            (opponent, me),
            (Shape::Rock, Shape::Paper)
                | (Shape::Paper, Shape::Scissors)
                | (Shape::Scissors, Shape::Rock)
        )
    }

    fn draw(&self, opponent: &Shape, me: &Shape) -> bool {
        opponent.eq(me)
    }
}

#[aoc_generator(day2)]
pub fn generator(input: &str) -> PaperRockScissors {
    let input = input.lines().filter_map(shapes_from_line).collect();

    PaperRockScissors { input }
}

fn shapes_from_line(line: &str) -> Option<(Shape, Shape)> {
    let opponent = line.chars().next()?;
    let me = line.chars().nth(2)?;

    Some((opponent.try_into().ok()?, me.try_into().ok()?))
}

#[aoc(day2, part1)]
pub fn solver1(input: &PaperRockScissors) -> usize {
    input.one()
}

#[aoc(day2, part2)]
pub fn solver2(input: &PaperRockScissors) -> usize {
    input.two()
}