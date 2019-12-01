use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let mut total = 0;
    for line in stdin.lock().lines() {
	let line = line.unwrap();
	let mass = line.parse::<u32>().unwrap();
	total += summing_fuel(mass);
    }
    println!("{}", total);
}

fn summing_fuel(mut mass: u32) -> u32 {
    let mut total = 0;
    while mass > 0 {
	mass = fuel(mass);
	total += mass;
    }
    total
}

fn fuel(mass: u32) -> u32 {
    (mass / 3).saturating_sub(2)
}


#[test]
fn test_fuel() {
    assert_eq!(fuel(12), 2);
    assert_eq!(fuel(14), 2);
    assert_eq!(fuel(1969), 654);
    assert_eq!(fuel(100756), 33583);
}

#[test]
fn test_summing_fuel() {
    assert_eq!(summing_fuel(14), 2);
    assert_eq!(summing_fuel(1969), 966);
    assert_eq!(summing_fuel(100756), 50346);
}
