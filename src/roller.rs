use std::error;

use crate::grpc::dice::{Face::*, self};
use crate::grpc::Dice;

type Generator = fn(i32) -> i32;

fn d0(f: Generator) -> i32 {
    return f(-1);
}
fn d2(f: Generator) -> i32 {
    return f(2);
}
fn d3(f: Generator) -> i32 {
    return f(3);
}
fn d4(f: Generator) -> i32 {
    return f(4);
}
fn d5(f: Generator) -> i32 {
    return f(5);
}
fn d6(f: Generator) -> i32 {
    return f(6);
}
fn d7(f: Generator) -> i32 {
    return f(7);
}
fn d8(f: Generator) -> i32 {
    return f(8);
}
fn d10(f: Generator) -> i32 {
    return f(10);
}
fn d12(f: Generator) -> i32 {
    return f(12);
}
fn d14(f: Generator) -> i32 {
    return f(14);
}
fn d16(f: Generator) -> i32 {
    return f(16);
}
fn d20(f: Generator) -> i32 {
    return f(20);
}
fn d24(f: Generator) -> i32 {
    return f(24);
}
fn d30(f: Generator) -> i32 {
    return f(30);
}

pub fn roll(dice: Dice, gen: Generator) -> Result<Dice, Box<dyn error::Error>> {
    let func = match dice.faces() {
        Two => d2,
        Three => d3,
        Four => d4,
        Five => d5,
        Six => d6,
        Seven => d7,
        Eight => d8,
        Ten => d10,
        Twelve => d12,
        Fourteen => d14,
        Sixteen => d16,
        Twenty => d20,
        TwentyFour => d24,
        Thirty => d30,
        _ => d0,
    };

    let mut interim = Vec::<i32>::new();

    for _ in 0..(dice.count) {
        let mut res = 0;

        loop {
            let roll = func(gen);
            res += roll;

            if roll != dice.explodes_on {
                break;
            }
        }

        interim.push(res)
    }


    match dice.drop_n {
        Some(dice::DropN::DropLowest(n)) => {
            if n <= interim.len() as i32 {
                interim.sort();
                interim = interim[n as usize..interim.len()].to_vec();
            }
        },
        Some(dice::DropN::DropHighest(n)) => {
            if n <= interim.len() as i32 {
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
        drop_n: dice.drop_n,
        explodes_on: dice.explodes_on,
    })
}
