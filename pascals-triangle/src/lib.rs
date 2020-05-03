use std::{iter::once, mem::replace};

pub struct PascalsTriangle(u32);

fn compute_next_row(prev_row: &Vec<u32>) -> Vec<u32> {
    // Sum pairs of internal elements
    let middle = prev_row.windows(2).map(|w| w.iter().sum());
    // Tack a '1' onto either end and put result into a Vec
    once(1).chain(middle).chain(once(1)).collect()
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        Self(row_count)
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        // Infinite iterator over rows of Pascal's Triangle
        let rows = (0..).scan(vec![1], |state, _| {
            Some(replace(state, compute_next_row(state)))
        });

        // Take only as many rows as have been requested
        let row_count = self.0 as usize;
        rows.take(row_count).collect()
    }
}
