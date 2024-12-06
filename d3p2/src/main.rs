use regex::Regex;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    
    const DODONT_REGEX: &'static str = r#"(do|don't)\(\)"#;
    const MUL_REGEX: &'static str = r#"mul\([0-9]{1,3},[0-9]{1,3}\)"#;
    let command_regex = Regex::new(&format!("{DODONT_REGEX}|{MUL_REGEX}")).unwrap();
    
    let mut total_products: u64 = 0;
    let mut enabled = true;

    // The docs for regex crate claim find_iter has worst-case runtime complexity O(mn^2).
    // If I didn't use this crate, and instead used  prefix tree, I would have guaranteed
    // worst-case runtime complexity O(mn) where m is the length of the input and n is the
    // length of the longest pattern.
    //
    // However, Advent of Code is long, and I am already behind, so I will leave that for
    // another day.
    for r#match in command_regex.find_iter(&input) {
        let match_str = r#match.as_str();

        // Not as effecient as a prefix tree, but nevertheless these checks are
        // O(1) runtime complexity.
        if match_str == "do()" { enabled = true; continue; }
        if match_str == "don't()" { enabled = false; continue; }

        if enabled {
            let trimmed = &match_str[4..(match_str.len() - 1)];
            let mut args_iter = trimmed.split(",").map(|s| s.parse::<u64>().unwrap());
            let left_operand = args_iter.next().unwrap();
            let right_operand = args_iter.next().unwrap();
            let product = left_operand.checked_mul(right_operand).unwrap();
            total_products = total_products.checked_add(product).unwrap();
        }
    }
    
    println!("total products: {}", total_products);
}
