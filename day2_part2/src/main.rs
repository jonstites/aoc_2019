use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut stdin = std::io::stdin();
    let mut inputs = String::new();
    stdin.read_to_string(&mut inputs)?;

    let inputs = parse(&inputs).expect("bad input");

    for noun in 0..=99 {
	for verb in 0..=99 {
	    let mut inputs = inputs.clone();
	    inputs[1] = noun;
	    inputs[2] = verb;
	    let outputs = ops(inputs);
	    if outputs[0] == 19690720 {
		println!("{} {}, {}", noun, verb, 100 * noun + verb);
	    }
	}
    }
    
    Ok(())
}


fn parse(inputs: &str) -> Result<Vec<i32>, <i32 as std::str::FromStr>::Err> {
    inputs.trim()
	.split(',')
	.into_iter()
	.map(|s| s.parse::<i32>())
	.collect()
}

fn ops(mut inputs: Vec<i32>) -> Vec<i32> {
    let mut current_index = 0;
    while inputs[current_index] != 99 {
	let op = inputs[current_index];
	let left = inputs[inputs[current_index + 1] as usize];
	let right = inputs[inputs[current_index + 2] as usize];
	let target_idx = inputs[current_index + 3] as usize;
	if op == 1 {
	    inputs[target_idx] = left + right;
	} else if op == 2 {
	    inputs[target_idx] = left * right;
	} else {
	    panic!(format!("bad opcode: {}", op));
	}

	current_index += 4;
    }
    inputs
}

#[test]
fn parse_ok() {
    let s = "3,3,4,10,11\n";
    assert_eq!(parse(s), Ok(vec![3,3,4,10,11]));
}

#[test]
fn parse_failure() {
    let s = "2,abc,3";
    assert!(parse(s).is_err());
}

#[test]
fn ops_correct() {
    assert_eq!(ops(vec![1,0,0,0,99]), vec![2,0,0,0,99]);
    assert_eq!(ops(vec![2,3,0,3,99]), vec![2,3,0,6,99]);
    assert_eq!(ops(vec![2,4,4,5,99,0]), vec![2,4,4,5,99,9801]);
    assert_eq!(ops(vec![1,1,1,4,99,5,6,0,99]), vec![30,1,1,4,2,5,6,0,99]);
}
