// Advent of Code 2015, Day 21: "RPG Simulator 20XX"
// https://adventofcode.com/2015/day/21

#[derive(Debug, Clone)]
struct Item {
    #[allow(dead_code)]
    name: std::borrow::Cow<'static, str>,
    cost: u32,
    damage: u32,
    armor: u32,
}

impl Item {
    pub const fn new(name: &'static str, cost: u32, damage: u32, armor: u32) -> Item {
        Item {
            name: std::borrow::Cow::Borrowed(name),
            cost,
            damage,
            armor,
        }
    }
}

const WEAPONS: [Item; 5] = [
    Item::new("Dagger", 8, 4, 0),
    Item::new("Shortsword", 10, 5, 0),
    Item::new("Warhammer", 25, 6, 0),
    Item::new("Longsword", 40, 7, 0),
    Item::new("Greataxe", 74, 8, 0),
];

const ARMOR: [Item; 5] = [
    Item::new("Leather", 13, 0, 1),
    Item::new("Chainmail", 31, 0, 2),
    Item::new("Splintmail", 53, 0, 3),
    Item::new("Bandedmail", 75, 0, 4),
    Item::new("Platemail", 102, 0, 5),
];

const RINGS: [Item; 6] = [
    Item::new("Damage +1", 25, 1, 0),
    Item::new("Damage +2", 50, 2, 0),
    Item::new("Damage +3", 100, 3, 0),
    Item::new("Defense +1", 20, 0, 1),
    Item::new("Defense +2", 40, 0, 2),
    Item::new("Defense +3", 80, 0, 3),
];

#[derive(Debug)]
struct Loadout {
    weapon: Item,
    armor: Option<Item>,
    ring_left: Option<Item>,
    ring_right: Option<Item>,
}

impl Loadout {
    pub fn new(weapon: Item, armor: Option<Item>, ring_left: Option<Item>, ring_right: Option<Item>) -> Loadout {
        Loadout {
            weapon,
            armor,
            ring_left,
            ring_right,
        }
    }

    pub fn cost(&self) -> u32 {
        let mut cost = 0;
        self.for_each_item(|item| cost += item.cost);
        cost
    }

    pub fn damage(&self) -> u32 {
        let mut damage = 0;
        self.for_each_item(|item| damage += item.damage);
        damage
    }

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

struct LoadoutGenerator {
    weapon_index: usize,
    armor_index: usize,
    ring_left_index: usize,
    ring_right_index: usize,
}

impl LoadoutGenerator {
    pub fn new() -> LoadoutGenerator {
        LoadoutGenerator {
            weapon_index: 0,
            armor_index: 0,
            ring_left_index: 0,
            ring_right_index: 0,
        }
    }
}

impl Iterator for LoadoutGenerator {
    type Item = Loadout;

    fn next(&mut self) -> Option<Self::Item> {
        if self.weapon_index >= WEAPONS.len() {
            return None;
        }

        // Generate the loadout.
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

        self.ring_right_index += 1;
        if self.ring_right_index > RINGS.len() {
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

#[derive(Debug)]
struct CombatantDescription {
    damage: u32,
    armor: u32,
    starting_hit_points: u32,
}

impl CombatantDescription {
    pub fn new(damage: u32, armor: u32, starting_hit_points: u32) -> CombatantDescription {
        CombatantDescription {
            damage,
            armor,
            starting_hit_points,
        }
    }
}

#[derive(Debug, PartialEq)]
enum CombatWinner {
    Player,
    Boss,
}

fn simulate_combat(
    player_description: &CombatantDescription,
    boss_description: &CombatantDescription,
    log_fn: Option<fn(&str)>,
) -> CombatWinner {
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

    if let Some(log_fn) = log_fn {
        log_fn("--------------------------------------------------");
        log_fn(&format!("Player: {player_description:?}"));
        log_fn(&format!("Boss: {boss_description:?}"));
    }

    let mut player_hit_points = player_description.starting_hit_points;
    let mut boss_hit_points = boss_description.starting_hit_points;
    loop {
        if simulate_attack(
            "Player",
            player_description,
            "Boss",
            boss_description,
            &mut boss_hit_points,
            log_fn,
        ) {
            return CombatWinner::Player;
        }

        if simulate_attack(
            "Boss",
            boss_description,
            "Player",
            player_description,
            &mut player_hit_points,
            log_fn,
        ) {
            return CombatWinner::Boss;
        }
    }
}

fn get_integer_from_line(input: &str) -> Result<u32, ()> {
    for word in input.split_whitespace() {
        if let Ok(value) = word.parse::<u32>() {
            return Ok(value);
        }
    }
    Err(())
}

fn load_boss_description(input: &str) -> CombatantDescription {
    let mut line_iter = input.lines();
    let boss_hit_points = get_integer_from_line(line_iter.next().unwrap()).unwrap();
    let boss_damage = get_integer_from_line(line_iter.next().unwrap()).unwrap();
    let boss_armor = get_integer_from_line(line_iter.next().unwrap()).unwrap();
    CombatantDescription::new(boss_damage, boss_armor, boss_hit_points)
}

fn solve(input: &str, log_fn: Option<fn(&str)>) -> (String, String) {
    let boss_description = load_boss_description(input);

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
            CombatWinner::Player => (std::cmp::min(min_cost, loadout.cost()), max_cost),
            CombatWinner::Boss => (min_cost, std::cmp::max(max_cost, loadout.cost())),
        },
    );
    (part1_result.to_string(), part2_result.to_string())
}

#[linkme::distributed_slice(crate::SOLUTIONS)]
static SOLUTION: crate::Solution = crate::Solution::new(2015, 21, solve);
