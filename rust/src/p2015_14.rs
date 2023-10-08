// Advent of Code 2015, Day 14: "Reindeer Olympics"
// https://adventofcode.com/2015/day/14

// Represents the desecription of a reindeer's characteristics.
#[derive(Clone, Debug)]
struct ReindeerDescription {
    #[allow(dead_code)]
    name: String,
    speed: u32,
    fly_time: u32,
    rest_time: u32,
}

// Parses a string into a ReindeerDescription.
fn parse_reindeer_description(line: &str) -> ReindeerDescription {
    let words = line.split_whitespace().collect::<Vec<_>>();
    let integers = words.iter().filter_map(|w| w.parse::<u32>().ok()).collect::<Vec<_>>();
    ReindeerDescription {
        name: words[0].to_string(),
        speed: integers[0],
        fly_time: integers[1],
        rest_time: integers[2],
    }
}

// Parses a string into a vector of ReindeerDescriptions, one for each line in the string.
fn parse_reindeer_descriptions(input: &str) -> Vec<ReindeerDescription> {
    input.lines().map(parse_reindeer_description).collect()
}

// Identifies the state of a reindeer's movement.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ReindeerMovementState {
    Flying,
    Resting,
}

// Represents a reindeer, including its capabilities and current state.
#[derive(Clone, Debug)]
struct Reindeer {
    description: ReindeerDescription,
    distance_traveled: u32,
    state: ReindeerMovementState,
    state_ticks_remaining: u32,
}

impl Reindeer {
    // Creates a new Reindeer with the given description.
    fn new(description: ReindeerDescription) -> Reindeer {
        let state_ticks_remaining = description.fly_time;
        Reindeer {
            description,
            distance_traveled: 0,
            state: ReindeerMovementState::Flying,
            state_ticks_remaining,
        }
    }

    // Advances the reindeer's state for the given number of ticks.
    fn advance(&mut self, ticks: u32) {
        // Continue advancing the reindeer's state until the given number of ticks has been used up.
        let mut ticks_remaining = ticks;
        while ticks_remaining > 0 {
            // Determine the number of ticks to advance by the minimum of the number of ticks remaining
            // in the current advancement interval and the number of ticks remaining for the reindeer's
            // current state. This ensures that only a single state will be processed in each iteration
            // and that no more than the given number of ticks will be advanced.
            let ticks_to_advance = ticks_remaining.min(self.state_ticks_remaining);

            // If the reindeer is flying, advance its distance traveled by the distance it would travel
            // in the current advancement interval.
            if self.state == ReindeerMovementState::Flying {
                self.distance_traveled += ticks_to_advance * self.description.speed;
            }

            // Reduce the number of ticks remaining for the reindeer's current state, and if the state
            // has finished, switch to the next state.
            self.state_ticks_remaining -= ticks_to_advance;
            if self.state_ticks_remaining == 0 {
                match self.state {
                    ReindeerMovementState::Flying => {
                        self.state = ReindeerMovementState::Resting;
                        self.state_ticks_remaining = self.description.rest_time;
                    }
                    ReindeerMovementState::Resting => {
                        self.state = ReindeerMovementState::Flying;
                        self.state_ticks_remaining = self.description.fly_time;
                    }
                }
            }

            ticks_remaining -= ticks_to_advance;
        }
    }
}

// Represents a single reindeer race.
#[derive(Clone, Debug)]
struct ReindeerRace {
    reindeer_race_states: Vec<(Reindeer, u32)>,
    time_elapsed: u32,
}

impl ReindeerRace {
    // Creates a new ReindeerRace for reindeer with the given descriptions.
    fn new(descriptions: &[ReindeerDescription]) -> ReindeerRace {
        let reindeer_race_states = descriptions.iter().map(|d| (Reindeer::new(d.clone()), 0)).collect();
        ReindeerRace {
            reindeer_race_states,
            time_elapsed: 0,
        }
    }

    // Advances the race (and all reindeer) by the given number of ticks.
    fn advance(&mut self, ticks: u32) {
        let mut ticks_remaining = ticks;
        while ticks_remaining > 0 {
            // Advance each reindeer by 1 tick.
            for r in &mut self.reindeer_race_states {
                r.0.advance(1);
            }

            // Award a point to each reindeer that is in the lead.
            let max_distance = self.get_max_distance_traveled();
            for r in &mut self.reindeer_race_states {
                if r.0.distance_traveled == max_distance {
                    r.1 += 1;
                }
            }

            ticks_remaining -= 1;
        }

        self.time_elapsed += ticks;
    }

    // Returns the maximum distance any reindeer has traveled.
    fn get_max_distance_traveled(&self) -> u32 {
        self.reindeer_race_states
            .iter()
            .map(|r| r.0.distance_traveled)
            .max()
            .unwrap()
    }

    // Returns the reindeer currently in the lead.
    fn get_lead_reindeer(&self) -> Vec<&Reindeer> {
        let max_distance = self.get_max_distance_traveled();
        self.reindeer_race_states
            .iter()
            .filter(|r| r.0.distance_traveled == max_distance)
            .map(|r| &r.0)
            .collect::<Vec<_>>()
    }

    // Returns the reindeer with the most points.
    fn get_winning_reindeer(&self) -> &(Reindeer, u32) {
        self.reindeer_race_states.iter().max_by_key(|r| r.1).unwrap()
    }
}

fn solve(input: &str, log_fn: Option<fn(&str)>) -> (String, String) {
    let reindeer_descriptions = parse_reindeer_descriptions(input);
    let mut race = ReindeerRace::new(&reindeer_descriptions);

    // Part 1: Find the distance the lead reindeer has traveled after 2503 seconds.
    race.advance(2503);
    let part1_result = race.get_max_distance_traveled();

    if let Some(log_fn) = log_fn {
        log_fn("Part 1");
        log_fn("----------");
        log_fn(&format!("Lead reindeer: {:#?}", race.get_lead_reindeer()));
        log_fn("-----");
        log_fn(&format!("{race:#?}"));
        log_fn("-----");
    }

    // Part 2: Find the number of points the winning reindeer has after 2503 seconds.
    let winning_reindeer = race.get_winning_reindeer();
    let part2_result = winning_reindeer.1;

    if let Some(log_fn) = log_fn {
        log_fn("Part 2");
        log_fn("----------");
        log_fn(&format!("Lead reindeer: {:#?}", race.get_lead_reindeer()));
        log_fn(&format!("Winning reindeer: {winning_reindeer:#?}"));
        log_fn("-----");
        log_fn(&format!("{race:#?}"));
        log_fn("-----");
    }

    (part1_result.to_string(), part2_result.to_string())
}

#[linkme::distributed_slice(crate::SOLUTIONS)]
static SOLUTION: crate::Solution = crate::Solution::new(2015, 14, solve);
