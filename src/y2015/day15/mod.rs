use itertools::Itertools;
use std::collections::HashMap;

use crate::read_input;

struct Ingredient {
    name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

fn read_ingredients() -> Vec<Ingredient> {
    read_input(2015, 15)
        .unwrap()
        .map_while(Result::ok)
        .map(|line| {
            // Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
            let (name, other) = line.split_once(": ").unwrap();

            let map: HashMap<String, i32> = other
                .split(", ")
                .map(|v| v.split_once(" ").unwrap())
                .map(|(key, value)| (key.to_string(), value.parse::<i32>().unwrap()))
                .collect();

            Ingredient {
                name: name.to_string(),
                capacity: *map.get("capacity").unwrap(),
                durability: *map.get("durability").unwrap(),
                flavor: *map.get("flavor").unwrap(),
                texture: *map.get("texture").unwrap(),
                calories: *map.get("calories").unwrap(),
            }
        })
        .collect()
}

fn generate_permutations(
    n: usize,
    remaining: i32,
    index: usize,
    current: &mut Vec<i32>,
    results: &mut Vec<Vec<i32>>,
) {
    if index == n - 1 {
        current[index] = remaining;
        results.push(current.clone());
        return;
    }

    for value in 0..=remaining {
        current[index] = value;
        generate_permutations(n, remaining - value, index + 1, current, results);
    }
}

const TEASPOONS: i32 = 100;

fn run(target_calories: Option<i32>) -> i32 {
    let ingredients = read_ingredients();

    let mut permutations = Vec::new();
    generate_permutations(
        ingredients.len(),
        TEASPOONS,
        0,
        &mut vec![0; ingredients.len()],
        &mut permutations,
    );

    let mut tts = permutations.iter().map(|p| {
        let tt_capacity = 0.max(
            ingredients
                .iter()
                .enumerate()
                .map(|(i, ingredient)| ingredient.capacity * p[i])
                .sum(),
        );
        let tt_durability = 0.max(
            ingredients
                .iter()
                .enumerate()
                .map(|(i, ingredient)| ingredient.durability * p[i])
                .sum(),
        );
        let tt_flavor = 0.max(
            ingredients
                .iter()
                .enumerate()
                .map(|(i, ingredient)| ingredient.flavor * p[i])
                .sum(),
        );
        let tt_texture = 0.max(
            ingredients
                .iter()
                .enumerate()
                .map(|(i, ingredient)| ingredient.texture * p[i])
                .sum(),
        );

        let tt_calories = 0.max(
            ingredients
                .iter()
                .enumerate()
                .map(|(i, ingredient)| ingredient.calories * p[i])
                .sum(),
        );

        (
            tt_capacity * tt_durability * tt_flavor * tt_texture,
            tt_calories,
        )
    });

    let tts: Vec<(i32, i32)> = match target_calories {
        Some(target_calories) => tts
            .filter(|(_, calories)| *calories == target_calories)
            .collect(),
        None => tts.collect(),
    };

    tts.into_iter().map(|(tt, _)| tt).max().unwrap()
}

pub fn run_part_1() {
    let result = run(None);
    assert_eq!(result, 18965440);
}

pub fn run_part_2() {
    let result = run(Some(500));
    assert_eq!(result, 15862900);
}
