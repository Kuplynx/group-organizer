use std::collections::HashMap;

use itertools::Itertools;

pub fn create_usize_map(
    map: &HashMap<String, Vec<String>>,
) -> (
    HashMap<usize, Vec<usize>>,
    HashMap<String, usize>,
    HashMap<usize, String>,
) {
    // take this map and convert each unique string to a usize
    // and return a map of usize to Vec<usize>
    // and a map of usize to String
    let mut unique_strings: Vec<String> = map.keys().cloned().collect();
    unique_strings.sort();
    unique_strings.dedup();
    let mut string_to_usize: HashMap<String, usize> = HashMap::new();
    let mut usize_to_string: HashMap<usize, String> = HashMap::new();
    for (i, s) in unique_strings.iter().enumerate() {
        string_to_usize.insert(s.clone(), i);
        usize_to_string.insert(i, s.clone());
    }
    let mut usize_map: HashMap<usize, Vec<usize>> = HashMap::new();
    for (k, v) in map.iter() {
        let k_usize = string_to_usize.get(k).unwrap();
        let mut v_usize: Vec<usize> = Vec::new();
        for s in v {
            let s_usize = string_to_usize.get(s).unwrap();
            v_usize.push(*s_usize);
        }
        usize_map.insert(*k_usize, v_usize);
    }
    (usize_map, string_to_usize, usize_to_string)
}

pub fn find_valid_permutations(
    map: &HashMap<usize, Vec<usize>>,
    group_size: usize,
) -> Vec<Vec<usize>> {
    let permutations: Vec<Vec<usize>> = map
        .iter()
        .permutations(group_size)
        .map(|combo: Vec<(&usize, &Vec<usize>)>| {
            combo
                .into_iter()
                .map(|(k, _)| k.clone())
                .collect::<Vec<usize>>()
        })
        .collect();

    let valid = permutations.into_iter().filter(|permutation: &Vec<usize>| {
        for i in 0..group_size {
            for j in 0..group_size {
                if i != j {
                    if map.get(&permutation[i]).unwrap().contains(&permutation[j])
                        || map.get(&permutation[j]).unwrap().contains(&permutation[i])
                    {
                        return false;
                    }
                }
            }
        }
        true
    });

    let valid: Vec<Vec<usize>> = valid.collect();
    valid
}
