use std::collections::{HashMap, HashSet};

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    
    let [rules_str, inputs_str] = input.split("\n\n")
        .collect::<Vec<&str>>().try_into().unwrap();

    let mut rules = HashMap::<&str, HashSet<&str>>::new();
    
    for rule_str in rules_str.lines() {
        let [left, right] = rule_str.split("|")
            .collect::<Vec<&str>>().try_into().unwrap();

        let disallowed_successors = 
            rules.entry(right).or_default();

        disallowed_successors.insert(left);
    }

    let mut sum = 0u64;

    for input_str in inputs_str.lines() {
        let seq = input_str.split(",").collect::<Vec<&str>>();
        let mut ok = true;
        for (i, el) in seq.iter().copied().enumerate() {
            let Some(disallowed) = rules.get(&el) else { continue };
            for succesor in seq[i..].iter().copied() {
                ok &= !disallowed.contains(succesor);
            }
        }
        if ok {
            let middle_i = seq.len() / 2;
            assert!(seq.len() > 0, "middle element undefined for empty seq");
            let middle_el = seq[middle_i].parse::<u64>().unwrap(); 
            sum = sum.checked_add(middle_el).unwrap();
        }
    }

    println!("{sum}");
}
