fn main() -> Result<(), std::io::Error> {
    let input = std::fs::read_to_string("input.txt")?;

    let mut total_safe = 0usize;
    
    for line in input.lines() {
        let mut increasing = true;
        let mut decreasing = true;
        let mut max_delta = 0u32;
        
        let levels = line.split(" ").map(|cell_text| cell_text.parse::<u32>().unwrap());
        let steps = std::iter::zip(levels.clone(), levels.clone().skip(1));
        for (left, right) in steps {
            increasing &= (left < right);
            decreasing &= (left > right);
            let step_delta = left.abs_diff(right);
            max_delta = std::cmp::max(step_delta, max_delta);
        }
        
        let safe = (increasing || decreasing) && (1u32..=3).contains(&max_delta);
        if safe { total_safe += 1; }
    }
    println!("total safe: {}", total_safe);
    return Ok(());
}
