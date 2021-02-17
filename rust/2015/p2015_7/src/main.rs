use std::io::BufRead;
use phf::phf_map;

// Returns an iterator that iterates through each line of the input file.
fn input_lines() -> impl Iterator<Item=String> {
    let input_file = std::fs::File::open("input.txt").expect("A file named \"input.txt\" with the problem data must be present in the current directory.");

    // Create a line-based iterator for the file contents.
    let reader = std::io::BufReader::new(input_file);
    reader.lines().map(|l| l.unwrap())
}

// Represents the different operations a node in the circuit can perform.
#[derive(Copy, Clone)]
enum NodeOperation {
    Set,
    And,
    Or,
    Not,
    LeftShift,
    RightShift,
}

impl NodeOperation {
    // Applies this operation to the given inputs and returns the result.
    // The second input parameter is optional, because not all operations
    // take two inputs.
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

// Represents an input to a node. A node input can either be an immediate value
// or the output of another node.
#[derive(Clone)]
enum NodeInput {
    Immediate(u16),
    Node(String),
}

// Describes a circuit node. This is only the description of the node, and it
// does not contain any state related to the node in a circuit.
#[derive(Clone)]
struct NodeDescription {
    pub name: String,
    pub operation: NodeOperation,
    pub input1: NodeInput,
    pub input2: Option<NodeInput>,
}

// Represents a node in a circuit, including both the description of the node's
// behavior and its current state.
struct CircuitNode {
    pub description: NodeDescription,
    pub value: std::cell::RefCell<Option<u16>>,
}

// Represents a circuit of connected nodes.
struct Circuit {
    node_list: std::collections::HashMap<String, CircuitNode>,
}

impl Circuit {
    // Creates a new circuit from the given node descriptions.
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

    // Returns the output value of the node with the given name.
    pub fn get_node_value(&self, node_name: &str) -> u16 {
        // Push the requested node node onto a stack of remaining node names to process,
        // and being processing the stack of node names. If the node specified by the name
        // on top of the stack has a value, pop it off. If the node doesn't have a value,
        // try to calculate it from its inputs. If the inputs themselves don't have values
        // push their names onto the stack and continue until the values for all relevant
        // nodes are determined.
        let mut remaining_node_names: Vec<String> = Vec::new();
        remaining_node_names.push(node_name.to_string());
        while remaining_node_names.len() > 0 {
            // Get the node corresponding to the name on top of the stack.
            let current_node_name = remaining_node_names.last().unwrap();
            let current_node = &self.node_list[current_node_name];
            
            // If the current node already has a value, pop it off the stack and continue;
            // it doesn't need to be processed any further.
            if current_node.value.borrow().is_some() {
                remaining_node_names.pop();
                continue;
            }

            // Since the current node doesn't have a value already, try to calculate it.
            // Start by trying to get its first input. If the first input is an immediate
            // input, then it is already available. If it is a node input, then check whether
            // the input node has a value. If it does, use it. If it doesn't push it onto the stack
            // for processing so that its value can be determined before coming back to determining
            // the value for the current node.
            let input1: Option<u16>;
            match &current_node.description.input1 {
                NodeInput::Immediate(value) => input1 = Some(*value),
                NodeInput::Node(name) => {
                    input1 = *self.node_list[name].value.borrow();
                    if input1.is_none() {
                        remaining_node_names.push(name.to_string());
                        continue;
                    }    
                }
            }

            // Do the same thing for the second node input, but only if this node actually
            // has a second input.
            let mut input2: Option<u16> = None;
            if current_node.description.input2.is_some() {
                match current_node.description.input2.as_ref().unwrap() {
                    NodeInput::Immediate(value) => input2 = Some(*value),
                    NodeInput::Node(name) => {
                        input2 = *self.node_list[name].value.borrow();
                        if input2.is_none() {
                            remaining_node_names.push(name.to_string());
                            continue;
                        }    
                    }
                }
            }

            // Now that any necessary inputs for the current node have been determined, apply the current
            // node's operation to the inputs to determine the current node's value.
            let current_node_value = current_node.description.operation.apply(input1.unwrap(), input2);
            *self.node_list[current_node_name].value.borrow_mut() = Some(current_node_value);
        }

        // Now that all relevant nodes have been processed, return the value associated with the requested node.
        self.node_list[node_name].value.borrow().unwrap()
    }

    // Returns the output values of all nodes in the circuit in the form of a mapping from
    // node name to node output value.
    pub fn get_all_node_values(&self) -> std::collections::HashMap<String, u16> {
        self.node_list.keys().map(|name| (name.clone(), self.get_node_value(name))).collect()
    }

    // Updates the circuit with the given node description. If a node with the same name as the given node already
    // exists, it will be replaced.
    pub fn update_node(&mut self, new_node_description: &NodeDescription) {
        *self.node_list.get_mut(&new_node_description.name).unwrap() = CircuitNode { description: new_node_description.clone(), value: std::cell::RefCell::new(None) };

        for n in self.node_list.values() {
            *n.value.borrow_mut() = None;
        }
    }
}

// Maps the names of node operations to the actual operations.
const NODE_OPERATION_NAME_MAP: phf::Map<&str, NodeOperation> = phf_map! {
    "AND" => NodeOperation::And,
    "OR" => NodeOperation::Or,
    "NOT" => NodeOperation::Not,
    "LSHIFT" => NodeOperation::LeftShift,
    "RSHIFT" => NodeOperation::RightShift,
};

// Given a string describing a node, parses it and returns a corresponding node description.
fn parse_node_line(line: &str) -> NodeDescription {
    let node_strings: Vec<&str> = line.split(" -> ").collect();
    let node_source_strings: Vec<&str> = node_strings[0].split(" ").collect();
    
    let name = node_strings[1];
    
    let operation: NodeOperation;
    let input1: NodeInput;
    let mut input2: Option<NodeInput> = None;
    match node_source_strings.len() {
        1 => {
            operation = NodeOperation::Set;
            input1 = match node_source_strings[0].parse::<u16>() {
                Ok(value) => NodeInput::Immediate(value),
                Err(_) => NodeInput::Node(node_source_strings[0].to_string())
            };
        },
        2 => {
            operation = NodeOperation::Not;
            input1 = NodeInput::Node(node_source_strings[1].to_string());
        },
        3 => {
            operation = NODE_OPERATION_NAME_MAP[node_source_strings[1]];

            input1 = match node_source_strings[0].parse::<u16>() {
                Ok(value) => NodeInput::Immediate(value),
                Err(_) => NodeInput::Node(node_source_strings[0].to_string())
            };

            input2 = Some(match node_source_strings[2].parse::<u16>() {
                Ok(value) => NodeInput::Immediate(value),
                Err(_) => NodeInput::Node(node_source_strings[2].to_string())
            });
        },
        _ => panic!("Invalid node input")
    }

    NodeDescription {
        name: name.to_string(),
        operation: operation,
        input1: input1,
        input2: input2,
    }
}

// Parses the circuit node descriptions specified by the given input lines and returns
// an iterator over NodeDescriptions.
fn parse_node_list<I>(input_lines: I) -> impl Iterator<Item=NodeDescription> where
    I: Iterator<Item=String> {
    input_lines.map(|l| parse_node_line(&l))
}

fn main() {
    // Create a circuit from the node list specified by the input.
    let mut circuit = Circuit::new(parse_node_list(input_lines()));

    // Print out the status of each node in the circuit.
    for v in circuit.get_all_node_values() {
        println!("{}: {}", v.0, v.1);
    }

    // Part 1: Print out the value of node "a" in the circuit specified by the input.
    let value_of_a = circuit.get_node_value("a");
    println!("------------------------------");
    println!("{}", value_of_a);
    println!("------------------------------");

    // Update the circuit with a new node "b".
    circuit.update_node(&NodeDescription {
        name: String::from("b"),
        operation: NodeOperation::Set,
        input1: NodeInput::Immediate(value_of_a),
        input2: None,
    });

    // Print out the status of each node in the circuit.
    for v in circuit.get_all_node_values() {
        println!("{}: {}", v.0, v.1);
    }

    // Part 2: Print out the value of node "a" in the modified circuit.
    println!("------------------------------");
    println!("{}", circuit.get_node_value("a"));
    println!("------------------------------");
}
