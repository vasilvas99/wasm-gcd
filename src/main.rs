use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Cli {
    first_number: u64,
    second_number: u64,
}

fn gcd(a: u64, b: u64) -> u64 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let remainder = a % b;
        a = b;
        b = remainder;
    }
    a
}

fn main() {
    let cli = Cli::parse();
    println!(
        "The greatest common divisor of {} and {} is: {}",
        cli.first_number,
        cli.second_number,
        gcd(cli.first_number, cli.second_number)
    )
}
