use std::io::BufRead;
use phf::phf_map;

fn input_lines() -> impl Iterator<Item=String> {
    let input_file = std::fs::File::open("input.txt").expect("A file named \"input.txt\" with the problem data must be present in the current directory.");

    // Create a line-based iterator for the file contents.
    let reader = std::io::BufReader::new(input_file);
    reader.lines().map(|l| l.unwrap())
}

#[derive(Copy, Clone)]
enum NodeOperationType {
    Set,
    And,
    Or,
    Not,
    LeftShift,
    RightShift,
}

impl NodeOperationType {
    pub fn apply(&self, i1: u16, i2: Option<u16>) -> u16 {
        match self {
            Self::Set => i1,
            Self::And => i1 & i2.unwrap(),
            Self::Or => i1 | i2.unwrap(),
            Self::Not => !i1,
            Self::LeftShift => i1 << i2.unwrap(),
            Self::RightShift => i1 >> i2.unwrap(),
        }
    }
}

#[derive(Clone)]
struct NodeDescription {
    pub name: String,
    pub operation_type: NodeOperationType,
    pub input1_immediate: Option<u16>,
    pub input1_name: Option<String>,
    pub input2_immediate: Option<u16>,
    pub input2_name: Option<String>,
}

struct CircuitNode {
    pub description: NodeDescription,
    pub value: std::cell::RefCell<Option<u16>>,
}

struct Circuit {
    node_list: std::collections::HashMap<String, CircuitNode>,
}

impl Circuit {
    pub fn new<I>(node_descriptions: I) -> Self where
        I: Iterator<Item=NodeDescription> {
        Circuit {
            node_list: node_descriptions.map(|d| {
                let node_name = d.name.clone();
                let node = CircuitNode {description: d, value: std::cell::RefCell::new(None)};
                (node_name, node)
            }).collect(),
        }
    }

    pub fn get_node_value(&self, node_name: &str) -> u16 {
        let mut remaining_node_names: Vec<String> = Vec::new();
        remaining_node_names.push(node_name.to_string());
        while remaining_node_names.len() > 0 {
            let current_node_name = remaining_node_names.last().unwrap();
            let current_node = &self.node_list[current_node_name];
            
            if current_node.value.borrow().is_some() {
                remaining_node_names.pop();
                continue;
            }

            let input1: Option<u16>;
            if current_node.description.input1_immediate.is_some() {
                input1 = current_node.description.input1_immediate;
            }
            else {
                let input1_node_name = current_node.description.input1_name.as_ref().unwrap();
                input1 = *self.node_list[input1_node_name].value.borrow();
                if input1.is_none() {
                    remaining_node_names.push(input1_node_name.to_string());
                    continue;
                }
            }

            let mut input2: Option<u16> = None;
            if current_node.description.input2_immediate.is_some() {
                input2 = current_node.description.input2_immediate;
            }
            else if current_node.description.input2_name.is_some() {
                let input2_node_name = current_node.description.input2_name.as_ref().unwrap();
                input2 = *self.node_list[input2_node_name].value.borrow();
                if input2.is_none() {
                    remaining_node_names.push(input2_node_name.to_string());
                    continue;
                }
            }

            let current_node_value = current_node.description.operation_type.apply(input1.unwrap(), input2);
            *self.node_list[current_node_name].value.borrow_mut() = Some(current_node_value);
        }

        self.node_list[node_name].value.borrow().unwrap()
    }

    pub fn get_all_node_values(&self) -> std::collections::HashMap<String, u16> {
        self.node_list.keys().map(|name| (name.clone(), self.get_node_value(name))).collect()
    }

    pub fn update_node(&mut self, new_node_description: &NodeDescription) {
        *self.node_list.get_mut(&new_node_description.name).unwrap() = CircuitNode { description: new_node_description.clone(), value: std::cell::RefCell::new(None) };

        for n in self.node_list.values() {
            *n.value.borrow_mut() = None;
        }
    }
}

const NODE_OPERATION_NAME_MAP: phf::Map<&str, NodeOperationType> = phf_map! {
    "AND" => NodeOperationType::And,
    "OR" => NodeOperationType::Or,
    "NOT" => NodeOperationType::Not,
    "LSHIFT" => NodeOperationType::LeftShift,
    "RSHIFT" => NodeOperationType::RightShift,
};

fn parse_node_line(line: &str) -> NodeDescription {
    let node_strings: Vec<&str> = line.split(" -> ").collect();
    let node_source_strings: Vec<&str> = node_strings[0].split(" ").collect();
    
    let name = node_strings[1];
    
    let operation_type: NodeOperationType;
    let mut input1_immediate: Option<u16> = None;
    let mut input1_name: Option<String> = None;
    let mut input2_immediate: Option<u16> = None;
    let mut input2_name: Option<String> = None;
    match node_source_strings.len() {
        1 => {
            match node_source_strings[0].parse::<u16>() {
                Ok(value) => {
                    operation_type = NodeOperationType::Set;
                    input1_immediate = Some(value);
                },
                Err(_) => {
                    operation_type = NodeOperationType::Set;
                    input1_name = Some(node_source_strings[0].to_string());
                }
            }
        },
        2 => {
            operation_type = NodeOperationType::Not;
            input1_name = Some(node_source_strings[1].to_string());
        },
        3 => {
            operation_type = NODE_OPERATION_NAME_MAP[node_source_strings[1]];

            match node_source_strings[0].parse::<u16>() {
                Ok(value) => {
                    input1_immediate = Some(value);
                },
                Err(_) => {
                    input1_name = Some(node_source_strings[0].to_string());
                }
            };

            match node_source_strings[2].parse::<u16>() {
                Ok(value) => {
                    input2_immediate = Some(value);
                },
                Err(_) => {
                    input2_name = Some(node_source_strings[2].to_string());
                }
            };
        },
        _ => panic!("Invalid node input")
    }

    NodeDescription {
        name: name.to_string(),
        operation_type: operation_type,
        input1_immediate: input1_immediate,
        input1_name: input1_name,
        input2_immediate: input2_immediate,
        input2_name: input2_name,
    }
}

fn parse_node_list<I>(input_lines: I) -> impl Iterator<Item=NodeDescription> where
    I: Iterator<Item=String> {
    input_lines.map(|l| parse_node_line(&l))
}

fn main() {
    let mut circuit = Circuit::new(parse_node_list(input_lines()));

    for v in circuit.get_all_node_values() {
        println!("{}: {}", v.0, v.1);
    }
    let value_of_a = circuit.get_node_value("a");
    println!("{}", value_of_a);

    circuit.update_node(&NodeDescription {
        name: String::from("b"),
        operation_type: NodeOperationType::Set,
        input1_immediate: Some(value_of_a),
        input1_name: None,
        input2_immediate: None,
        input2_name: None,
    });
    for v in circuit.get_all_node_values() {
        println!("{}: {}", v.0, v.1);
    }
    println!("{}", circuit.get_node_value("a"));
}
