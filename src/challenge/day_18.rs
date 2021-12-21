use std::ops::Add;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let result = input
        .iter()
        .map(|line| Number::parse(line))
        .reduce(|left, right| left + right)
        .unwrap();

    Ok(result.magnitude())
}

#[derive(Copy, Clone)]
struct Digit {
    value: u8,
    depth: u8,
}

impl Digit {
    fn new(value: u8, depth: u8) -> Self {
        Digit { value, depth }
    }

    fn value(&self) -> u8 {
        self.value
    }

    fn depth(&self) -> u8 {
        self.depth
    }

    fn add(&mut self, value: u8) {
        self.value += value;
    }

    fn nest(&self) -> Self {
        Digit {
            value: self.value,
            depth: self.depth + 1,
        }
    }
}

struct Number(Vec<Digit>);

impl Number {
    fn parse(string: &str) -> Self {
        let mut depth = 0;
        let mut digits = Vec::new();

        for byte in string.bytes() {
            match byte {
                b'[' => depth += 1,
                b']' => depth -= 1,
                b',' => {}
                _ => digits.push(Digit::new(byte - b'0', depth)),
            }
        }

        Number(digits)
    }

    fn magnitude(&self) -> usize {
        let mut sum = 0;
        let mut stack = Vec::new();

        for digit in &self.0 {
            let depth = digit.depth();
            let mut value = digit.value() as usize;

            stack.extend((stack.len()..depth as usize).map(|_| false));

            for right in &stack {
                if *right {
                    value *= 2;
                } else {
                    value *= 3;
                }
            }

            while let Some(last) = stack.last_mut() {
                if *last {
                    stack.pop();
                } else {
                    *last = true;
                    break;
                }
            }

            sum += value;
        }

        sum
    }

    fn explode(&mut self) -> bool {
        let index = match self.0.iter().position(|digit| digit.depth > 4) {
            Some(index) => index,
            None => return false,
        };

        let left = self.0[index];
        let right = self.0.remove(index + 1);
        let depth = left.depth();

        if index > 0 {
            self.0[index - 1].add(left.value());
        }

        if index + 1 < self.0.len() {
            self.0[index + 1].add(right.value());
        }

        self.0[index] = Digit::new(0, depth - 1);

        true
    }

    fn split(&mut self) -> bool {
        let index = match self.0.iter().position(|digit| digit.value > 9) {
            Some(index) => index,
            None => return false,
        };

        let digit = self.0[index];
        let value = digit.value();
        let depth = digit.depth();

        let left = value / 2;
        let right = value - left;

        if depth < 4 {
            self.0[index] = Digit::new(left, depth + 1);
            self.0.insert(index + 1, Digit::new(right, depth + 1));
        } else {
            if index > 0 {
                self.0[index - 1].add(left);
            }

            if index + 1 < self.0.len() {
                self.0[index + 1].add(right);
            }

            self.0[index] = Digit::new(0, depth);
        }

        true
    }
}

impl Add for Number {
    type Output = Number;

    fn add(self, rhs: Self) -> Self::Output {
        let mut digits = Vec::with_capacity(self.0.len() + rhs.0.len());
        digits.extend(self.0.iter().map(|digit| digit.nest()));
        digits.extend(rhs.0.iter().map(|digit| digit.nest()));

        let mut number = Number(digits);
        while number.explode() {}
        while number.split() {}
        number
    }
}
