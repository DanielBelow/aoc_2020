use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub struct IngredientList {
    ingredients: Vec<String>,
    allergens: Vec<String>,
}

#[aoc_generator(day21)]
pub fn generate(inp: &str) -> Vec<IngredientList> {
    inp.lines().fold(Vec::new(), |mut acc, it| {
        if let Some(allergens_start) = it.find('(') {
            let ingredients = it[..allergens_start - 1]
                .split(' ')
                .map(String::from)
                .collect_vec();

            if let Some(allergens) = it[allergens_start..it.len() - 1]
                .strip_prefix("(contains ")
                .map(|it| it.split(", ").map(String::from).collect_vec())
            {
                let ing_list = IngredientList {
                    ingredients,
                    allergens,
                };
                acc.push(ing_list);
            }
        }

        acc
    })
}

fn get_allergen_map(data: &[IngredientList]) -> HashMap<String, HashSet<String>> {
    data.iter().fold(HashMap::new(), |mut acc, it| {
        let ingredients = it.ingredients.iter().cloned().collect::<HashSet<_>>();
        for all in it.allergens.iter() {
            acc.entry(all.to_string())
                .and_modify(|it| *it = it.intersection(&ingredients).cloned().collect())
                .or_insert_with(|| ingredients.clone());
        }
        acc
    })
}

#[aoc(day21, part1)]
pub fn part1(data: &[IngredientList]) -> usize {
    let allergen_map = get_allergen_map(data);

    let safe_ingredients = data
        .iter()
        .flat_map(|it| it.ingredients.clone())
        .filter(|it| allergen_map.values().find(|f| f.contains(it)).is_none())
        .collect::<HashSet<_>>();

    data.iter().fold(0, |acc, it| {
        acc + it
            .ingredients
            .iter()
            .filter(|i| safe_ingredients.contains(*i))
            .count()
    })
}

fn find_single_ingredients(
    all_map: &mut HashMap<String, HashSet<String>>,
) -> Vec<(String, String)> {
    let mut res = Vec::new();

    while !all_map.is_empty() {
        all_map
            .iter()
            .filter_map(|(k, v)| {
                if let Ok(v) = v.iter().exactly_one() {
                    Some((k, v))
                } else {
                    None
                }
            })
            .for_each(|(k, v)| res.push((k.to_owned(), v.to_owned())));

        res.iter().for_each(|(k, _)| {
            all_map.remove(k);
        });

        all_map.values_mut().for_each(|it| {
            let ings = res.iter().map(|(_, v)| v).cloned().collect();
            *it = it.difference(&ings).cloned().collect();
        })
    }

    res
}

fn join_ingredients_by_allergen(all_ing_pairs: Vec<(String, String)>) -> String {
    all_ing_pairs
        .iter()
        .sorted_by_key(|(k, _)| k)
        .map(|(_, v)| v)
        .join(",")
}

#[aoc(day21, part2)]
pub fn part2(data: &[IngredientList]) -> String {
    let mut allergen_map = get_allergen_map(data);
    let allergen_ingredient_pairs = find_single_ingredients(&mut allergen_map);
    join_ingredients_by_allergen(allergen_ingredient_pairs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let inp = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";

        let data = generate(inp);
        assert_eq!(5, part1(&data));
    }

    #[test]
    fn test_part2() {
        let inp = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";

        let data = generate(inp);
        assert_eq!("mxmxvkd,sqjhc,fvjkl", part2(&data));
    }
}
