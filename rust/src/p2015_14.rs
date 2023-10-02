#[derive(Clone, Debug)]
struct ReindeerDescription {
    #[allow(dead_code)]
    name: String,
    speed: u32,
    fly_time: u32,
    rest_time: u32,
}

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

fn parse_reindeer_descriptions(input: &str) -> Vec<ReindeerDescription> {
    input.lines().map(parse_reindeer_description).collect()
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ReindeerState {
    Flying,
    Resting,
}

#[derive(Clone, Debug)]
struct Reindeer {
    description: ReindeerDescription,
    distance_traveled: u32,
    state: ReindeerState,
    state_ticks_remaining: u32,
}

impl Reindeer {
    fn new(description: ReindeerDescription) -> Reindeer {
        let state_ticks_remaining = description.fly_time;
        Reindeer {
            description,
            distance_traveled: 0,
            state: ReindeerState::Flying,
            state_ticks_remaining,
        }
    }

    fn advance(&mut self, ticks: u32) {
        let mut ticks_remaining = ticks;
        while ticks_remaining > 0 {
            let ticks_to_advance = ticks_remaining.min(self.state_ticks_remaining);

            if self.state == ReindeerState::Flying {
                self.distance_traveled += ticks_to_advance * self.description.speed;
            }

            self.state_ticks_remaining -= ticks_to_advance;
            if self.state_ticks_remaining == 0 {
                match self.state {
                    ReindeerState::Flying => {
                        self.state = ReindeerState::Resting;
                        self.state_ticks_remaining = self.description.rest_time;
                    }
                    ReindeerState::Resting => {
                        self.state = ReindeerState::Flying;
                        self.state_ticks_remaining = self.description.fly_time;
                    }
                }
            }

            ticks_remaining -= ticks_to_advance;
        }
    }
}

#[derive(Clone, Debug)]
struct ReindeerRace {
    reindeer_race_states: Vec<(Reindeer, u32)>,
    time_elapsed: u32,
}

impl ReindeerRace {
    fn new(descriptions: &[ReindeerDescription]) -> ReindeerRace {
        let reindeer_race_states = descriptions.iter().map(|d| (Reindeer::new(d.clone()), 0)).collect();
        ReindeerRace {
            reindeer_race_states,
            time_elapsed: 0,
        }
    }

    fn advance(&mut self, ticks: u32) {
        let mut ticks_remaining = ticks;
        while ticks_remaining > 0 {
            for r in &mut self.reindeer_race_states {
                r.0.advance(1);
            }

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

    fn get_max_distance_traveled(&self) -> u32 {
        self.reindeer_race_states
            .iter()
            .map(|r| r.0.distance_traveled)
            .max()
            .unwrap()
    }

    fn get_lead_reindeer(&self) -> Vec<&Reindeer> {
        let max_distance = self.get_max_distance_traveled();
        self.reindeer_race_states
            .iter()
            .filter(|r| r.0.distance_traveled == max_distance)
            .map(|r| &r.0)
            .collect::<Vec<_>>()
    }

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
        log_fn(&format!("Lead reindeer: {:?}", race.get_lead_reindeer()));
        log_fn("-----");
        log_fn(&format!("{race:?}"));
        log_fn("-----");
    }

    // Part 2: Find the number of points the winning reindeer has after 2503 seconds.
    let winning_reindeer = &race.get_winning_reindeer();
    let part2_result = winning_reindeer.1;

    if let Some(log_fn) = log_fn {
        log_fn("Part 2");
        log_fn("----------");
        log_fn(&format!("Lead reindeer: {:?}", race.get_lead_reindeer()));
        log_fn(&format!("Winning reindeer: {winning_reindeer:?}"));
        log_fn("-----");
        log_fn(&format!("{race:?}"));
        log_fn("-----");
    }

    (part1_result.to_string(), part2_result.to_string())
}

#[linkme::distributed_slice(crate::SOLUTIONS)]
static SOLUTION: crate::Solution = crate::Solution::new(2015, 14, solve);
