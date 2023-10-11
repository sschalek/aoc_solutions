// Advent of Code 2015, Day 13: "Knights of the Dinner Table"
// https://adventofcode.com/2015/day/13

struct DinnerDescription {
    guests: Vec<String>,
    happiness_rule_matrix: Vec<i32>,
}

impl DinnerDescription {
    pub fn new(rules: &[((String, String), i32)]) -> Self {
        let guests = Vec::new();
        let happiness_rule_matrix = Vec::new();
        let mut description = Self {
            guests,
            happiness_rule_matrix,
        };

        for ((subject_name, object_name), happiness) in rules {
            description.add_rule(subject_name, object_name, *happiness);
        }

        description
    }

    pub fn get_happiness_change(&self, guest1_id: usize, guest2_id: usize) -> i32 {
        let index = self.get_happiness_rule_matrix_index(guest1_id, guest2_id);
        self.happiness_rule_matrix[index]
    }

    pub fn ensure_guest(&mut self, guest_name: &str) -> usize {
        if let Some(guest_id) = self.guests.iter().position(|g| g == guest_name) {
            guest_id
        } else {
            self.guests.push(guest_name.to_string());
            for i in 0..self.guests.len() {
                self.happiness_rule_matrix
                    .insert(self.get_happiness_rule_matrix_index(i, self.guests.len() - 1), 0);
            }
            self.happiness_rule_matrix.append(&mut vec![0; self.guests.len()]);
            self.guests.len() - 1
        }
    }

    pub fn add_rule(&mut self, subject_name: &str, object_name: &str, happiness: i32) {
        let subject_id = self.ensure_guest(subject_name);
        let object_id = self.ensure_guest(object_name);
        let index = self.get_happiness_rule_matrix_index(subject_id, object_id);
        self.happiness_rule_matrix[index] = happiness;
    }

    fn get_happiness_rule_matrix_index(&self, subject_id: usize, object_id: usize) -> usize {
        (subject_id * self.guests.len()) + object_id
    }
}

impl std::fmt::Display for DinnerDescription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:7} Object", "Subject")?;
        write!(f, "{:7} ", "")?;
        for guest in &self.guests {
            write!(f, "{guest:7} ")?;
        }
        writeln!(f)?;
        for i in 0..self.guests.len() {
            write!(f, "{:7} ", self.guests[i])?;
            for j in 0..self.guests.len() {
                write!(
                    f,
                    "{:7} ",
                    self.happiness_rule_matrix[self.get_happiness_rule_matrix_index(i, j)]
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn find_optimal_seating_arrangement_happiness(dinner_description: &DinnerDescription) -> i32 {
    fn get_seating_arrangement_total_happiness(
        dinner_description: &DinnerDescription,
        seating_arrangement: &[usize],
    ) -> i32 {
        let mut total_happiness = 0;
        for i in 0..seating_arrangement.len() {
            let guest1_id = seating_arrangement[i];
            let guest2_id = seating_arrangement[(i + 1) % seating_arrangement.len()];
            total_happiness += dinner_description.get_happiness_change(guest1_id, guest2_id);
            total_happiness += dinner_description.get_happiness_change(guest2_id, guest1_id);
        }
        total_happiness
    }

    fn find_optimal_seating_arrangement_happiness_recursive(
        dinner_description: &DinnerDescription,
        remaining_guests: &[usize],
        seating_arrangement: &[usize],
    ) -> i32 {
        if remaining_guests.is_empty() {
            get_seating_arrangement_total_happiness(dinner_description, seating_arrangement)
        } else {
            let mut max_happiness = 0;
            for i in 0..remaining_guests.len() {
                let mut remaining_guests = remaining_guests.to_vec();
                let mut seating_arrangement = seating_arrangement.to_vec();
                let guest_id = remaining_guests.remove(i);
                seating_arrangement.push(guest_id);
                let happiness = find_optimal_seating_arrangement_happiness_recursive(
                    dinner_description,
                    &remaining_guests,
                    &seating_arrangement,
                );
                if happiness > max_happiness {
                    max_happiness = happiness;
                }
            }
            max_happiness
        }
    }

    let remaining_guests: Vec<usize> = (0..dinner_description.guests.len()).collect();
    let seating_arrangement = Vec::new();
    find_optimal_seating_arrangement_happiness_recursive(dinner_description, &remaining_guests, &seating_arrangement)
}

fn parse_happiness_rule(line: &str) -> ((String, String), i32) {
    let words = line.split_whitespace().collect::<Vec<_>>();
    let subject_name = words[0].to_string();
    let object_name = words[10].trim_end_matches('.').to_string();
    let happiness = words[3].parse::<i32>().unwrap();
    if words[2] == "lose" {
        ((subject_name, object_name), -happiness)
    } else {
        ((subject_name, object_name), happiness)
    }
}

fn parse_dinner_description(input: &str) -> DinnerDescription {
    DinnerDescription::new(&input.lines().map(parse_happiness_rule).collect::<Vec<_>>())
}

fn solve(input: &str, log_fn: Option<fn(&str)>) -> (String, String) {
    let mut dinner_description = parse_dinner_description(input);
    if let Some(log_fn) = log_fn {
        log_fn(&format!("{dinner_description}"));
        log_fn("-----");
    }

    // Part 1: Find the optimal seating arrangement for the given happiness rules.
    if let Some(log_fn) = log_fn {
        log_fn("Part 1");
        log_fn("----------");
        log_fn(&format!("{dinner_description}"));
        log_fn("-----");
    }
    let part1_result = find_optimal_seating_arrangement_happiness(&dinner_description);

    // Part 2: Find the optimal seating arrangement for the given happiness rules, with yourself added.
    dinner_description.ensure_guest("You");
    if let Some(log_fn) = log_fn {
        log_fn("Part 2");
        log_fn("----------");
        log_fn(&format!("{dinner_description}"));
        log_fn("-----");
    }
    let part2_result = find_optimal_seating_arrangement_happiness(&dinner_description);

    (part1_result.to_string(), part2_result.to_string())
}

#[linkme::distributed_slice(crate::SOLUTIONS)]
static SOLUTION: crate::Solution = crate::Solution::new(2015, 13, solve);
