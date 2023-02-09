use std::error;

use crate::grpc::{dice, Dice};

type Generator = fn(i32) -> i32;

pub fn roll(dice: Dice, gen: Generator) -> Result<Dice, Box<dyn error::Error>> {
    let mut rolls = Vec::<i32>::new();

    for _ in 0..dice.count {
        let mut result = 0;

        loop {
            let roll = gen(dice.faces);
            result += roll;

            if !dice.explodes_ons.contains(&roll) {
                break;
            }
        }

        rolls.push(result)
    }

    rolls = match dice.optional_drop_n {
        Some(ref drop_n) => {
            rolls.sort();

            match drop_n {
                dice::OptionalDropN::DropHighest(n) => {
                    rolls.reverse();
                    drop(*n, rolls)
                }
                dice::OptionalDropN::DropLowest(n) => drop(*n, rolls),
            }
        }
        None => rolls,
    };

    Ok(Dice {
        faces: dice.faces().into(),
        count: dice.count,
        result: rolls.iter().sum(),
        optional_drop_n: dice.optional_drop_n,
        explodes_ons: dice.explodes_ons,
    })
}

fn drop(n: i32, v: Vec<i32>) -> Vec<i32> {
    v[n as usize..v.len()].to_vec()
}
