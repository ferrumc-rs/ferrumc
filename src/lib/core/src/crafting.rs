use std::collections::HashMap;
use ferrumc_data::items::Item;
use ferrumc_data::recipes::{Recipe, RecipeType};
use ferrumc_data::tags::TagData;

pub fn get_recipes_from_2x2(grid: [[Option<&Item>; 2]; 2]) -> Vec<&Recipe> {
    get_recipes_from_3x3([
        [grid[0][0], grid[0][1], None],
        [grid[1][0], grid[1][1], None],
        [None,       None,       None],
    ])
}

pub fn get_recipes_from_3x3(grid: [[Option<&Item>; 3]; 3]) -> Vec<&Recipe> {
    Recipe::ALL_RECIPES
        .iter()
        .filter(|recipe| recipe.is_crafting())
        .filter(|recipe| match recipe.recipe_type {
            RecipeType::CraftingShaped =>
                matches_crafting_shaped(recipe, grid).is_some(),
            RecipeType::CraftingShapeless =>
                matches_crafting_shapeless(recipe, grid).is_some(),
            _ => unreachable!(),
        })
        .map(|r| *r)
        .collect()
}

fn normalize_grid(grid: &mut Vec<Vec<Option<&str>>>) {
    if grid.is_empty() || grid[0].is_empty() {
        return;
    }

    let top = (0..grid.len())
        .find(|&r| grid[r].iter().any(|c| c.is_some()))
        .unwrap_or_default();

    let bottom = (0..grid.len())
        .rfind(|&r| grid[r].iter().any(|c| c.is_some()))
        .unwrap_or(grid.len() - 1);

    let left = (0..grid[0].len())
        .find(|&r| grid.iter().any(|row| row[r].is_some()))
        .unwrap_or_default();

    let right = (0..grid[0].len())
        .rfind(|&r| grid.iter().any(|row| row[r].is_some()))
        .unwrap_or(grid[0].len() - 1);

    let mut trimmed = Vec::with_capacity(bottom - top + 1);
    for r in top..=bottom {
        trimmed.push(grid[r][left..=right].to_vec());
    }

    *grid = trimmed;
}

fn matches_crafting_shaped(recipe: &Recipe, grid: [[Option<&Item>; 3]; 3]) -> Option<()> {
    let key = recipe.key.as_ref()?;

    let lookup = key.iter().cloned().collect::<HashMap<&str, &[_]>>();

    let mut cells = grid
        .iter()
        .map(|row| row.iter().map(|slot| slot.as_ref().map(|i| i.registry_key)).collect())
        .collect::<Vec<Vec<_>>>();

    normalize_grid(&mut cells);

    let pattern = recipe.pattern?;
    if cells.len() != pattern.len() { return None; }
    if cells[0].len() != pattern[0][0].len() { return None; }

    for (r, row) in pattern.iter().enumerate() {
        for (c, symbol) in row[0].chars().enumerate() {
            match cells[r][c] {
                None => if symbol != ' ' { return None; }
                Some(name) => {
                    let allowed = lookup.get(format!("{symbol}").as_str())?;
                    if !symbol_matches_item(allowed, name) {
                        return None;
                    }
                }
            }
        }
    }

    Some(())
}

fn matches_crafting_shapeless(recipe: &Recipe, grid: [[Option<&Item>; 3]; 3]) -> Option<()> {
    let ingredients = recipe.ingredients.as_ref()?;

    let items = grid
        .iter()
        .flat_map(|row| row.iter())
        .filter_map(|slot| slot.as_ref().map(|i| i.registry_key))
        .collect::<Vec<_>>();

    if items.len() != ingredients.len() {
        return None;
    }

    let mut used = vec![false; items.len()];

    'outer: for ingredient in *ingredients {
        for (i, item) in items.iter().enumerate() {
            if used[i] { continue }

            if symbol_matches_item(std::slice::from_ref(ingredient), item) {
                used[i] = true;
                continue 'outer;
            }
        }

        return None;
    }

    Some(())
}

fn symbol_matches_item(symbol_allowed: &[&str], item_id: &str) -> bool {
    for allowed in symbol_allowed {
        if let Some(tag) = allowed.strip_prefix('#') {
            if let Some(values)= TagData::get_item_tag(tag) {
                if values.values.iter().any(|&v| v == item_id) {
                    return true;
                }
            }
        } else if *allowed == item_id {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_none() {
        let recipes = get_recipes_from_3x3([
            [Some(&Item::OAK_PLANKS),   Some(&Item::OAK_PLANKS),    None                    ],
            [None,                      Some(&Item::STICK),         Some(&Item::OAK_PLANKS) ],
            [None,                      Some(&Item::ANVIL),         None                    ],
        ]);

        assert_eq!(recipes.len(), 0);
    }

    #[test]
    fn test_find_wooden_pickaxe() {
        let recipes = get_recipes_from_3x3([
            [Some(&Item::OAK_PLANKS),   Some(&Item::OAK_PLANKS),    Some(&Item::OAK_PLANKS) ],
            [None,                      Some(&Item::STICK),         None                    ],
            [None,                      Some(&Item::STICK),         None                    ],
        ]);

        assert_eq!(recipes, [&Recipe::RECIPE_1387]);
    }

    #[test]
    fn test_find_oak_button() {
        let recipes1 = get_recipes_from_3x3([
            [Some(&Item::OAK_PLANKS),   None,   None ],
            [None,                      None,   None ],
            [None,                      None,   None ],
        ]);

        let recipes2 = get_recipes_from_3x3([
            [None,   None,   None ],
            [None,   Some(&Item::OAK_PLANKS),   None ],
            [None,   None,   None ],
        ]);

        let recipes3 = get_recipes_from_3x3([
            [None,   None,   None ],
            [None,   None,   None ],
            [Some(&Item::OAK_PLANKS),   None,   None ],
        ]);

        let recipes4 = get_recipes_from_3x3([
            [None,   None,   None ],
            [None,   None,   Some(&Item::OAK_PLANKS) ],
            [None,   None,   None ],
        ]);

        assert_eq!(recipes1, recipes2);
        assert_eq!(recipes2, recipes3);
        assert_eq!(recipes3, recipes4);
        assert_eq!(recipes4, [&Recipe::RECIPE_807])
    }

    #[test]
    fn test_oak_planks() {
        let recipes = get_recipes_from_2x2([
            [Some(&Item::OAK_LOG), None],
            [None, None],
        ]);

        assert_eq!(recipes.len(), 1);
        assert_eq!(recipes, [&Recipe::RECIPE_813]);
    }

    #[test]
    fn test_sticks() {
        let recipes = get_recipes_from_2x2([
            [Some(&Item::OAK_PLANKS), None],
            [Some(&Item::OAK_PLANKS), None],
        ]);

        assert_eq!(recipes, [&Recipe::RECIPE_1150]);
    }
}