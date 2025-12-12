use crate::utils;

#[derive(Debug)]
pub struct Data {
    fresh_ingredients: Vec<(usize, usize)>,
    ingredients: Vec<usize>,
}

pub fn run() -> usize {
    let result = utils::challenge_file("09");

    let mut raw = result.split("\n\n");
    let fresh_ingredients = raw
        .next()
        .unwrap_or("")
        .split('\n')
        .map(|x| {
            let start_and_end = x.split('-').collect::<Vec<&str>>();
            return (
                start_and_end[0].parse::<usize>().unwrap(),
                start_and_end[1].parse::<usize>().unwrap(),
            );
        })
        .collect::<Vec<(usize, usize)>>();

    let ingredients = raw
        .next()
        .unwrap_or("")
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|x| {
            return x.parse::<usize>().unwrap();
        })
        .collect::<Vec<usize>>();

    let data = Data {
        fresh_ingredients,
        ingredients,
    };

    return find_fresh_ingredients_count(&data);
}
fn find_fresh_ingredients_count(data: &Data) -> usize {
    let mut total = 0;

    for ingredient in data.ingredients.iter() {
        let mut is_fresh = false;
        for id_ranges in data.fresh_ingredients.iter() {
            if ingredient <= &id_ranges.1 && ingredient >= &id_ranges.0 {
                is_fresh = true;
            }
        }

        if is_fresh {
            total += 1;
        }
    }

    total
}
