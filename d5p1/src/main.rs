// The scanning procedure has runtime complexity O(k^2) where k is the length of
// the sequence. Proof: For every element in the sequence we visit the remaining
// elements in the sequence. Then the total number of visits is equal to the k'th
// triangle number which grows with the square of k.
//
// If there are m sequences, each with k elements, then obviously the runtime complexity
// of scanning all sequences is O(m * k^2). 
// 
// Of course our scanning procedure is predicated on the existence of the "rules" hash table.
// Constructing the rules hash table has runtime complexity O(n) where n is the total number
// of rules.
//
// The summary worst-case runtime complexity is then O(max(m * k^2), n).
//
// The space complexity ignoring the input text is entirely determined by the "rules" table
// which has a length bounded by the total number of identifiers p.
//
// In the worst-case the number of rules is equal to p^2 / 2 - p, that is, each identifier
// is explicitly related to all the identifiers in the universal set by a rule.
// Then the space complexity is O(p^2).

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
