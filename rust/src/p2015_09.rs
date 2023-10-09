// Advent of Code 2015, Day 9: "All in a Single Night"
// https://adventofcode.com/2015/day/9

use std::collections::HashMap;

struct CityGraph {
    cities: Vec<String>,
    distance_matrix: Vec<Option<u32>>,
}

impl CityGraph {
    fn new(distance_list: &[(String, String, u32)]) -> Self {
        fn ensure_city_id(cities: &mut Vec<String>, city_name: &str) -> usize {
            if let Some(city_id) = cities.iter().position(|c| c == city_name) {
                city_id
            } else {
                cities.push(city_name.to_string());
                cities.len() - 1
            }
        }

        let mut graph = CityGraph {
            cities: Vec::new(),
            distance_matrix: Vec::new(),
        };

        for (city_name1, city_name2, distance) in distance_list {
            let city_id1 = ensure_city_id(&mut graph.cities, city_name1);
            let city_id2 = ensure_city_id(&mut graph.cities, city_name2);
            let index = graph.get_distance_matrix_index(city_id1, city_id2);
            graph.distance_matrix.resize(index + 1, None);
            graph.distance_matrix[index] = Some(*distance);
        }
        graph
    }

    fn get_distance_by_id(&self, city_id1: usize, city_id2: usize) -> Option<u32> {
        let index = self.get_distance_matrix_index(city_id1, city_id2);
        self.distance_matrix[index]
    }

    fn get_distance_matrix_index(&self, city_id1: usize, city_id2: usize) -> usize {
        let min_id = std::cmp::min(city_id1, city_id2);
        let max_id = std::cmp::max(city_id1, city_id2);
        (self.cities.len() * min_id) + max_id
    }
}

struct PathSolver<'a> {
    graph: &'a CityGraph,
    known_distances: HashMap<(Vec<usize>, usize), u32>,
}

impl<'a> PathSolver<'a> {
    pub fn new(graph: &'a CityGraph) -> Self {
        Self {
            graph,
            known_distances: HashMap::new(),
        }
    }

    pub fn find_shortest_hamiltonian_path_distance(&mut self) -> Option<u32> {
        let target_city_id = 0;
        let through_city_ids = (1..=self.graph.cities.len()).collect::<Vec<_>>();
        self.find_optimized_path_distance_memoize(&through_city_ids, target_city_id, true)
    }

    pub fn find_longest_hamiltonian_path_distance(&mut self) -> Option<u32> {
        let target_city_id = 0;
        let through_city_ids = (1..=self.graph.cities.len()).collect::<Vec<_>>();
        self.find_optimized_path_distance_memoize(&through_city_ids, target_city_id, false)
    }

    fn find_optimized_path_distance_memoize(
        &mut self,
        through_city_ids: &[usize],
        target_city_id: usize,
        find_shortest: bool,
    ) -> Option<u32> {
        let key = (through_city_ids.to_vec(), target_city_id);
        if let Some(distance) = self.known_distances.get(&key) {
            return Some(*distance);
        }

        let distance = self.find_optimized_path_distance(through_city_ids, target_city_id, find_shortest);
        if let Some(distance) = distance {
            self.known_distances.insert(key, distance);
        }
        distance
    }

    fn find_optimized_path_distance(
        &mut self,
        through_city_ids: &[usize],
        target_city_id: usize,
        find_shortest: bool,
    ) -> Option<u32> {
        if through_city_ids.is_empty() {
            // println!(
            //     "through_city_ids: {:?}, target_city_id: {}, shortest_distance: {:?}",
            //     through_city_ids, target_city_id, 0
            // );
            return Some(0);
        }

        if through_city_ids.len() == 1 {
            // println!(
            //     "through_city_ids: {:?}, target_city_id: {}, shortest_distance: {:?}",
            //     through_city_ids,
            //     target_city_id,
            //     self.get_city_distance(through_city_ids[0], target_city_id)
            // );
            return self.get_city_distance(through_city_ids[0], target_city_id);
        }

        let mut shortest_distance = None;
        for id in through_city_ids {
            let mut remaining_city_ids = through_city_ids.to_vec();
            remaining_city_ids.retain(|c| c != id);
            let distance = self.find_optimized_path_distance_memoize(&remaining_city_ids, *id, find_shortest);
            if let Some(distance) = distance {
                let total_distance = self.get_city_distance(*id, target_city_id).unwrap() + distance;
                if shortest_distance.is_none()
                    || (find_shortest && total_distance < shortest_distance.unwrap())
                    || (!find_shortest && total_distance > shortest_distance.unwrap())
                {
                    shortest_distance = Some(total_distance);
                }
            }
        }
        // println!(
        //     "through_city_ids: {:?}, target_city_id: {}, shortest_distance: {:?}",
        //     through_city_ids, target_city_id, shortest_distance
        // );
        shortest_distance
    }

    // Returns the distance between the two cities with the given IDs or None if no path exists between them,
    // where 0 is the virtual starting city.
    fn get_city_distance(&self, city_id1: usize, city_id2: usize) -> Option<u32> {
        if city_id1 == 0 || city_id2 == 0 {
            return Some(0);
        }
        self.graph.get_distance_by_id(city_id1 - 1, city_id2 - 1)
    }
}

// Parses the given input string into a CityGraph instance.
fn parse_city_graph(input: &str) -> CityGraph {
    let mut distance_list = Vec::new();
    for line in input.lines() {
        let mut parts = line.split(" = ");
        let mut cities = parts.next().unwrap().split(" to ");
        let distance = parts.next().unwrap().parse::<u32>().unwrap();
        distance_list.push((
            cities.next().unwrap().to_string(),
            cities.next().unwrap().to_string(),
            distance,
        ));
    }
    CityGraph::new(&distance_list)
}

fn solve(input: &str, _log_fn: Option<fn(&str)>) -> (String, String) {
    let graph = parse_city_graph(input);
    // println!("graph: {:?}", graph.cities);

    // Part 1: Find the shortest Hamiltonian path through the given cities.
    let mut shortest_path_solver = PathSolver::new(&graph);
    let part1_result = shortest_path_solver.find_shortest_hamiltonian_path_distance().unwrap();

    // Part 2: Find the longest Hamiltonian path through the given cities.
    let mut longest_path_solver = PathSolver::new(&graph);
    let part2_result = longest_path_solver.find_longest_hamiltonian_path_distance().unwrap();

    (part1_result.to_string(), part2_result.to_string())
}

#[linkme::distributed_slice(crate::SOLUTIONS)]
static SOLUTION: crate::Solution = crate::Solution::new(2015, 9, solve);
