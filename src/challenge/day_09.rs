const MAX_DEPTH: u8 = 9;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let map = Map::new(input);
    let mut risk_level = 0usize;

    for y in 0..map.height() {
        for x in 0..map.width() {
            let depth = map.get(x, y);

            let is_lowest = depth < map.up(x, y)
                && depth < map.down(x, y)
                && depth < map.left(x, y)
                && depth < map.right(x, y);

            if is_lowest {
                risk_level += depth as usize + 1;
            }
        }
    }

    Ok(risk_level)
}

struct Map {
    width: usize,
    height: usize,
    depths: Vec<u8>,
}

impl Map {
    fn new(input: &[&str]) -> Map {
        let width = input[0].len();
        let height = input.len();

        let values = input
            .iter()
            .flat_map(|line| line.bytes())
            .map(|byte| byte - b'0');

        let mut depths = Vec::<u8>::with_capacity(width * height);
        depths.extend(values);

        Map {
            width,
            height,
            depths,
        }
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn get(&self, x: usize, y: usize) -> u8 {
        self.depths[(x + y * self.width)]
    }

    fn up(&self, x: usize, y: usize) -> u8 {
        if y == 0 {
            MAX_DEPTH
        } else {
            self.depths[(x + (y - 1) * self.width)]
        }
    }

    fn down(&self, x: usize, y: usize) -> u8 {
        let y = y + 1;

        if y >= self.height {
            MAX_DEPTH
        } else {
            self.depths[(x + y * self.width)]
        }
    }

    fn left(&self, x: usize, y: usize) -> u8 {
        if x == 0 {
            MAX_DEPTH
        } else {
            self.depths[((x - 1) + y * self.width)]
        }
    }

    fn right(&self, x: usize, y: usize) -> u8 {
        let x = x + 1;

        if x >= self.width {
            MAX_DEPTH
        } else {
            self.depths[(x + y * self.width)]
        }
    }
}
