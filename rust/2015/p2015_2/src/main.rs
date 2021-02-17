use std::io::BufRead;

// Returns an iterator that iterates through each line of the input file.
fn input_lines() -> impl Iterator<Item=String> {
    let input_file = std::fs::File::open("input.txt").expect("A file named \"input.txt\" with the problem data must be present in the current directory.");

    // Create a line-based iterator for the file contents.
    let reader = std::io::BufReader::new(input_file);
    reader.lines().map(|l| l.unwrap())
}

// Returns an iterator that iterates through the package dimensions specified by the input data as
// tuples of three integers each, one integer for each dimension.
fn package_dimensions() -> impl Iterator<Item=(i32, i32, i32)> {
    // Create an iterator that returns the dimensions of each package in a tuple from the line-based iterator.
    // Split each input line on "x" and parse each of the resulting dimension parts as an integer.
    let dimensions = input_lines().map(|l| l.split("x").map(|d| d.parse::<i32>().unwrap()).collect::<Vec<_>>());
    dimensions.map(|d| (d[0], d[1], d[2]))
}

// Given a tuple specifying the dimensions of a package, returns the area of paper required to wrap it.
fn get_paper_area_required(dimensions: (i32, i32, i32)) -> i32 {
    // Calculate the areas of each unique side. Note that there are
    // two faces with each of the areas.
    let mut side_areas = vec![
        dimensions.0 * dimensions.1,
        dimensions.1 * dimensions.2,
        dimensions.0 * dimensions.2];
    
    // Sort the areas least to greatest, and then calculate the total surface area
    // plus again the area of the smallest side.
    side_areas.sort_unstable();
    side_areas.iter().fold(side_areas[0], |total, side| total + 2 * side)
}

// Given a tuple specifying the dimensions of a package, returns the length of ribbon required to wrap it.
fn get_ribbon_length_required(dimensions: (i32, i32, i32)) -> i32 {
    // Calculate the perimeters of each unique side.
    let side_perimeters = vec![
        (dimensions.0 + dimensions.1) * 2,
        (dimensions.1 + dimensions.2) * 2,
        (dimensions.0 + dimensions.2) * 2];

    // Find the smallest perimeter, and then add the volume of the entire package to
    // get the required amount of ribbon.
    let smallest_perimeter = side_perimeters.iter().min().unwrap();
    let volume = dimensions.0 * dimensions.1 * dimensions.2;
    smallest_perimeter + volume
}

fn main() {
    // Part 1: Print out the total area of wrapping paper required for all packages specified in the input.
    let total_paper_area = package_dimensions().fold(0, |total, dimensions| return total + get_paper_area_required(dimensions));
    println!("{}", total_paper_area);

    // Part 2: Print out the total length of ribbon required for all packages specified in the input.
    let total_ribbon_length = package_dimensions().fold(0, |total, dimensions| return total + get_ribbon_length_required(dimensions));
    println!("{}", total_ribbon_length);
}
