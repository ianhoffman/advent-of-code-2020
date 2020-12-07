use std::collections::HashSet;
use std::fs;
use std::iter::FromIterator;
use std::iter::Peekable;

struct Parser<I>
where
    I: Iterator<Item = String>,
{
    c: Peekable<I>,
}

impl<I> Parser<I>
where
    I: Iterator<Item = String>,
{
    fn from_iter(iter: I) -> Parser<I> {
        return Parser { c: iter.peekable() };
    }

    fn next(&mut self) -> String {
        self.c.next().unwrap_or("".to_owned())
    }

    fn peek(&mut self) -> Option<&String> {
        self.c.peek()
    }

    fn is_done(&mut self) -> bool {
        self.peek() == None
    }

    fn is_emptyline(&mut self) -> bool {
        match self.peek() {
            Some(line) => line.is_empty(),
            None => false,
        }
    }

    fn skip_emptylines(&mut self) {
        while self.is_emptyline() {
            self.next();
        }
    }

    fn parse_one(&mut self) -> usize {
        let mut questions: HashSet<char> = HashSet::from_iter(self.next().chars());
        while !self.is_emptyline() && !self.is_done() {
            let qs: HashSet<char> = HashSet::from_iter(self.next().chars());
            questions = questions.into_iter().filter(|c| qs.contains(&c)).collect();
        }
        questions.len()
    }

    fn parse(&mut self) -> usize {
        let mut count = 0;
        while !self.is_done() {
            self.skip_emptylines();
            count += self.parse_one();
        }
        count
    }
}

fn main() {
    let content = fs::read_to_string("input.txt").expect("Failed to read input.txt");
    let mut parser = Parser::from_iter(content.lines().map(|line| line.to_owned()));
    let count = parser.parse();
    println!("Total count: {}", count);
}
