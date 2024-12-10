// In my opinion the prompt for this problem was extremely ambiguous.
// Definitely my least favorite problem so far.

#![feature(portable_simd)]

use std::simd::prelude::*;

const ROW_END_CH: u8 = '\n' as u8;
const EMPTY_CELL: u8 = '.' as u8;

fn main() {
    let input = std::fs::read("input.txt").unwrap();

    let width = input.iter().position(|c| *c == ROW_END_CH).unwrap();
    let height = input.iter().filter(|c| **c == ROW_END_CH).count();

    let idx_of = |p: usizex2| { 
        let [x, y] = p.into();
        y * (width + 1) + x
    };

    let mut antennas = Vec::<usizex2>::new();

    for x in 0..width {
        for y in 0..height {
            let p: usizex2 = [x, y].into();
            if input[idx_of(p)] != EMPTY_CELL {
                antennas.push(p);
            }
        }
    }

    let mut antinodes = 
        std::collections::HashSet::<usizex2>::new();
    
    let inbounds = |p: isizex2| {
        let [x, y] = p.into();
        let x_inbounds = (0..(width as isize)).contains(&x);
        let y_inbounds = (0..(height as isize)).contains(&y);
        x_inbounds && y_inbounds
    };


    for i in 0..antennas.len() {
        for j in 0..antennas.len() {
            if j <= i { continue; }
            
            let (p0, p1) = (antennas[i], antennas[j]);
            let c0 = input[idx_of(p0)];
            let c1 = input[idx_of(p1)];
            if c0 != c1 { continue; }

            let (ps0, ps1): (isizex2, isizex2) = (p0.cast(), p1.cast());
            let delta = ps0 - ps1; // vector from ps1 to ps0
            
            let mut candidate = ps0;
            loop {
                if !(inbounds)(candidate) { break; }
                antinodes.insert(candidate.cast());
                candidate += delta;
            }

            candidate = ps1;
            loop {
                if !(inbounds)(candidate) { break; }
                antinodes.insert(candidate.cast());
                candidate -= delta;
            }
        }
    }
        
    println!("{}", antinodes.len());
}

