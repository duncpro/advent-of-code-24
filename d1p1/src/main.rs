use std::io::{BufReader, prelude::*};

// Each cell contains exactly five decimal digits (0-9).
// Therefore the maximum representable number is 10^5 - 1.
// We must choose a Rust integer datatype that can represent any number in the range (0..10^5).
// The question is, what is the minimum number of bits we need to encode an unsigned integer capable
// of representing 10^5?
//
// We know that an unsigned integer with n bits can represent any number in the range 0..=(2^n - 1).
// That is, 2^n distinct values. We need to represent 10^5 distinct values, and so we must solve the
// following equation for n: 2^n = 10^5. Obviously, n = log2(10^5). According to my calculator, this is approximately 16.6.
// Meaning, we need 16.6 bits to represent 10^5 distinct values. 16 bits alone is not sufficient, and there are no half-bits,
// so then 17 bits are the minimum necessary to represent a number in the range 0..10^5. 
/// Therfore, we choose the Rust data type u32.
type CellInt = u32;

fn main() -> Result<(), std::io::Error> {
    let mut file_reader = BufReader::new(std::fs::File::open("input.txt")?);
    let lines: Vec<String> = file_reader.lines().map(|res| res.unwrap()).collect();
    let mut left_col: Vec<u32> = Vec::with_capacity(lines.len());
    let mut right_col: Vec<u32> = Vec::with_capacity(lines.len());
    for line in &lines {
        let mut cells = line.split("   ");
        let left_cell = cells.next().unwrap().parse::<u32>().unwrap();
        let right_cell = cells.next().unwrap().parse::<u32>().unwrap();
        left_col.push(left_cell);
        right_col.push(right_cell);
    }

    left_col.sort_unstable();
    right_col.sort_unstable();
    let pairs = std::iter::zip(left_col.iter().copied(), right_col.iter().copied());

    let mut total_distance: u32 = 0;
    for (left_val, right_val) in pairs {
        let local_distance = left_val.abs_diff(right_val);
        total_distance = total_distance.checked_add(local_distance).unwrap();
    }
    
    println!("total distance: {}", total_distance);
    
    return Ok(());
}

