use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut multiples: HashSet<u32> = HashSet::new();

    for f in factors {
        let f = *f;
        if f != 0 {
            for m in (f..limit).step_by(f as usize) {
                multiples.insert(m);
            }
        }
    }

    multiples.iter().sum()
}
