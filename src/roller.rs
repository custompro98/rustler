use std::error;

use crate::grpc::{dice, Dice};

type Generator = fn(i32) -> i32;

pub fn roll(dice: Dice, gen: Generator) -> Result<Dice, Box<dyn error::Error>> {
    let mut interim = Vec::<i32>::new();

    for _ in 0..dice.count {
        let mut res = 0;

        loop {
            let roll = gen(dice.faces);
            res += roll;

            if !dice.explodes_ons.contains(&roll) {
                break;
            }
        }

        interim.push(res)
    }


    match dice.optional_drop_n {
        Some(dice::OptionalDropN::DropLowest(n)) => {
            if n <= interim.len() as i32 {
                println!("Dropping lowest {} from {:?}", n, interim);
                interim.sort();
                interim = interim[n as usize..interim.len()].to_vec();
            }
        },
        Some(dice::OptionalDropN::DropHighest(n)) => {
            if n <= interim.len() as i32 {
                println!("Dropping highest {} from {:?}", n, interim);
                interim.sort();
                interim = interim[0..interim.len() - n as usize].to_vec();
            }
        }
        None => {},
    }

    Ok(Dice {
        faces: dice.faces().into(),
        count: dice.count,
        result: interim.iter().sum(),
        optional_drop_n: dice.optional_drop_n,
        explodes_ons: dice.explodes_ons,
    })
}
