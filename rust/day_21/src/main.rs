use aoc::parse_lines;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use std::str::FromStr;

#[derive(Clone, Debug)]
struct Recipe {
    ingredients: HashSet<String>,
    allergens: HashSet<String>,
}

impl FromStr for Recipe {
    type Err = ();

    fn from_str(input: &str) -> Result<Recipe, Self::Err> {
        let regex = Regex::new(r"([\w\s]+) \(contains ([\w\s,]+)\)").unwrap();
        let cap = regex.captures_iter(input).next().unwrap();
        return Ok(Recipe {
            ingredients: cap[1].split(" ").map(|s| s.to_string()).collect(),
            allergens: cap[2].split(", ").map(|s| s.to_string()).collect(),
        });
    }
}

fn get_all_ingredients(recipes: &Vec<Recipe>) -> HashSet<String> {
    let mut ingredients = HashSet::new();
    for recipe in recipes {
        for ingredient in &recipe.ingredients {
            ingredients.insert(ingredient.clone());
        }
    }
    return ingredients;
}

fn map_allergens(recipes: &Vec<Recipe>) -> HashMap<String, String> {
    let mut possible_ingredients: HashMap<String, HashSet<String>> = HashMap::new();

    for recipe in recipes {
        for allergen in recipe.allergens.clone() {
            match possible_ingredients.clone().get(&allergen) {
                Some(s) => {
                    let new_ingredients = recipe.ingredients.intersection(s);
                    possible_ingredients
                        .insert(allergen, new_ingredients.map(|s| s.clone()).collect());
                }
                None => {
                    possible_ingredients.insert(allergen, recipe.ingredients.clone());
                }
            }
        }
    }

    let mut allergens_map: HashMap<String, String> = HashMap::new();
    let mut reserved: HashSet<String> = HashSet::new();
    loop {
        for (k, v) in &possible_ingredients {
            let candidates = v
                .clone()
                .difference(&reserved)
                .map(|s| s.clone())
                .collect::<HashSet<String>>();

            if candidates.len() == 1 {
                let ingredient = candidates.iter().next().unwrap().clone();
                allergens_map.insert(k.clone(), ingredient.clone());
                reserved.insert(ingredient);
            }
        }

        if allergens_map.len() == possible_ingredients.len() {
            break;
        }
    }

    return allergens_map;
}

fn part_a(recipes: &Vec<Recipe>) -> usize {
    // Get all ingredients.
    let ingredients = get_all_ingredients(recipes);

    // Get a map of {allergen: ingredient}.
    let allergens: HashMap<String, String> = map_allergens(recipes);

    // Filter the ingredients that have zero.
    let unsafe_ingredients: HashSet<String> = allergens.values().map(|s| s.clone()).collect();
    let safe_ingredients: HashSet<String> = ingredients
        .difference(&unsafe_ingredients)
        .map(|s| s.clone())
        .collect();

    // Count how often these ingredients occur.
    return recipes
        .iter()
        .map(|r| {
            r.ingredients
                .intersection(&safe_ingredients)
                .map(|s| s.clone())
                .collect::<HashSet<String>>()
                .len()
        })
        .sum();
}

fn part_b(recipes: &Vec<Recipe>) -> String {
    let allergens_map = map_allergens(recipes);
    let mut unsafe_ingredients: Vec<(&String, &String)> = Vec::from_iter(allergens_map.iter());
    unsafe_ingredients.sort();
    return unsafe_ingredients
        .iter()
        .map(|&t| t.1.clone())
        .collect::<Vec<String>>()
        .join(",");
}

fn main() {
    let inputs: Vec<Recipe> = parse_lines()
        .iter()
        .map(|l| Recipe::from_str(l).unwrap())
        .collect();
    println!("Loaded {} recipes", inputs.len());

    println!("A: {}", part_a(&inputs));
    println!("B: {}", part_b(&inputs));
}
