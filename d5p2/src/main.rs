use std::collections::{HashMap, HashSet};

fn reorder<'a>(seq: &Vec<&'a str>, rules: &HashMap<&str, HashSet<&str>>) 
-> Vec<&'a str> 
{
    let mut placed = HashSet::<&str>::new();
    let mut ordered_list = Vec::<&str>::new();
    let seq_set = HashSet::<&str>::from_iter(seq.iter().copied());

    'place: while placed.len() < seq.len() {
        for candidate in seq.iter().copied() {
            if placed.contains(candidate) { continue; }
             
            let mut ok = true; 
            if let Some(dependencies) = rules.get(&candidate) {
                for dependency in dependencies {
                    if seq_set.contains(dependency) {
                        ok &= placed.contains(dependency);
                    }
                }
            }
            if ok {
                ordered_list.push(candidate);
                placed.insert(candidate);
                continue 'place;
            }
        }
        panic!("impossible to place some");
    }
    return ordered_list;
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    
    let [rules_str, inputs_str] = input.split("\n\n")
        .collect::<Vec<&str>>().try_into().unwrap();

    let mut rules = HashMap::<&str, HashSet<&str>>::new();
    
    for rule_str in rules_str.lines() {
        let [left, right] = rule_str.split("|")
            .collect::<Vec<&str>>().try_into().unwrap();

        let dependencies = rules.entry(right).or_default();

        dependencies.insert(left);
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
        if !ok {
            let reordered_seq = reorder(&seq, &rules);
            let middle_i = reordered_seq.len() / 2;
            assert!(seq.len() > 0, "middle element undefined for empty seq");
            let middle_el = reordered_seq[middle_i].parse::<u64>().unwrap(); 
            sum = sum.checked_add(middle_el).unwrap();
        }
    }

    println!("{sum}");
}
