use std::io::BufRead;

struct LightGrid {
    light_status: Vec<usize>,
    width: usize,
}

impl LightGrid {
    pub fn new(width: usize, height: usize) -> LightGrid {
        let light_status = vec![0; width * height];
        LightGrid {
            light_status,
            width,
        }
    }

    pub fn get_total_brightness(&self) -> usize {
        return self.light_status.iter().fold(0, |total, b| total + b);
    }

    pub fn toggle_rect(&mut self, upper_left: (usize, usize), lower_right: (usize, usize)) {
        self.for_light_rect(upper_left, lower_right, |light_status| {
            *light_status = if *light_status > 0 { 0 } else { 1 };
        });
    }

    pub fn turn_on_rect(&mut self, upper_left: (usize, usize), lower_right: (usize, usize)) {
        self.for_light_rect(upper_left, lower_right, |light_status| {
            *light_status = 1;
        });
    }

    pub fn turn_off_rect(&mut self, upper_left: (usize, usize), lower_right: (usize, usize)) {
        self.for_light_rect(upper_left, lower_right, |light_status| {
            *light_status = 0;
        });
    }

    pub fn inc_rect(&mut self, upper_left: (usize, usize), lower_right: (usize, usize), amount: usize) {
        self.for_light_rect(upper_left, lower_right, |light_status| {
            *light_status += amount;
        });
    }

    pub fn dec_rect(&mut self, upper_left: (usize, usize), lower_right: (usize, usize)) {
        self.for_light_rect(upper_left, lower_right, |light_status| {
            if *light_status > 0 { *light_status -= 1 }
        });
    }

    fn for_light_rect<Cb: Fn (&mut usize)>(&mut self, upper_left: (usize, usize), lower_right: (usize, usize), callback: Cb) {
        for x in upper_left.0..(lower_right.0 + 1) {
            for y in upper_left.1..(lower_right.1 + 1) {
                callback(&mut self.light_status[y * self.width + x]);
            }
        }
    }
}

fn input_lines() -> impl Iterator<Item=String> {
    let input_file = std::fs::File::open("input.txt").expect("A file named \"input.txt\" with the problem data must be present in the current directory.");

    // Create a line-based iterator for the file contents.
    let reader = std::io::BufReader::new(input_file);
    return reader.lines().map(|l| l.unwrap());
}

fn parse_instruction_point(instruction_point_str: &str) -> (usize, usize) {
    let coordinate_strings: Vec<&str> = instruction_point_str.split(",").collect();
    return (coordinate_strings[0].parse::<usize>().unwrap(), coordinate_strings[1].parse::<usize>().unwrap());
}

fn parse_instruction_rect(instruction_rect_str: &str) -> ((usize, usize), (usize, usize)) {
    let point_strings: Vec<&str> = instruction_rect_str.split(" through ").collect();
    return (parse_instruction_point(point_strings[0]), parse_instruction_point(point_strings[1]));
}

fn process_light_instructions<ToggleFn, OnFn, OffFn>(instruction_iter: impl Iterator<Item=String>, mut toggle_fn: ToggleFn, mut on_fn: OnFn, mut off_fn: OffFn) where
    ToggleFn: FnMut((usize, usize), (usize, usize)),
    OnFn: FnMut((usize, usize), (usize, usize)),
    OffFn: FnMut((usize, usize), (usize, usize)),
    {
    const TOGGLE_STR: &str = "toggle ";
    const TURN_ON_STR: &str = "turn on ";
    const TURN_OFF_STR: &str = "turn off ";

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
    let shared_light_grid1: std::rc::Rc<std::cell::RefCell<LightGrid>> = std::rc::Rc::new(std::cell::RefCell::new(LightGrid::new(1000, 1000)));
    process_light_instructions(input_lines(), |ul, lr| { shared_light_grid1.borrow_mut().toggle_rect(ul, lr); }, |ul, lr| { shared_light_grid1.borrow_mut().turn_on_rect(ul, lr); }, |ul, lr| { shared_light_grid1.borrow_mut().turn_off_rect(ul, lr); });
    println!("{}", shared_light_grid1.borrow().get_total_brightness());

    let shared_light_grid2: std::rc::Rc<std::cell::RefCell<LightGrid>> = std::rc::Rc::new(std::cell::RefCell::new(LightGrid::new(1000, 1000)));
    process_light_instructions(input_lines(), |ul, lr| { shared_light_grid2.borrow_mut().inc_rect(ul, lr, 2); }, |ul, lr| { shared_light_grid2.borrow_mut().inc_rect(ul, lr, 1); }, |ul, lr| { shared_light_grid2.borrow_mut().dec_rect(ul, lr); });
    println!("{}", shared_light_grid2.borrow().get_total_brightness());
}
