// Advent of Code 2015, Day 3: "Perfectly Spherical Houses in a Vacuum"
// https://adventofcode.com/2015/day/3

use std::collections::hash_map::Entry;

// Given a character from the input instructions, returns a tuple representing a vector indicating
// which movement direction the instruction character indicates.
fn get_movement_vector_from_char(character: char) -> (isize, isize) {
    match character {
        '<' => (-1, 0),
        '^' => (0, -1),
        '>' => (1, 0),
        'v' => (0, 1),
        _ => panic!("Invalid character"),
    }
}

// Returns the number of unique houses visited by the given number of actor(s) processing
// the given instructions.
fn get_unique_house_count(instructions_string: &str, actor_count: usize) -> usize {
    // This implementation processes the multiple actors moving around between the houses
    // in parallel. This is probably not more efficient given the overhead, but
    // it's interesting to try out the relevant concurrency functionality.

    // Represents the house visitation state that will be shared across all actors visiting houses.
    struct HouseVisitedState {
        pub unique_house_count: usize,
        pub house_visitation_counts: std::collections::HashMap<(isize, isize), usize>,
    }

    impl HouseVisitedState {
        pub fn new() -> HouseVisitedState {
            HouseVisitedState {
                unique_house_count: 1,
                house_visitation_counts: std::collections::HashMap::new(),
            }
        }
    }

    // Set up the state that will be shared between the concurrent threads.
    let shared_visited_state = std::sync::Arc::new(std::sync::Mutex::new(HouseVisitedState::new()));
    shared_visited_state
        .lock()
        .unwrap()
        .house_visitation_counts
        .insert((0, 0), 1);

    // Create a scope that will guarantee all threads started within it have been joined
    // and exited after the scope is exited.
    crossbeam::thread::scope(|s| {
        // Start a thread for each actor moving between the houses.
        for i in 0..actor_count {
            // Set up an iterator that will step through every nth item in the input command string,
            // corresponding to the instructions that this particular actor should process.
            let mut instructions_iterator = instructions_string.chars();
            for _ in 0..i {
                instructions_iterator.next();
            }
            let stepped_instructions_iterator = instructions_iterator.step_by(actor_count);

            // Create a reference to the shared visitation state that can be given to and owned by
            // the thread associated with the current actor being started, and then start the thread
            // for the current actor.
            let shared_visited_state = std::sync::Arc::clone(&shared_visited_state);
            s.spawn(move |_| {
                // Start out at location 0, 0, and move according to each instruction returned
                // by the instruction iterator for this actor.
                let mut location = (0, 0);
                for v in stepped_instructions_iterator.map(get_movement_vector_from_char) {
                    location = (location.0 + v.0, location.1 + v.1);

                    // Update the visited count at the new location after processing the current
                    // movement instruction. If the house at the new location is newly visited, then
                    // mark it so that the unique house count can be incremented.
                    let mut visited_state = shared_visited_state.lock().unwrap();
                    let mut was_vacant = false;
                    match visited_state.house_visitation_counts.entry(location) {
                        Entry::Occupied(e) => *e.into_mut() += 1,
                        Entry::Vacant(e) => {
                            was_vacant = true;
                            e.insert(1);
                        }
                    }

                    if was_vacant {
                        visited_state.unique_house_count += 1;
                    }
                }
            });
        }
    })
    .unwrap();

    return shared_visited_state.lock().unwrap().unique_house_count;
}

fn solve(input: &str, _log_fn: Option<fn(&str)>) -> (String, String) {
    // Part 1: Find the number of unique houses visited when only one actor is processing
    // the instruction string.
    let unique_house_count = get_unique_house_count(input, 1);

    // Part 2: Find the number of unique houses visited when two actors are processing
    // the instruction string.
    let unique_house_count_two_actors = get_unique_house_count(input, 2);

    (
        unique_house_count.to_string(),
        unique_house_count_two_actors.to_string(),
    )
}

#[linkme::distributed_slice(crate::SOLUTIONS)]
static SOLUTION: crate::Solution = crate::Solution::new(2015, 3, solve);
