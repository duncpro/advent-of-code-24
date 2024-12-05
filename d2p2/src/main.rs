fn main() -> Result<(), std::io::Error> {
    let input = std::fs::read_to_string("input.txt")?;

    let mut total_safe = 0usize;

    'reports:
    for line in input.lines() {        
        let mut levels = line.split(" ").map(|cell_text| cell_text.parse::<u32>().unwrap()).collect::<Vec<u32>>();

        // Use the exact same approach as in d2p1, except try each report k times where k is equal to the length of the report.
        // For each iteration we exlcude the k'th level from the report and see if it is safe.
        // 
        // This DOES NOT work for input.txt where k might be less than or equal to 2. Luckily, our is not. 
        // Why? Assume k <= 2, then removing an element, makes levels have length <= 1, which means
        // the iterator returned by zip will be empty, which means max_delta will never be assigned to any value above 0,
        // which means the candidate will never be deemed safe.
        //
        // I also believe this solution to have suboptimal runtime complexity. I will try and remember to come back to it
        // and optimize it when I have time. 
        //
        // Currently the worst-case runtime complexity is approximately O(m * n * (2n)) = O(m * n^2) where
        // m is the number of lines in the file and n is the maximum number of levels in a report.
        //
        // This reminds me of the archetypal maximum length subarray problem, and so if I were to guess I could
        // probably optimize this to be O(m*n). I will do that once I've caught up.
         
        for i in 0..levels.len() {
            let removed = levels[i];
            levels.remove(i);

            let mut increasing = true;
            let mut decreasing = true;
            let mut max_delta = 0u32;
        
            let steps = std::iter::zip(levels.iter().copied(), levels.iter().skip(1).copied());
            for (left, right) in steps {
                increasing &= (left < right);
                decreasing &= (left > right);
                let step_delta = left.abs_diff(right);
                max_delta = std::cmp::max(step_delta, max_delta);
            }

            levels.insert(i, removed);
            
            let safe = (increasing || decreasing) && (1u32..=3).contains(&max_delta);
            if safe { total_safe += 1; continue 'reports; }
        }
    }
    println!("total safe: {}", total_safe);
    return Ok(());
}
