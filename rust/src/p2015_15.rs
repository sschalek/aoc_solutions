// Advent of Code 2015, Day 15: "Science for Hungry People"
// https://adventofcode.com/2015/day/15

#[derive(Clone, Debug)]
struct IngredientDescription {
    #[allow(dead_code)]
    name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

fn parse_ingredient_description(line: &str) -> IngredientDescription {
    let words = line.split(&[' ', ',']).collect::<Vec<_>>();
    let integers = words.iter().filter_map(|w| w.parse::<i32>().ok()).collect::<Vec<_>>();
    IngredientDescription {
        name: words[0].to_string(),
        capacity: integers[0],
        durability: integers[1],
        flavor: integers[2],
        texture: integers[3],
        calories: integers[4],
    }
}

fn parse_ingredient_descriptions(input: &str) -> Vec<IngredientDescription> {
    input.lines().map(parse_ingredient_description).collect()
}

// Represents a cookie recipe.
#[derive(Clone, Debug)]
struct CookieRecipe {
    ingredient_usage: Vec<(IngredientDescription, u32)>,
}

impl CookieRecipe {
    // Creates a new CookieRecipe with the given ingredient usage.
    pub fn new(ingredient_usage: Vec<(IngredientDescription, u32)>) -> CookieRecipe {
        CookieRecipe { ingredient_usage }
    }

    // Returns the total score of the recipe.
    pub fn score(&self) -> i32 {
        std::cmp::max(self.sum_property_impact(|i| i.capacity), 0)
            * std::cmp::max(self.sum_property_impact(|i| i.durability), 0)
            * std::cmp::max(self.sum_property_impact(|i| i.flavor), 0)
            * std::cmp::max(self.sum_property_impact(|i| i.texture), 0)
    }

    // Returns the total number of calories in the recipe.
    pub fn calories(&self) -> i32 {
        self.sum_property_impact(|i| i.calories)
    }

    // Helper function that sums the impact on the specified property across all ingredient usages.
    fn sum_property_impact(&self, field: fn(&IngredientDescription) -> i32) -> i32 {
        self.ingredient_usage
            .iter()
            .map(|(ingredient, usage)| field(ingredient) * (*usage as i32))
            .sum()
    }
}

// Represents a cookie recipe generator that will generate all possible cookie recipes
// using exactly the given number of teaspoons of the given ingredients.
#[derive(Clone, Debug)]
struct CookieRecipeGenerator {
    ingredients: Vec<IngredientDescription>,
    ingredient_amounts: Vec<u32>,
    required_total_teaspoons: u32,
}

impl CookieRecipeGenerator {
    // Creates a new CookieRecipeGenerator with the given ingredients and total number of teaspoons.
    fn new(ingredients: Vec<IngredientDescription>, total_teaspoons: u32) -> CookieRecipeGenerator {
        let ingredient_amounts = vec![0; ingredients.len()];
        CookieRecipeGenerator {
            ingredients,
            ingredient_amounts,
            required_total_teaspoons: total_teaspoons,
        }
    }
}

impl Iterator for CookieRecipeGenerator {
    type Item = CookieRecipe;

    fn next(&mut self) -> Option<Self::Item> {
        // Increment the ingredient amounts to the next valid recipe.
        let mut index = 1;
        loop {
            self.ingredient_amounts[index] += 1;
            if self.ingredient_amounts[1..].iter().sum::<u32>() <= self.required_total_teaspoons {
                break;
            }

            self.ingredient_amounts[index] = 0;
            index += 1;
            if index >= self.ingredient_amounts.len() {
                return None;
            }
        }
        let partial_total_ingredients_amount = self.ingredient_amounts[1..].iter().sum::<u32>();
        self.ingredient_amounts[0] = self.required_total_teaspoons - partial_total_ingredients_amount;

        // Create a new recipe from the current ingredient amounts.
        let ingredient_usage = self
            .ingredients
            .iter()
            .zip(self.ingredient_amounts.iter())
            .map(|(ingredient, usage)| (ingredient.clone(), *usage))
            .collect();
        Some(CookieRecipe::new(ingredient_usage))
    }
}

fn solve(input: &str, _log_fn: Option<fn(&str)>) -> (String, String) {
    let ingredients = parse_ingredient_descriptions(input);

    // Part 1: Find the highest scoring cookie that can be made with the given ingredients.
    // Part 2: Find the highest scoring cookie that can be made with the given ingredients, but
    // with 500 calories.
    let generator = CookieRecipeGenerator::new(ingredients.clone(), 100);
    let (part1_result, part2_result) = generator.fold((0, 0), |(part1, part2), recipe| {
        let score = recipe.score();
        let calories = recipe.calories();
        (
            std::cmp::max(part1, score),
            if calories == 500 {
                std::cmp::max(part2, score)
            } else {
                part2
            },
        )
    });

    (part1_result.to_string(), part2_result.to_string())
}

#[linkme::distributed_slice(crate::SOLUTIONS)]
static SOLUTION: crate::Solution = crate::Solution::new(2015, 15, solve);
