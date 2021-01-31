use std::io::prelude::Read;
use std::collections::hash_map::Entry;
extern crate crossbeam;

fn get_instructions_string() -> String {
    let mut input_file = std::fs::File::open("input.txt").expect("A file named \"input.txt\" with the problem data must be present in the current directory.");
    let mut instructions_string = String::new();
    input_file.read_to_string(&mut instructions_string).expect("Unable to read input.");
    return instructions_string;
}

fn get_movement_vector_from_char(character: char) -> (i32, i32) {
    return match character {
        '<' => (-1, 0),
        '^' => (0, -1),
        '>' => (1, 0),
        'v' => (0, 1),
        _ => panic!("Invalid character")
    }
}

fn get_unique_house_count(instructions_string: &String, actor_count: usize) -> i32 {
    // This implementation processes the multiple actors moving around between the houses
    // in paralell. This is almost certainly not more efficient given the overhead, but
    // it's interesting to try out the relevant concurrency functionality.

    // Set up the state that will be shared between the concurrent threads:
    //  * The count of unique houses visited
    //  * The map of visited locations to how many times each has been visited
    let shared_visited_state = std::sync::Arc::new(std::sync::Mutex::new(
        (1, std::collections::HashMap::new())
    ));
    shared_visited_state.lock().unwrap().1.insert((0, 0), 1);

    // Create a scope that will guarantee all threads started within it have been joined
    // and exited after the scope is exited.
    crossbeam::thread::scope(|s| {
        // Start a thread for each "actor" moving between the houses.
        for i in 0..actor_count {
            // Set up an iterator that will step through every nth item in the input command string,
            // corresponding to the instructions that this particular actor should process.
            let mut instructions_iterator = instructions_string.chars();
            for _ in 0..i {instructions_iterator.next();}
            let stepped_instructions_iterator = instructions_iterator.step_by(actor_count);

            let shared_visited_state = std::sync::Arc::clone(&shared_visited_state);
            s.spawn(move |_| {
                let mut location = (0, 0);
                for v in stepped_instructions_iterator.map(|c| get_movement_vector_from_char(c)) {
                    location = (location.0 + v.0, location.1 + v.1);

                    let mut visited_state = shared_visited_state.lock().unwrap();
                    let mut was_vacant = false;
                    match (*visited_state).1.entry(location) {
                        Entry::Occupied(e) => *e.into_mut() += 1,
                        Entry::Vacant(e) => {
                            was_vacant = true;
                            e.insert(1);
                        }
                    }
                    if was_vacant {
                        (*visited_state).0 += 1;
                    }
                }
            });
        }
    }).unwrap();

    return shared_visited_state.lock().unwrap().0;
}

fn main() {
    let instructions_string = get_instructions_string();
    println!("{}", get_unique_house_count(&instructions_string, 1));
    println!("{}", get_unique_house_count(&instructions_string, 2));
}
