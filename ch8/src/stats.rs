/**
 * Given a list of integers, use a vector and return the mean
 * (the average value), median (when sorted, the value in the
 * middle position), and mode (the value that occurs most
 * often) of the list.
 */

use std::collections::HashMap;

pub struct ListStats {
    pub mean: f64,
    pub median: f64,
    pub mode: i64
}

pub fn print(stats: &ListStats) {
    println!("Statistics:");
    println!("mean: {}", stats.mean);
    println!("median: {}", stats.median);
    println!("mode: {}", stats.mode);
}

/// vec must be sorted.
pub fn get(list: &Vec<i64>) -> Option<ListStats> {
    let first = list.first()?;
    let mut sum = *first;
    let mut occurrences: HashMap<i64, usize> = HashMap::new();
    list.iter().skip(1).for_each(|e| {
        sum += e;
        let count = occurrences.entry(*e).or_insert(0);
        *count += 1;
    });
    let mode = (occurrences.iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .unwrap()).0;
    let res = ListStats {
        mean: sum as f64 / list.len() as f64,
        median: get_median(&list).unwrap(),
        mode: *mode
    };
    Some(res)
}

fn get_median(list: &Vec<i64>) -> Option<f64> {
    let len = list.len();
    if len == 0 {
        return None
    }

    let mid_idx = len / 2;
    let mid = list[mid_idx] as f64;
    if len % 2 == 0 {
        Some((mid + list[mid_idx + 1] as f64) / 2.0)
    } else {
        Some(mid)
    }
}

#cfg(test)