use std::collections::VecDeque;

use crate::types::{ExactCover, UnsafePointer};

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

    pub fn solve(&mut self, on_billion_attempts: &dyn Fn(u64)) -> Vec<Vec<usize>> {
        let mut used_students: Vec<bool> =
            vec![false; self.groups.iter().flatten().max().unwrap() + 1];
        let mut sol_v2_vec: Vec<Vec<usize>> = vec![vec![]; self.target_cover];
        let mut selected_groups: VecDeque<&[usize]> = VecDeque::new();
        let groups: &Vec<Vec<usize>> = &self.groups;
        self.solved = find_valid_set(
            groups,
            &mut selected_groups,
            used_students.as_mut_ptr(),
            &mut sol_v2_vec as *mut _,
            &UnsafePointer {
                ptr: &mut self.attempts_made as *mut u64,
            },
            &UnsafePointer {
                ptr: &mut self.times_backtracked as *mut u64,
            },
            self.target_cover,
            on_billion_attempts,
        );
        let sol: Vec<Vec<usize>> = sol_v2_vec;
        sol
    }
    pub fn attempts(&self) -> u64 {
        self.attempts_made
    }
    pub fn backtracks(&self) -> u64 {
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
    on_billion_attempts: &dyn Fn(u64),
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
            if *attempts_made.ptr % 1_000_000 == 0 {
                on_billion_attempts(*attempts_made.ptr);
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
            on_billion_attempts,
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