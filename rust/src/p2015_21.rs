// Advent of Code 2015, Day 21: "RPG Simulator 20XX"
// https://adventofcode.com/2015/day/21

// Represents an item that can be equipped by the player.
#[derive(Debug, Clone)]
struct Item {
    #[allow(dead_code)]
    name: std::borrow::Cow<'static, str>,
    cost: u32,
    damage: u32,
    armor: u32,
}

impl Item {
    // Creates a new instance with the given properties.
    pub const fn new(name: &'static str, cost: u32, damage: u32, armor: u32) -> Item {
        Item {
            name: std::borrow::Cow::Borrowed(name),
            cost,
            damage,
            armor,
        }
    }
}

// The set of weapons that can be purchased by the player.
const WEAPONS: [Item; 5] = [
    Item::new("Dagger", 8, 4, 0),
    Item::new("Shortsword", 10, 5, 0),
    Item::new("Warhammer", 25, 6, 0),
    Item::new("Longsword", 40, 7, 0),
    Item::new("Greataxe", 74, 8, 0),
];

// The set of armors that can be purchased by the player.
const ARMOR: [Item; 5] = [
    Item::new("Leather", 13, 0, 1),
    Item::new("Chainmail", 31, 0, 2),
    Item::new("Splintmail", 53, 0, 3),
    Item::new("Bandedmail", 75, 0, 4),
    Item::new("Platemail", 102, 0, 5),
];

// The set of rings that can be purchased by the player.
const RINGS: [Item; 6] = [
    Item::new("Damage +1", 25, 1, 0),
    Item::new("Damage +2", 50, 2, 0),
    Item::new("Damage +3", 100, 3, 0),
    Item::new("Defense +1", 20, 0, 1),
    Item::new("Defense +2", 40, 0, 2),
    Item::new("Defense +3", 80, 0, 3),
];

// Represents a specific loadout of items equipped by the player.
#[derive(Debug)]
struct Loadout {
    weapon: Item,
    armor: Option<Item>,
    ring_left: Option<Item>,
    ring_right: Option<Item>,
}

impl Loadout {
    // Creates a new instance with the given items equipped.
    pub fn new(weapon: Item, armor: Option<Item>, ring_left: Option<Item>, ring_right: Option<Item>) -> Loadout {
        Loadout {
            weapon,
            armor,
            ring_left,
            ring_right,
        }
    }

    // Returns the total cost of the items in the loadout.
    pub fn cost(&self) -> u32 {
        let mut cost = 0;
        self.for_each_item(|item| cost += item.cost);
        cost
    }

    // Returns the total damage dealt by the items in the loadout.
    pub fn damage(&self) -> u32 {
        let mut damage = 0;
        self.for_each_item(|item| damage += item.damage);
        damage
    }

    // Returns the total armor provided by the items in the loadout.
    pub fn armor(&self) -> u32 {
        let mut armor = 0;
        self.for_each_item(|item| armor += item.armor);
        armor
    }

    // Run the given function for each item in the loadout.
    pub fn for_each_item<F>(&self, mut f: F)
    where
        F: FnMut(&Item),
    {
        f(&self.weapon);
        if let Some(armor) = &self.armor {
            f(armor);
        }
        if let Some(ring_left) = &self.ring_left {
            f(ring_left);
        }
        if let Some(ring_right) = &self.ring_right {
            f(ring_right);
        }
    }
}

// An iterator that generates all possible loadouts of items that can be equipped by the player.
struct LoadoutGenerator {
    weapon_index: usize,
    armor_index: usize,
    ring_left_index: usize,
    ring_right_index: usize,
}

impl LoadoutGenerator {
    // Creates a new instance.
    pub fn new() -> LoadoutGenerator {
        LoadoutGenerator {
            weapon_index: 0,
            armor_index: 0,
            ring_left_index: 0,
            ring_right_index: 0,
        }
    }
}

// Implement the Iterator trait for the LoadoutGenerator struct, so that it can be used as an iterator.
impl Iterator for LoadoutGenerator {
    type Item = Loadout;

    // Returns the next loadout in the sequence.
    fn next(&mut self) -> Option<Self::Item> {
        // If the weapon index is out of bounds, then there are no more loadouts to generate.
        if self.weapon_index >= WEAPONS.len() {
            return None;
        }

        // Generate the item loadout based on the current item indices.
        let weapon = WEAPONS[self.weapon_index].clone();
        let armor = if self.armor_index == ARMOR.len() {
            None
        } else {
            Some(ARMOR[self.armor_index].clone())
        };
        let ring_left = if self.ring_left_index == RINGS.len() {
            None
        } else {
            Some(RINGS[self.ring_left_index].clone())
        };
        let ring_right = if self.ring_right_index == RINGS.len() {
            None
        } else {
            Some(RINGS[self.ring_right_index].clone())
        };

        // Update the item indices for the next iteration. The item indices are updated in order,
        // and each time an index is incremented past the end of its range, it is reset to 0 and
        // the next index is incremented.
        self.ring_right_index += 1;
        if self.ring_right_index > RINGS.len() {
            // If the right ring index is out of bounds, then reset it. The right ring index is
            // reset to the left ring index so that duplicate ring pairs are not generated.
            self.ring_right_index = self.ring_left_index;
            self.ring_left_index += 1;
        }

        if self.ring_left_index > RINGS.len() {
            self.ring_left_index = 0;
            self.armor_index += 1;
        }

        if self.armor_index > ARMOR.len() {
            self.armor_index = 0;
            self.weapon_index += 1;
        }

        Some(Loadout::new(weapon, armor, ring_left, ring_right))
    }
}

// Defines the properties of a combatant.
#[derive(Debug)]
struct CombatantDescription {
    damage: u32,
    armor: u32,
    starting_hit_points: u32,
}

impl CombatantDescription {
    // Creates a new instance with the given properties.
    pub fn new(damage: u32, armor: u32, starting_hit_points: u32) -> CombatantDescription {
        CombatantDescription {
            damage,
            armor,
            starting_hit_points,
        }
    }
}

// Defines the possible combatant types.
#[derive(Debug, PartialEq)]
enum CombatantType {
    Player,
    Boss,
}

// Simulates a combat between the given combatants and returns the type of the winning combatant.
fn simulate_combat(
    player_description: &CombatantDescription,
    boss_description: &CombatantDescription,
    log_fn: Option<fn(&str)>,
) -> CombatantType {
    // Simulates an attack by the given attacker against the given defender.
    // Returns true if the defender has been defeated.
    fn simulate_attack(
        attacker_name: &str,
        attacker_description: &CombatantDescription,
        defender_name: &str,
        defender_description: &CombatantDescription,
        defender_hit_points: &mut u32,
        log_fn: Option<fn(&str)>,
    ) -> bool {
        // Calculate the damage dealt by the attacker and update the defender's hit points.
        let attacker_damage = std::cmp::max(
            attacker_description.damage.saturating_sub(defender_description.armor),
            1,
        );
        *defender_hit_points = defender_hit_points.saturating_sub(attacker_damage);

        // Log the attack if a log function was provided.
        if let Some(log_fn) = log_fn {
            log_fn(&format!(
                "{attacker_name} hits for {attacker_damage}. {defender_name} now has {defender_hit_points} hit points."
            ));
        }

        // Check whether the attacker has won.
        if *defender_hit_points == 0 {
            if let Some(log_fn) = log_fn {
                log_fn(&format!("== {attacker_name} wins! =="));
            }
            true
        } else {
            false
        }
    }

    // Log the combatants if a log function was provided.
    if let Some(log_fn) = log_fn {
        log_fn("--------------------------------------------------");
        log_fn(&format!("Player: {player_description:?}"));
        log_fn(&format!("Boss: {boss_description:?}"));
    }

    // Simulate the combat until one of the combatants has been defeated.
    let mut player_hit_points = player_description.starting_hit_points;
    let mut boss_hit_points = boss_description.starting_hit_points;
    loop {
        // Simulate an attack by the player against the boss. The player always attacks first.
        if simulate_attack(
            "Player",
            player_description,
            "Boss",
            boss_description,
            &mut boss_hit_points,
            log_fn,
        ) {
            return CombatantType::Player;
        }

        // Simulate an attack by the boss against the player.
        if simulate_attack(
            "Boss",
            boss_description,
            "Player",
            player_description,
            &mut player_hit_points,
            log_fn,
        ) {
            return CombatantType::Boss;
        }
    }
}

// Returns the first integer value successfully parsed from the given string.
fn get_integer_from_line(input: &str) -> Result<u32, ()> {
    for word in input.split_whitespace() {
        if let Ok(value) = word.parse::<u32>() {
            return Ok(value);
        }
    }
    Err(())
}

// Parses the given input string into a CombatantDescription for the boss.
fn load_boss_description(input: &str) -> CombatantDescription {
    let mut line_iter = input.lines();
    let boss_hit_points = get_integer_from_line(line_iter.next().unwrap()).unwrap();
    let boss_damage = get_integer_from_line(line_iter.next().unwrap()).unwrap();
    let boss_armor = get_integer_from_line(line_iter.next().unwrap()).unwrap();
    CombatantDescription::new(boss_damage, boss_armor, boss_hit_points)
}

fn solve(input: &str, log_fn: Option<fn(&str)>) -> (String, String) {
    let boss_description = load_boss_description(input);

    // Generate all possible loadouts of items that can be equipped by the player.
    let loadout_generator = LoadoutGenerator::new();
    let combat_results = loadout_generator.map(|loadout| {
        let player_description = CombatantDescription::new(loadout.damage(), loadout.armor(), 100);
        let winner = simulate_combat(&player_description, &boss_description, log_fn);
        (loadout, winner)
    });

    // Part 1: Find the minimum cost of a loadout that can beat the boss.
    // Part 2: Find the maximum cost of a loadout that can lose to the boss.
    let (part1_result, part2_result) = combat_results.fold(
        (u32::MAX, u32::MIN),
        |(min_cost, max_cost), (loadout, winner)| match winner {
            CombatantType::Player => (std::cmp::min(min_cost, loadout.cost()), max_cost),
            CombatantType::Boss => (min_cost, std::cmp::max(max_cost, loadout.cost())),
        },
    );
    (part1_result.to_string(), part2_result.to_string())
}

#[linkme::distributed_slice(crate::SOLUTIONS)]
static SOLUTION: crate::Solution = crate::Solution::new(2015, 21, solve);
