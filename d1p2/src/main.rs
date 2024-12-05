use std::collections::HashMap;

fn main() -> Result<(), std::io::Error> {
    let input = std::fs::read_to_string("input.txt")?;
    let cells = input.lines()
        .flat_map(|line| line.split("   "))
        .map(|cell| cell.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let mut freq_map: HashMap<u32, u32> = HashMap::new();
    
    let right_list = cells.iter().skip(1).step_by(2).copied();
    for right_cell in right_list {
        let count = freq_map.entry(right_cell).or_default();
        *count += 1;
    }

    let mut similarity_score = 0u32;

    let left_list = cells.iter().step_by(2).copied();
    for left_cell in left_list {
        let freq = freq_map.get(&left_cell).copied().unwrap_or_default();
        let product = left_cell.checked_mul(freq).unwrap();
        let new_similarity_score = similarity_score.checked_add(product).unwrap();
        similarity_score = new_similarity_score;
    }
    
    println!("similarity score: {}", similarity_score);
    return Ok(());
}
