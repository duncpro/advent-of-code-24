use regex::Regex;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let regex = Regex::new(r#"mul\([0-9]{1,3},[0-9]{1,3}\)"#).unwrap();
    let mut total_products: u64 = 0;
    for r#match in regex.find_iter(&input) {
        let match_str = r#match.as_str();
        let trimmed = &match_str[4..(match_str.len() - 1)];
        let mut args_iter = trimmed.split(",").map(|s| s.parse::<u64>().unwrap());
        let left_operand = args_iter.next().unwrap();
        let right_operand = args_iter.next().unwrap();
        let product = left_operand.checked_mul(right_operand).unwrap();
        total_products = total_products.checked_add(product).unwrap();
    }
    println!("total products: {}", total_products);
}
