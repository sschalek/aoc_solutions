// Advent of Code 2015, Day 19: "Medicine for Rudolph"
// https://adventofcode.com/2015/day/19

use priority_queue::PriorityQueue;
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    hash::Hash,
    rc::Rc,
};

// Represents the chemistry space of possible elements.
#[derive(Debug)]
struct Chemistry {
    elements: Vec<String>,
    element_ids: HashMap<String, usize>,
}

impl Chemistry {
    pub fn new() -> Chemistry {
        Chemistry {
            elements: Vec::new(),
            element_ids: HashMap::new(),
        }
    }

    pub fn ensure_element(&mut self, element: &str) -> usize {
        if let Some(element_id) = self.element_ids.get(element) {
            return *element_id;
        }

        let element_id = self.elements.len();
        self.elements.push(element.to_string());
        self.element_ids.insert(element.to_string(), element_id);
        element_id
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Molecule {
    element_ids: Vec<usize>,
}

impl Molecule {
    pub fn new(element_ids: Vec<usize>) -> Molecule {
        Molecule { element_ids }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct ReplacementRule {
    from_element_id: usize,
    to_element_ids: Vec<usize>,
}

impl ReplacementRule {
    pub fn new(from_element_id: usize, to_element_ids: Vec<usize>) -> ReplacementRule {
        ReplacementRule {
            from_element_id,
            to_element_ids,
        }
    }
}

struct MoleculeMachine {
    replacement_rules: HashMap<usize, Vec<ReplacementRule>>,
    replacement_rules_reverse: HashMap<Vec<usize>, usize>,
    chemistry: Rc<RefCell<Chemistry>>,
    terminal_elements: HashSet<usize>,
}

impl MoleculeMachine {
    pub fn new(replacement_rules: Vec<ReplacementRule>, chemistry: Rc<RefCell<Chemistry>>) -> MoleculeMachine {
        let mut machine = MoleculeMachine {
            replacement_rules: HashMap::new(),
            replacement_rules_reverse: HashMap::new(),
            chemistry,
            terminal_elements: HashSet::new(),
        };

        for rule in replacement_rules {
            machine
                .replacement_rules
                .entry(rule.from_element_id)
                .or_insert_with(Vec::new)
                .push(rule.clone());
            machine
                .replacement_rules_reverse
                .insert(rule.to_element_ids.clone(), rule.from_element_id);
        }

        for element_id in machine.chemistry.borrow().element_ids.values() {
            if !machine.replacement_rules.contains_key(element_id) {
                machine.terminal_elements.insert(*element_id);
            }
        }
        machine
    }

    pub fn generate_all_single_replacements(&self, input_molecule: &Molecule) -> Vec<Molecule> {
        let mut unique_molecules = HashSet::new();
        for (i, element_id) in input_molecule.element_ids.iter().enumerate() {
            if let Some(rules) = self.replacement_rules.get(element_id) {
                for rule in rules {
                    let mut new_molecule_list = input_molecule.element_ids.clone();
                    new_molecule_list.splice(i..=i, rule.to_element_ids.iter().copied());
                    unique_molecules.insert(Molecule::new(new_molecule_list));
                }
            }
        }
        unique_molecules.into_iter().collect()
    }

    fn find_shortest_replacement_sequence(&self, start_molecule: &Molecule, end_molecule: &Molecule) -> Option<usize> {
        fn estimate_distance(start_molecule: &Molecule, end_molecule: &Molecule) -> usize {
            let mut distance = 0;
            for (start_element_id, end_element_id) in
                start_molecule.element_ids.iter().zip(end_molecule.element_ids.iter())
            {
                if start_element_id != end_element_id {
                    distance += 1;
                }
            }
            distance += end_molecule.element_ids.len() - start_molecule.element_ids.len();
            distance
        }

        let mut search_queue: PriorityQueue<Molecule, usize> = PriorityQueue::new();
        let mut best_distances = HashMap::new();

        let estimated_distance_to_goal = estimate_distance(start_molecule, end_molecule);
        search_queue.push(start_molecule.clone(), usize::MAX - estimated_distance_to_goal);
        best_distances.insert(start_molecule.clone(), 0);

        let mut i = 0;
        while let Some((current_molecule, score)) = search_queue.pop() {
            let current_distance = *best_distances.get(&current_molecule).unwrap();
            if current_molecule == *end_molecule {
                return Some(current_distance);
            }

            if i % 1000 == 0 {
                println!(
                    "Iteration: {}, queue size: {}, score: {}",
                    i,
                    search_queue.len(),
                    usize::MAX - score
                );
                println!("Current molecule: {current_molecule:#?}");
            }

            let next_distance = current_distance + 1;
            for next_molecule in self.generate_all_single_replacements(&current_molecule) {
                if next_molecule.element_ids.len() > end_molecule.element_ids.len() {
                    continue;
                }

                let current_best_distance = best_distances.get(&next_molecule).copied().unwrap_or(usize::MAX);
                if next_distance < current_best_distance {
                    best_distances.insert(next_molecule.clone(), next_distance);
                    let estimated_distance_to_goal = estimate_distance(&next_molecule, end_molecule);
                    search_queue.push(next_molecule, usize::MAX - estimated_distance_to_goal);
                }
            }
            i += 1;
        }

        None
    }
}

fn parse_element_list(list: &str, chemistry: &mut Chemistry) -> Vec<usize> {
    let mut element_ids = Vec::new();
    let mut current_element_name = String::new();
    for c in list.chars() {
        if c.is_uppercase() {
            if !current_element_name.is_empty() {
                element_ids.push(chemistry.ensure_element(&current_element_name));
                current_element_name.clear();
            }
            current_element_name.push(c);
        } else {
            current_element_name.push(c);
        }
    }
    if !current_element_name.is_empty() {
        element_ids.push(chemistry.ensure_element(&current_element_name));
    }
    element_ids
}

fn parse_replacement_rule(line: &str, chemistry: &mut Chemistry) -> ReplacementRule {
    let mut parts = line.split(" => ");
    let from_element = parts.next().unwrap();
    let to_elements = parts.next().unwrap();

    let from_element_id = chemistry.ensure_element(from_element);
    let to_element_ids = parse_element_list(to_elements, chemistry);
    ReplacementRule::new(from_element_id, to_element_ids)
}

fn parse_replacement_rules(input: &str, chemistry: &mut Chemistry) -> Vec<ReplacementRule> {
    input
        .lines()
        .map(|line| parse_replacement_rule(line, chemistry))
        .collect()
}

fn parse_molecule(input: &str, chemistry: &mut Chemistry) -> Molecule {
    let element_ids = parse_element_list(input, chemistry);
    Molecule { element_ids }
}

fn parse_input(input: &str, chemistry: &mut Chemistry) -> (Vec<ReplacementRule>, Molecule) {
    let input_parts = input.split("\n\n").collect::<Vec<_>>();
    let replacement_rules = parse_replacement_rules(input_parts[0], chemistry);
    let calibration_molecule = parse_molecule(input_parts[1], chemistry);
    (replacement_rules, calibration_molecule)
}

fn solve(input: &str, log_fn: Option<fn(&str)>) -> (String, String) {
    let chemistry = Rc::new(RefCell::new(Chemistry::new()));
    let (replacement_rules, medicine_molecule) = parse_input(input, &mut chemistry.borrow_mut());

    if let Some(log_fn) = log_fn {
        log_fn(&format!("Chemistry: {chemistry:#?}"));
        log_fn(&format!("Replacement rules: {replacement_rules:#?}"));
        log_fn(&format!("Calibration input molecule: {medicine_molecule:#?}"));
    }

    // Part 1: How many distinct molecules can be created after a single replacement of any element
    // in the calibration input molecule?
    let molecule_machine = MoleculeMachine::new(replacement_rules, chemistry.clone());
    let calibration_output_molecules = molecule_machine.generate_all_single_replacements(&medicine_molecule);
    let part1_result = calibration_output_molecules.len();

    if let Some(log_fn) = log_fn {
        log_fn(&format!("Calibration molecules: {calibration_output_molecules:#?}"));
    }

    // Part 2: What is the fewest number of steps to go from the calibration input molecule to the
    // medicine molecule?
    let seed_molecule = Molecule::new(vec![chemistry.borrow_mut().ensure_element("e")]);
    let part2_result = molecule_machine
        .find_shortest_replacement_sequence(&seed_molecule, &medicine_molecule)
        .unwrap();

    (part1_result.to_string(), part2_result.to_string())
}

#[linkme::distributed_slice(crate::SOLUTIONS)]
static SOLUTION: crate::Solution = crate::Solution::new(2015, 19, solve);
