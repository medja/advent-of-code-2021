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

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let map = Map::new(input);
    let mut max = [0usize; 3];

    for size in basin_sizes(&map) {
        if size > max[2] {
            max[0] = max[1];
            max[1] = max[2];
            max[2] = size;
        } else if size > max[1] {
            max[0] = max[1];
            max[1] = size;
        } else if size > max[0] {
            max[0] = size;
        }
    }

    Ok(max.iter().product::<usize>())
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

fn basin_sizes(map: &Map) -> impl Iterator<Item = usize> {
    let mut basin_sizes = vec![0usize];
    let mut lookbehind = vec![0usize; map.width()];

    for y in 0..map.height() {
        for x in 0..map.width() {
            if map.get(x, y) == MAX_DEPTH {
                lookbehind[x] = 0;
                continue;
            }

            let top = lookbehind[x];
            let left = if x == 0 { 0 } else { lookbehind[x - 1] };

            if left == 0 && top == 0 {
                // found a potentially new basin
                lookbehind[x] = basin_sizes.len();
                basin_sizes.push(1);
            } else if top == 0 {
                lookbehind[x] = left;
                basin_sizes[left] += 1;
            } else if left == 0 || top == left {
                // no need to update lookbehind, it's already correct
                basin_sizes[top] += 1;
            } else {
                // looks like we previously detected a new basin that connects to an existing one
                // invalidate the left basin and add it to the top basin
                basin_sizes[top] += basin_sizes[left] + 1;
                basin_sizes[left] = 0;

                for prev in lookbehind[0..x].iter_mut().rev() {
                    if *prev == left {
                        *prev = top;
                    } else {
                        break;
                    }
                }
            }
        }
    }

    basin_sizes.into_iter().filter(|size| *size > 0)
}
