use anyhow::anyhow;
use std::str::FromStr;

const REGISTER_COUNT: usize = 4;
const REGISTER_Z: Register = Register(3);

const DIGIT_COUNT: usize = 14;
const MIN_DIGIT_RANGE: [isize; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
const MAX_DIGIT_RANGE: [isize; 9] = [9, 8, 7, 6, 5, 4, 3, 2, 1];

/// MONAD consists of a set of 14 blocks of instructions (of the same length)
/// Each block will read the input into `w`, and compute a few values
/// Because `x` and `y` are always cleared and `w` is always used to read the input,
/// only `z` carries over
///
/// There are two types of instructions blocks
/// The first kind always multiplies `z` by 26 and adds something (consisting of the input) to it
/// The second kind will either divide `z` by 26, or divide, multiply and add to z
/// The action is chosen based on a condition
///
/// Because the goal is to find inputs which set `z` to 0, it's important to set the inputs in a way
/// that we divide `z` by 26 in the second kind of instruction blocks.
/// The type of the instruction block can be determined by checking for division instructions, as the
/// first type of blocks always divide by 1.
///
/// A bit more investigation reveals that `z` works as a sort of stack (assuming all values < 26)
/// So this means that the first reducing instruction block will affect the last (unaffected) adding
/// instruction block prior to it.
///
/// Pseudocode:
///
/// ```
/// fn solve(digits: [isize; 14]) -> isize {
///     let mut acc = 0;
///
///     for (i, input) in digits.iter().enumerate() {
///         acc = if BLOCK_TYPE[i] == 1 {
///             (acc * 26) + *input + ADD[i]
///         } else if BLOCK_TYPE[i] == 26 {
///             if ((acc % 26) + CHECK[i]) != *input {
///                 ((acc / 26) * 26) + *input + ADD[i]
///             } else {
///                 (acc / 26)
///             }
///         }
///     }
///
///     acc
/// }
/// ```

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    solve(&MAX_DIGIT_RANGE, input)
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    solve(&MIN_DIGIT_RANGE, input)
}

fn solve(digit_rage: &[isize; 9], input: &[&str]) -> anyhow::Result<u64> {
    let instructions = input
        .iter()
        .map(|line| line.parse())
        .collect::<Result<Vec<Instruction>, _>>()?;

    let blocks = instructions
        .chunks_exact(instructions.len() / DIGIT_COUNT)
        .collect::<Vec<_>>();

    let pairs = find_pairs(&blocks);
    let mut digits = [0; DIGIT_COUNT];

    for pair in pairs {
        solve_pair(digit_rage, pair, &mut digits, &blocks);
    }

    Ok(digits.iter().fold(0, |acc, digit| acc * 10 + *digit as u64))
}

fn find_pairs(blocks: &[&[Instruction]]) -> Vec<(usize, usize)> {
    let mut pairs = Vec::with_capacity(DIGIT_COUNT / 2);
    let mut stack = Vec::with_capacity(DIGIT_COUNT / 2);

    for (i, block) in blocks.iter().enumerate() {
        let reduce_block = block
            .iter()
            .any(|i| matches!(i, Instruction::Divide(REGISTER_Z, Value::Literal(div)) if *div > 1));

        if reduce_block {
            pairs.push((stack.pop().unwrap(), i));
        } else {
            stack.push(i);
        }
    }

    pairs.sort_unstable();
    pairs
}

fn solve_pair(
    digit_rage: &[isize; 9],
    pair: (usize, usize),
    digits: &mut [u8],
    blocks: &[&[Instruction]],
) {
    let add_block = blocks[pair.0];
    let reduce_block = blocks[pair.1];

    for &add_value in digit_rage {
        for &reduce_value in digit_rage {
            let inputs = [add_value, reduce_value];
            let mut vm = VirtualMachine::new(&inputs);

            add_block.iter().for_each(|i| vm.execute(i));
            reduce_block.iter().for_each(|i| vm.execute(i));

            if vm.read(REGISTER_Z) != 0 {
                continue;
            }

            digits[pair.0] = add_value as u8;
            digits[pair.1] = reduce_value as u8;
            return;
        }
    }
}

#[derive(Eq, PartialEq)]
struct Register(u8);

impl Register {
    fn index(&self) -> usize {
        self.0 as usize
    }
}

impl From<u8> for Register {
    fn from(char: u8) -> Self {
        Register(char - b'w')
    }
}

enum Value {
    Register(Register),
    Literal(isize),
}

impl FromStr for Value {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let byte = input.as_bytes()[0];

        let value = match byte {
            b'w'..=b'z' => Value::Register(byte.into()),
            _ => Value::Literal(input.parse()?),
        };

        Ok(value)
    }
}

enum Instruction {
    Input(Register),
    Add(Register, Value),
    Multiply(Register, Value),
    Divide(Register, Value),
    Modulo(Register, Value),
    Equals(Register, Value),
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let dst = input.as_bytes()[4].into();

        let instruction = match &input[0..3] {
            "inp" => Instruction::Input(dst),
            "add" => Instruction::Add(dst, input[6..].parse()?),
            "mul" => Instruction::Multiply(dst, input[6..].parse()?),
            "div" => Instruction::Divide(dst, input[6..].parse()?),
            "mod" => Instruction::Modulo(dst, input[6..].parse()?),
            "eql" => Instruction::Equals(dst, input[6..].parse()?),
            unknown => return Err(anyhow!("Unknown instruction {}", unknown)),
        };

        Ok(instruction)
    }
}

struct VirtualMachine<'a> {
    registers: [isize; REGISTER_COUNT],
    inputs: &'a [isize],
}

impl<'a> VirtualMachine<'a> {
    fn new(inputs: &'a [isize]) -> Self {
        let registers = [0; REGISTER_COUNT];
        VirtualMachine { registers, inputs }
    }

    fn read(&self, register: Register) -> isize {
        self.registers[register.index()]
    }

    fn execute(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Input(dst) => {
                self.registers[dst.index()] = self.inputs[0];
                self.inputs = &self.inputs[1..];
            }
            Instruction::Add(dst, value) => {
                self.registers[dst.index()] += self.evaluate(value);
            }
            Instruction::Multiply(dst, value) => {
                self.registers[dst.index()] *= self.evaluate(value);
            }
            Instruction::Divide(dst, value) => {
                self.registers[dst.index()] /= self.evaluate(value);
            }
            Instruction::Modulo(dst, value) => {
                self.registers[dst.index()] %= self.evaluate(value);
            }
            Instruction::Equals(dst, value) => {
                let value = self.evaluate(value);
                let dst = &mut self.registers[dst.index()];
                *dst = (*dst == value) as isize
            }
        }
    }

    fn evaluate(&self, value: &Value) -> isize {
        match value {
            Value::Register(register) => self.registers[register.index()],
            Value::Literal(value) => *value,
        }
    }
}
