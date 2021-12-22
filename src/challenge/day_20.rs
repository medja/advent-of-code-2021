use std::ops::Index;

const VALUE_MASK: usize = 0b111111111; // 2^9 - 1
const VALUE_COUNT: usize = 512; // 2^9

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let algorithm = Algorithm::new(input[0]);
    let mut image = Image::from_str(&input[2..]);

    image.enhance(&algorithm);
    image.enhance(&algorithm);

    Ok(image.count_lit_pixels())
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let algorithm = Algorithm::new(input[0]);
    let mut image = Image::from_str(&input[2..]);

    for _ in 0..50 {
        image.enhance(&algorithm);
    }

    Ok(image.count_lit_pixels())
}

struct Algorithm([bool; VALUE_COUNT]);

impl Algorithm {
    fn new(input: &str) -> Self {
        let mut algorithm = [false; VALUE_COUNT];

        for (index, byte) in input.bytes().enumerate() {
            if byte == b'#' {
                algorithm[index] = true;
            }
        }

        Algorithm(algorithm)
    }
}

impl Index<usize> for Algorithm {
    type Output = bool;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

struct Image {
    width: usize,
    height: usize,
    lit_value: bool,
    pixels: Vec<bool>,
}

impl Image {
    fn from_str(input: &[&str]) -> Self {
        let width = input[0].len() + 4;
        let height = input.len() + 4;
        let lit_value = true;

        let mut pixels = Vec::with_capacity(width * height);
        pixels.extend(std::iter::repeat(false).take(width * 2 - 2));

        for line in input {
            pixels.extend(std::iter::repeat(false).take(4));
            pixels.extend(line.bytes().map(|byte| byte == b'#'));
        }

        pixels.extend(std::iter::repeat(false).take(width * 2 + 2));

        Image {
            width,
            height,
            lit_value,
            pixels,
        }
    }

    fn count_lit_pixels(&self) -> usize {
        self.pixels
            .iter()
            .filter(|pixel| **pixel == self.lit_value)
            .count()
    }

    fn enhance(&mut self, algorithm: &Algorithm) {
        let width = self.width + 2;
        let height = self.height + 2;

        let lit_value = if algorithm[0] {
            !self.lit_value
        } else {
            self.lit_value
        };

        let mut pixels = vec![false; width * height];

        for y in 1..(self.height - 1) {
            let src_index = y * self.width;
            let dst_index = (y + 1) * width;

            for x in 1..(self.width - 1) {
                let src_index = x + src_index;
                let dst_index = x + 1 + dst_index;

                let count = self.count_surrounding_lit_pixels(src_index);
                pixels[dst_index] = algorithm[count] == lit_value;
            }
        }

        self.width = width;
        self.height = height;
        self.lit_value = lit_value;
        self.pixels = pixels;
    }

    fn count_surrounding_lit_pixels(&self, index: usize) -> usize {
        let count = (self.pixels[index - self.width - 1] as usize) << 8
            | (self.pixels[index - self.width] as usize) << 7
            | (self.pixels[index - self.width + 1] as usize) << 6
            | (self.pixels[index - 1] as usize) << 5
            | (self.pixels[index] as usize) << 4
            | (self.pixels[index + 1] as usize) << 3
            | (self.pixels[index + self.width - 1] as usize) << 2
            | (self.pixels[index + self.width] as usize) << 1
            | self.pixels[index + self.width + 1] as usize;

        if self.lit_value {
            count
        } else {
            count ^ VALUE_MASK
        }
    }
}
