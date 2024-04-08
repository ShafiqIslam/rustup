use std::collections::HashMap;

pub fn median_and_mod() {
    let mut v = vec![1, 1, 8, 5, 3, 8, 9, 1, 6, 2, 3];
    v.sort();

    println!("The median is: {}", v[v.len() / 2]);

    let mut map = HashMap::new();
    for n in v {
        let count = map.entry(n).or_insert(0);
        *count += 1;
    }

    let mut max_freq = i32::MIN;
    let mut mod_value = i32::MIN;
    for (k, v) in &map {
        if max_freq < *v {
            mod_value = *k;
            max_freq = *v;
        }
    }
    println!("The mod is: {mod_value}");
}
