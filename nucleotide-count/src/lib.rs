use std::collections::HashMap;

static VALID_NUCLEOTIDES: &'static str = "ACGT";

fn valid(c: char) -> Result<char, char> {
    if VALID_NUCLEOTIDES.contains(c) {
        Ok(c)
    } else {
        Err(c)
    }
}

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    valid(nucleotide)?;
    let mut count = 0;

    for c in dna.chars() {
        if valid(c)? == nucleotide {
            count += 1;
        }
    }
    Ok(count)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut map: HashMap<char, usize> = VALID_NUCLEOTIDES.chars().map(|c| (c, 0)).collect();
    for nucleotide in dna.chars() {
        if let Some(n) = map.get_mut(&nucleotide) {
            *n += 1;
        } else {
            return Err(nucleotide);
        }
    }
    Ok(map)
}
