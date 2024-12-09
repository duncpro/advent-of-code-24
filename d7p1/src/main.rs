fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let mut sum_accepted_targets = 0u64; 

    for line in input.lines() {
        let [target_str, eq_str] = line.split(": ")
            .collect::<Vec<&str>>().try_into().unwrap();

        let target = target_str.parse::<u64>().unwrap();

        let terms = eq_str.split(" ").map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

        let accepted = test(&terms[1..], terms[0], target);

        if accepted {
            sum_accepted_targets = sum_accepted_targets.checked_add(target)
                .unwrap();
        }
    }

    println!("{sum_accepted_targets}");
}

fn test(terms: &[u64], curr: u64, target: u64) -> bool {
    if terms.is_empty() { return curr == target; }

    let term = terms[0];
    let rest = &terms[1..];

    let mut accept = false;
    accept |= test(rest, term + curr, target);
    accept |= test(rest, term * curr, target);
    return accept;
}
