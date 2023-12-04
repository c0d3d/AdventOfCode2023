#![feature(exclusive_range_pattern)]

use std::fs::read_to_string;

type Coord = (usize, usize);

// Last is num digits
#[derive(Debug)]
struct Number(u32, Coord, usize);

impl Number {
    fn next_to(&self, (x, y): Coord) -> bool {
        let Number(_, (x0, y0), off) = self;
        for x_idx in 0..*off {
            let x_abs = (x0 + x_idx).abs_diff(x);
            let y_abs = y0.abs_diff(y);
            if x_abs <= 1 && y_abs <= 1 {
                return true;
            }
        }
        return false;
    }
}

struct SpecialSymbol(char, Coord);

#[derive(Debug)]
enum Symbol {
    Digit(char),
    Blank,
    Other(char),
}

struct Engine {
    syms: Vec<Vec<Symbol>>,
}

impl Engine {
    fn new() -> Engine {
        Engine { syms: Vec::new() }
    }

    fn add_line(&mut self, l: &str) {
        let mut nxt_line = Vec::new();
        for c in l.chars() {
            nxt_line.push(match c {
                '0'..='9' => Symbol::Digit(c),
                '.' => Symbol::Blank,
                _ => Symbol::Other(c)
            });
        }
        self.syms.push(nxt_line);
    }

    // Returns parsed numebrs, and indicator symbols
    fn parse(&self) -> (Vec<Number>, Vec<SpecialSymbol>) {
        let mut others = Vec::new();
        let mut numbers = Vec::new();

        for (y, row) in self.syms.iter().enumerate() {
            let mut cur_num = None;
            for (x, el) in row.iter().enumerate() {
                match (el, &cur_num) {
                    (Symbol::Other(d), None) => others.push(SpecialSymbol(*d, (x,y))),
                    (Symbol::Digit(d), None) => {
                        cur_num = Some(d.to_string());
                    },
                    (Symbol::Digit(d), Some(s)) => {
                        cur_num = Some(s.to_string() + &d.to_string());
                    }
                    (Symbol::Other(d), Some(s)) => {
                        let num_digits = s.len();
                        let num = s.parse().unwrap();
                        numbers.push(Number(num, (x - num_digits, y), num_digits));
                        cur_num = None;
                        others.push(SpecialSymbol(*d, (x, y)));
                    }
                    (Symbol::Blank, Some(s)) => {
                        let num_digits = s.len();
                        let num = s.parse().unwrap();
                        numbers.push(Number(num, (x - num_digits, y), num_digits));
                        cur_num = None;
                    },
                    (Symbol::Blank, None) => {
                        // Noop.
                    }
                }
            }

            // Final trailing number
            if let Some(s) = cur_num {
                let num_digits = s.len();
                let num = s.parse().unwrap();
                numbers.push(Number(num, (row.len() - 1 - num_digits, y), num_digits));
            }
        }

        return (numbers, others);
    }
}

fn main() {
    let mut eng = Engine::new();
    read_to_string("input.txt")
        .unwrap()
        .lines()
        .for_each(|x| eng.add_line(x));

    let (numbers, target_symbols) = eng.parse();

    let mut sum_p1 = 0;
    for number in numbers.iter() {
        for sym in target_symbols.iter() {
            if number.next_to(sym.1) {
                sum_p1 += number.0;
                break;
            }
        }
    }

    println!("{sum_p1}");

    let mut sum_p2 = 0;

    for sym in target_symbols.iter() {
        let mut adjacent = Vec::new();
        for number in numbers.iter() {
            if number.next_to(sym.1) {
                adjacent.push(number);
            }
        }

        if adjacent.len() == 2 {
            sum_p2 += adjacent[0].0 * adjacent[1].0;
        }
    }

    println!("{sum_p2}");
}
