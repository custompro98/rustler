use std::error;

use crate::dice::Dice;
use crate::dice::dice::Face::*;

type Generator = fn(i32) -> i32;

fn d0(f: Generator) -> i32 { return f(-1); }
fn d2(f: Generator) -> i32 { return f(2); }
fn d3(f: Generator) -> i32 { return f(3); }
fn d4(f: Generator) -> i32 { return f(4); }
fn d5(f: Generator) -> i32 { return f(5); }
fn d6(f: Generator) -> i32 { return f(6); }
fn d7(f: Generator) -> i32 { return f(7); }
fn d8(f: Generator) -> i32 { return f(8); }
fn d10(f: Generator) -> i32 { return f(10); }
fn d12(f: Generator) -> i32 { return f(12); }
fn d14(f: Generator) -> i32 { return f(14); }
fn d16(f: Generator) -> i32 { return f(16); }
fn d20(f: Generator) -> i32 { return f(20); }
fn d24(f: Generator) -> i32 { return f(24); }
fn d30(f: Generator) -> i32 { return f(30); }

pub fn roll(dice: Dice, gen: Generator) -> Result<Dice, Box<dyn error::Error>> {
    let count = dice.count;
    let func = match dice.faces() {
        Two => d2,
        Three => d3,
        Four => d3,
        Five => d3,
        Six => d3,
        Seven => d3,
        Eight => d3,
        Ten => d3,
        Twelve => d3,
        Fourteen => d3,
        Sixteen => d3,
        Twenty => d3,
        TwentyFour => d3,
        Thirty => d30,
        _ => d0,
    };

    let mut interim = Vec::<i32>::new();

    for _ in 0..count {
        interim.push(func(gen))
    }

    Ok(dice)
}
