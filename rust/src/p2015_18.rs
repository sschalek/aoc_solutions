// Advent of Code 2015, Day 18: "Like a GIF For Your Yard"
// https://adventofcode.com/2015/day/18

#[derive(Clone, Debug, PartialEq, Eq)]
struct LightGrid {
    lights: Vec<bool>,
    width: usize,
    fixed_lights: Vec<((usize, usize), bool)>,
}

impl LightGrid {
    // Creates a new instance with the given initial state.
    pub fn new(initial_state: &[bool], width: usize, fixed_lights: &[((usize, usize), bool)]) -> Self {
        let mut grid = Self {
            lights: initial_state.to_vec(),
            width,
            fixed_lights: Vec::new(),
        };
        grid.fix_lights(fixed_lights);
        grid
    }

    pub fn fix_lights(&mut self, fixed_lights: &[((usize, usize), bool)]) {
        self.fixed_lights.append(&mut fixed_lights.to_vec());
        for ((x, y), state) in fixed_lights {
            let index = self.get_light_index(*x, *y);
            self.lights[index] = *state;
        }
    }

    pub fn step_animation(&mut self, count: usize) {
        for _ in 0..count {
            let mut next_state = vec![false; self.lights.len()];
            for x in 0..self.width {
                for y in 0..self.width {
                    let index = self.get_light_index(x, y);

                    // If the light is fixed, then its state never changes.
                    if self.fixed_lights.iter().any(|((fx, fy), _)| *fx == x && *fy == y) {
                        next_state[index] = self.lights[index];
                        continue;
                    }

                    // Otherwise, the light's next state is determined by its current state and the number of
                    // its neighbours that are on.
                    let neighbour_on_count = self.get_neighbour_on_count(x, y);
                    if self.lights[index] {
                        next_state[index] = neighbour_on_count == 2 || neighbour_on_count == 3;
                    } else {
                        next_state[index] = neighbour_on_count == 3;
                    }
                }
            }
            self.lights = next_state;
        }
    }

    fn get_light_index(&self, x: usize, y: usize) -> usize {
        x + y * self.width
    }

    fn get_neighbour_on_count(&self, x: usize, y: usize) -> usize {
        let mut count = 0;
        for i in -1..=1 {
            for j in -1..=1 {
                if i == 0 && j == 0 {
                    continue;
                }

                let x = x as i32 + i;
                let y = y as i32 + j;
                if x < 0 || y < 0 || x >= self.width as i32 || y >= self.width as i32 {
                    continue;
                }

                if self.lights[self.get_light_index(x as usize, y as usize)] {
                    count += 1;
                }
            }
        }
        count
    }
}

impl std::fmt::Display for LightGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.width {
            for x in 0..self.width {
                let index = self.get_light_index(x, y);
                write!(f, "{} ", if self.lights[index] { '#' } else { '.' })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn parse_light_grid(input: &str) -> LightGrid {
    let mut lights = Vec::new();
    let mut width = None;
    for line in input.lines() {
        for c in line.chars() {
            match c {
                '#' => lights.push(true),
                '.' => lights.push(false),
                _ => panic!("Unexpected character in input: {c}"),
            }
        }

        match width {
            None => width = Some(line.len()),
            Some(w) => assert_eq!(w, line.len()),
        }
    }
    LightGrid::new(&lights, width.unwrap(), &[])
}

fn run_animation(grid: &mut LightGrid, count: usize, log_fn: Option<fn(&str)>) {
    let first_part = count.min(10);
    for i in 0..first_part {
        if let Some(log_fn) = log_fn {
            log_fn(&format!("Step {}", i + 1));
            log_fn(&format!("{grid}"));
        }
        grid.step_animation(1);
    }

    if count > 10 {
        grid.step_animation(count - 10);
        if let Some(log_fn) = log_fn {
            log_fn(&format!("Step {count}"));
            log_fn(&format!("{grid}"));
        }
    }
}

fn solve(input: &str, log_fn: Option<fn(&str)>) -> (String, String) {
    let mut light_grid = parse_light_grid(input);
    let mut light_grid2 = light_grid.clone();

    // Part 1: Find the number of lights that are on after 100 animation steps.
    run_animation(&mut light_grid, 100, log_fn);
    let part1_result = light_grid.lights.iter().filter(|&&light| light).count();

    // Part 2: Find the number of lights that are on after 100 animation steps, with the four corners always on.
    let max_index = light_grid2.width - 1;
    light_grid2.fix_lights(&[
        ((0, 0), true),
        ((0, max_index), true),
        ((max_index, 0), true),
        ((max_index, max_index), true),
    ]);
    run_animation(&mut light_grid2, 100, log_fn);
    let part2_result = light_grid2.lights.iter().filter(|&&light| light).count();

    (part1_result.to_string(), part2_result.to_string())
}

#[linkme::distributed_slice(crate::SOLUTIONS)]
static SOLUTION: crate::Solution = crate::Solution::new(2015, 18, solve);
