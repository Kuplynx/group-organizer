use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub struct UnsafePointer<T> {
    pub ptr: *mut T,
}

unsafe impl Send for UnsafePointer<u64> {}
unsafe impl Sync for UnsafePointer<u64> {}
impl Clone for UnsafePointer<u64> {
    fn clone(&self) -> Self {
        UnsafePointer { ptr: self.ptr }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ExactCover {
    pub groups: Vec<Vec<usize>>,
    pub target_cover: usize,
    pub solved: bool,
    pub attempts_made: u64,
    pub times_backtracked: u64,
}

unsafe impl Send for ExactCover {}
unsafe impl Sync for ExactCover {}

impl Clone for ExactCover {
    fn clone(&self) -> Self {
        ExactCover {
            groups: self.groups.clone(),
            target_cover: self.target_cover,
            solved: self.solved,
            attempts_made: self.attempts_made,
            times_backtracked: self.times_backtracked,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct InputData {
    pub map: HashMap<String, Vec<String>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct OutputData {
    pub cover: Vec<Vec<String>>,
    pub attempts_made: u64,
    pub times_backtracked: u64,
    pub time_taken_secs: f64,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct QueryInfo {
    pub group_size: usize,
    pub num_groups: usize,
}