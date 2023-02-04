use std::io::BufRead;

mod dice;
mod generator;

fn main() {
    let stdin = std::io::stdin();

    loop {
        let mut pattern = String::new();
        println!("\nWhat should we roll?");

        let mut handle = stdin.lock();
        let result = handle.read_line(&mut pattern);

        match result {
            Err(e) => println!("Input is invalid: {}", e),
            Ok(_) => match pattern.trim() {
                "2" => println!("{}", dice::d2(generator::rand)),
                "3" => println!("{}", dice::d3(generator::rand)),
                "4" => println!("{}", dice::d4(generator::rand)),
                "5" => println!("{}", dice::d5(generator::rand)),
                "6" => println!("{}", dice::d6(generator::rand)),
                "7" => println!("{}", dice::d7(generator::rand)),
                "8" => println!("{}", dice::d8(generator::rand)),
                "10" => println!("{}", dice::d10(generator::rand)),
                "12" => println!("{}", dice::d12(generator::rand)),
                "14" => println!("{}", dice::d14(generator::rand)),
                "16" => println!("{}", dice::d16(generator::rand)),
                "20" => println!("{}", dice::d20(generator::rand)),
                "24" => println!("{}", dice::d24(generator::rand)),
                "30" => println!("{}", dice::d30(generator::rand)),
                _ => println!("Invalid input"),
            },
        }
    }
}
