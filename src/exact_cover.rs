use std::{collections::VecDeque, vec};

use crate::types::{ExactCover, UnsafePointer};

const MAX_ATTEMPTS: u64 = 1 << 37; // ~137 billion (a little over a minute on my M4 macbook pro)

impl ExactCover {
    pub fn new(groups: Vec<Vec<usize>>, target_groups_count: usize) -> ExactCover {
        ExactCover {
            solved: false,
            groups: groups,
            attempts_made: 0 as u64,
            times_backtracked: 0 as u64,
            target_cover: target_groups_count,
        }
    }

    pub fn solve(&mut self) -> Vec<Vec<usize>> {
        // let mut used_students: Vec<bool> =
        //     vec![false; self.groups.iter().flatten().max().unwrap() + 1];
        // let mut sol_v2_vec: Vec<Vec<usize>> = vec![vec![]; self.target_cover];
        // let mut selected_groups: VecDeque<&[usize]> = VecDeque::new();
        // let groups: &Vec<Vec<usize>> = &self.groups;
        let group_masks: Vec<u128> = self
            .groups
            .iter()
            .map(|group| {
                group
                    .iter()
                    .fold(0u128, |mask, &student| mask | (1 << student))
            })
            .collect();
        let mut selected_indices = Vec::with_capacity(self.target_cover);
        let initial_mask: u128 = 0;
        let start_index = 0;
        if find_valid_set_bitmasked(
            &group_masks,
            initial_mask,
            start_index,
            &mut selected_indices,
            self.target_cover,
            &UnsafePointer {
                ptr: &mut self.attempts_made as *mut u64,
            },
            &UnsafePointer {
                ptr: &mut self.times_backtracked as *mut u64,
            },
        ) {
            // 4. Convert indices back to the original groups for the result
            let result = selected_indices
                .into_iter()
                .map(|i| self.groups[i].clone())
                .collect();
            return Some(result).unwrap();
        }

        vec![]
        // self.solved = find_valid_set(
        //     groups,
        //     &mut selected_groups,
        //     used_students.as_mut_ptr(),
        //     &mut sol_v2_vec as *mut _,
        //     &UnsafePointer {
        //         ptr: &mut self.attempts_made as *mut u64,
        //     },
        //     &UnsafePointer {
        //         ptr: &mut self.times_backtracked as *mut u64,
        //     },
        //     self.target_cover,
        // );
        // let sol: Vec<Vec<usize>> = sol_v2_vec;
        // sol
    }
    pub fn _attempts(&self) -> u64 {
        self.attempts_made
    }
    pub fn _backtracks(&self) -> u64 {
        self.times_backtracked
    }
}

fn find_valid_set<'a>(
    groups: &'a [Vec<usize>],
    selected_groups: &mut VecDeque<&'a [usize]>,
    used_students: *mut bool,
    solution_vec: *mut Vec<Vec<usize>>,
    attempts_made: &UnsafePointer<u64>,
    times_backtracked: &UnsafePointer<u64>,
    target_groups_count: usize,
) -> bool {
    if selected_groups.len() == target_groups_count {
        let solution: Vec<Vec<usize>> = selected_groups
            .iter()
            .map(|g: &&[usize]| g.to_vec())
            .collect();
        // {
        //     let mut solution_vec_writer: RwLockWriteGuard<'_, Vec<Vec<usize>>> =
        //         solution_vec.write().unwrap();
        //     *solution_vec_writer = solution;
        // }
        unsafe {
            *solution_vec = solution;
        }
        return true;
    }
    let attempts_made_ref: &UnsafePointer<u64> = &attempts_made;
    let times_backtracked_ref: &UnsafePointer<u64> = &times_backtracked;

    for group in groups {
        unsafe {
            *attempts_made_ref.ptr += 1;
            if *attempts_made_ref.ptr >= MAX_ATTEMPTS {
                return false;
            }
        }

        if group
            .iter()
            .any(|student: &usize| unsafe { *used_students.add(*student) })
        {
            continue;
        }

        selected_groups.push_back(group.as_slice());

        for student in group {
            unsafe {
                *used_students.add(*student) = true;
            }
        }
        let result: bool = find_valid_set(
            groups,
            selected_groups,
            used_students,
            solution_vec,
            attempts_made_ref,
            times_backtracked_ref,
            target_groups_count,
        );

        if result {
            return result;
        }
        selected_groups.pop_back();
        for student in group {
            unsafe {
                *used_students.add(*student) = false;
            }
        }
    }
    unsafe {
        *times_backtracked_ref.ptr += 1;
    }

    false
}

fn find_valid_set_bitmasked(
    group_masks: &[u128], // The pre-calculated masks
    current_mask: u128,   // Represents all 'used_students'
    start_index: usize,   // Optimization #3: prevents re-checking old groups
    selected_indices: &mut Vec<usize>,
    target_count: usize,
    attempts_made_ref: &UnsafePointer<u64>,
    times_backtracked_ref: &UnsafePointer<u64>
) -> bool {
    // Base Case
    if selected_indices.len() == target_count {
        return true;
    }

    let remaining_needed = target_count - selected_indices.len();
    if group_masks.len() - start_index < remaining_needed {
        return false;
    }

    // Iterating from start_index prevents P(n,k) permutations
    // and focuses on C(n,k) combinations
    for i in start_index..group_masks.len() {
        unsafe {
            *attempts_made_ref.ptr += 1;
            if *attempts_made_ref.ptr >= MAX_ATTEMPTS {
                return false;
            }
        }
        let group_mask = group_masks[i];

        // BITWISE CHECK: replaces group.iter().any(|s| used_students[s])
        // If the bitwise AND is 0, there is no overlap between students
        if (current_mask & group_mask) == 0 {
            selected_indices.push(i);

            // RECURSE: current_mask | group_mask "sets" the bits for the new students
            if find_valid_set_bitmasked(
                group_masks,
                current_mask | group_mask,
                i + 1, // Move forward to avoid duplicates
                selected_indices,
                target_count,
                attempts_made_ref,
                times_backtracked_ref,
            ) {
                return true;
            }

            // Backtrack
            selected_indices.pop();
            unsafe {
                *times_backtracked_ref.ptr += 1;
            }
        }
    }

    false
}
