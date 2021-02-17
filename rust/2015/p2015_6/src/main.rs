use std::io::BufRead;

// Represents a square grid of lights that can be turned on and off and dimmed/brightened.
struct LightGrid {
    light_status: Vec<usize>,
    width: usize,
}

impl LightGrid {
    // Creates a new LightGrid instance with a square grid of the given width.
    pub fn new(width: usize) -> LightGrid {
        let light_status = vec![0; width * width];
        LightGrid {
            light_status,
            width,
        }
    }

    // Returns the total brightness of all lights in the grid together.
    pub fn get_total_brightness(&self) -> usize {
        return self.light_status.iter().fold(0, |total, b| total + b);
    }

    // Toggles the given rectangular area of the grid on or off. If a light within the area is off
    // it is turned on with brightness of 1. If a light within the area is on at any level, it is
    // turned off.
    pub fn toggle_rect(&mut self, upper_left: (usize, usize), lower_right: (usize, usize)) {
        self.for_light_rect(upper_left, lower_right, |light_status| {
            *light_status = if *light_status > 0 { 0 } else { 1 };
        });
    }

    // Turns on, with brightness one, each light within the given rectangular area, regardless of
    // its current state.
    pub fn turn_on_rect(&mut self, upper_left: (usize, usize), lower_right: (usize, usize)) {
        self.for_light_rect(upper_left, lower_right, |light_status| {
            *light_status = 1;
        });
    }

    // Turns off each light within the given rectangular area, regardless of its current state.
    pub fn turn_off_rect(&mut self, upper_left: (usize, usize), lower_right: (usize, usize)) {
        self.for_light_rect(upper_left, lower_right, |light_status| {
            *light_status = 0;
        });
    }

    // Increments the brightness of each light within the given rectangular area by the given amount.
    pub fn inc_rect(&mut self, upper_left: (usize, usize), lower_right: (usize, usize), amount: usize) {
        self.for_light_rect(upper_left, lower_right, |light_status| {
            *light_status += amount;
        });
    }

    // Decrements the brightness of each light within the given rectangular area by one, go a minimum of zero (off).
    pub fn dec_rect(&mut self, upper_left: (usize, usize), lower_right: (usize, usize)) {
        self.for_light_rect(upper_left, lower_right, |light_status| {
            if *light_status > 0 { *light_status -= 1 }
        });
    }

    // Runs the given callback on the state of each light within the given rectangular area.
    fn for_light_rect<Cb: Fn (&mut usize)>(&mut self, upper_left: (usize, usize), lower_right: (usize, usize), callback: Cb) {
        for x in upper_left.0..(lower_right.0 + 1) {
            for y in upper_left.1..(lower_right.1 + 1) {
                callback(&mut self.light_status[y * self.width + x]);
            }
        }
    }
}

// Returns an iterator that iterates through each line of the input file.
fn input_lines() -> impl Iterator<Item=String> {
    let input_file = std::fs::File::open("input.txt").expect("A file named \"input.txt\" with the problem data must be present in the current directory.");

    // Create a line-based iterator for the file contents.
    let reader = std::io::BufReader::new(input_file);
    reader.lines().map(|l| l.unwrap())
}

// Parses a point component of an instruction string and returns a 2-tuple of integers representing the point.
fn parse_instruction_point(instruction_point_str: &str) -> (usize, usize) {
    // Split the given string on "," and parse each resulting component of the point as an integer.
    let coordinate_strings: Vec<&str> = instruction_point_str.split(",").collect();
    return (coordinate_strings[0].parse::<usize>().unwrap(), coordinate_strings[1].parse::<usize>().unwrap());
}

// Parses a rectangle component of an instruction string and returns a 2-tuple of point 2-tuples representing
// the rectangle in (upper-left, lower-right) form.
fn parse_instruction_rect(instruction_rect_str: &str) -> ((usize, usize), (usize, usize)) {
    // Split the given string on " through " and parse each resulting component of the rectangle definition as a point.
    let point_strings: Vec<&str> = instruction_rect_str.split(" through ").collect();
    return (parse_instruction_point(point_strings[0]), parse_instruction_point(point_strings[1]));
}

// Processes each instruction of the given instruction string list by calling the appropriate given callback for each different
// instruction type ("toggle", "turn on", or "turn off").
fn process_light_instructions<ToggleFn, OnFn, OffFn>(instruction_iter: impl Iterator<Item=String>, mut toggle_fn: ToggleFn, mut on_fn: OnFn, mut off_fn: OffFn) where
    ToggleFn: FnMut((usize, usize), (usize, usize)),
    OnFn: FnMut((usize, usize), (usize, usize)),
    OffFn: FnMut((usize, usize), (usize, usize)),
    {
    const TOGGLE_STR: &str = "toggle ";
    const TURN_ON_STR: &str = "turn on ";
    const TURN_OFF_STR: &str = "turn off ";

    // Go through each instruction string, determining which instruction type it specifies, parsing out the
    // rectangle it should apply to, and calling the supplied callback for teh determined instruction type.
    for i in instruction_iter {
        if i.find(TOGGLE_STR).is_some() {
            let rect = parse_instruction_rect(&i[TOGGLE_STR.len()..]);
            toggle_fn(rect.0, rect.1);
        }
        else if i.find(TURN_ON_STR).is_some() {
            let rect = parse_instruction_rect(&i[TURN_ON_STR.len()..]);
            on_fn(rect.0, rect.1);
        }
        else if i.find(TURN_OFF_STR).is_some() {
            let rect = parse_instruction_rect(&i[TURN_OFF_STR.len()..]);
            off_fn(rect.0, rect.1);
        }
        else {
            panic!("Invalid input");
        }
    }
}

fn main() {
    // Note: Rc and RefCell are used below to allow the same LightGrid instance to be referenced by the multiple callbacks
    // (for each different instruction) and to allow each closure to dynamically borrow the instance to modify it. Just
    // trying to give the same instance to each closure will fail at compile time, because multiple mutable references to the
    // same instance cannot be given out at the same time.

    // Part 1: Print out the total brightness of the resulting light grid after processing and applying each instruction using the part 1 rules.
    let shared_light_grid1: std::rc::Rc<std::cell::RefCell<LightGrid>> = std::rc::Rc::new(std::cell::RefCell::new(LightGrid::new(1000)));
    process_light_instructions(input_lines(), |ul, lr| { shared_light_grid1.borrow_mut().toggle_rect(ul, lr); }, |ul, lr| { shared_light_grid1.borrow_mut().turn_on_rect(ul, lr); }, |ul, lr| { shared_light_grid1.borrow_mut().turn_off_rect(ul, lr); });
    println!("{}", shared_light_grid1.borrow().get_total_brightness());

    // Part 2: Print out the total brightness of the resulting light grid after processing and applying each instruction using the part 2 rules.
    let shared_light_grid2: std::rc::Rc<std::cell::RefCell<LightGrid>> = std::rc::Rc::new(std::cell::RefCell::new(LightGrid::new(1000)));
    process_light_instructions(input_lines(), |ul, lr| { shared_light_grid2.borrow_mut().inc_rect(ul, lr, 2); }, |ul, lr| { shared_light_grid2.borrow_mut().inc_rect(ul, lr, 1); }, |ul, lr| { shared_light_grid2.borrow_mut().dec_rect(ul, lr); });
    println!("{}", shared_light_grid2.borrow().get_total_brightness());
}
