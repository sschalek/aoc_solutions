use std::io::BufRead;

fn package_dimensions() -> impl Iterator<Item=(i32, i32, i32)> {
    let input_file = std::fs::File::open("input.txt").expect("A file named \"input.txt\" with the problem data must be present in the current directory.");

    // Create a line-based iterator for the file contents.
    let reader = std::io::BufReader::new(input_file);
    let lines = reader.lines().map(|l| l.unwrap());

    // Create an iterator that returns the dimensions of each package in a tuple from the line-based iterator.
    let dimensions = lines.map(|l| l.split("x").map(|d| d.parse::<i32>().unwrap()).collect::<Vec<_>>());
    return dimensions.map(|d| (d[0], d[1], d[2]));
}

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
    return side_areas.iter().fold(side_areas[0], |total, side| total + 2 * side);
}

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
    return smallest_perimeter + volume;
}

fn main() {
    let total_paper_area = package_dimensions().fold(0, |total, dimensions| return total + get_paper_area_required(dimensions));
    println!("{}", total_paper_area);
    let total_ribbon_length = package_dimensions().fold(0, |total, dimensions| return total + get_ribbon_length_required(dimensions));
    println!("{}", total_ribbon_length);
}