#[derive(Debug, Clone)]
struct Node {
    left: usize,
    right: usize,
    up: usize,
    down: usize,
    col: usize,
    row_idx: usize,
    size: usize, // Only used for column headers
}

pub struct DlxSolver {
    nodes: Vec<Node>,
    answers: Vec<usize>,
    target: usize,
}

impl DlxSolver {
    pub fn new(groups: &Vec<Vec<usize>>, student_count: usize, target: usize) -> Self {
        let num_cols = student_count;
        // Pre-allocate to prevent mid-search reallocations
        let mut nodes = Vec::with_capacity(num_cols + 1 + (groups.len() * 3));

        // 1. Initialize Column Headers
        for i in 0..=num_cols {
            nodes.push(Node {
                left: if i == 0 { num_cols } else { i - 1 },
                right: if i == num_cols { 0 } else { i + 1 },
                up: i,
                down: i,
                col: i,
                row_idx: usize::MAX,
                size: 0,
            });
        }

        // 2. Build the Grid
        for (r_idx, group) in groups.iter().enumerate() {
            let mut first_in_row: Option<usize> = None;
            for &student in group {
                let col_idx = student + 1; // 1-based index (0 is root)
                let new_idx = nodes.len();

                let col_header = col_idx;
                let old_up = nodes[col_header].up;

                nodes.push(Node {
                    left: new_idx,
                    right: new_idx,
                    up: old_up,
                    down: col_header,
                    col: col_header,
                    row_idx: r_idx,
                    size: 0,
                });

                // Link vertical neighbors
                nodes[old_up].down = new_idx;
                nodes[col_header].up = new_idx;
                nodes[col_header].size += 1; // Increment column size

                // Link horizontal neighbors
                if let Some(first) = first_in_row {
                    let last = nodes[first].left;
                    nodes[new_idx].left = last;
                    nodes[new_idx].right = first;
                    nodes[last].right = new_idx;
                    nodes[first].left = new_idx;
                } else {
                    first_in_row = Some(new_idx);
                }
            }
        }

        DlxSolver {
            nodes,
            answers: Vec::with_capacity(target),
            target,
        }
    }

    pub fn solve(&mut self) -> Option<Vec<usize>> {
        if self.search() {
            return Some(self.answers.clone());
        }
        None
    }

    fn cover(&mut self, c: usize) {
        // Step 1: Extract indices into locals to end the immutable borrow
        let l = self.nodes[c].left;
        let r = self.nodes[c].right;

        // Step 2: Use those locals for the mutable updates
        self.nodes[r].left = l;
        self.nodes[l].right = r;

        let mut i = self.nodes[c].down;
        while i != c {
            let mut j = self.nodes[i].right;
            while j != i {
                let down = self.nodes[j].down;
                let up = self.nodes[j].up;
                let col = self.nodes[j].col;

                self.nodes[down].up = up;
                self.nodes[up].down = down;
                self.nodes[col].size -= 1;

                j = self.nodes[j].right;
            }
            i = self.nodes[i].down;
        }
    }

    fn uncover(&mut self, c: usize) {
        let mut i = self.nodes[c].up;
        while i != c {
            let mut j = self.nodes[i].left;
            while j != i {
                let col = self.nodes[j].col;
                let down = self.nodes[j].down;
                let up = self.nodes[j].up;

                self.nodes[col].size += 1;
                self.nodes[down].up = j;
                self.nodes[up].down = j;

                j = self.nodes[j].left;
            }
            i = self.nodes[i].up;
        }

        let l = self.nodes[c].left;
        let r = self.nodes[c].right;
        self.nodes[r].left = c;
        self.nodes[l].right = c;
    }

    fn search(&mut self) -> bool {
        // If no columns left, we found a solution
        if self.nodes[0].right == 0 {
            return self.answers.len() == self.target;
        }

        // Optimization: Stop if we've already exceeded or can't meet target
        if self.answers.len() >= self.target {
            return false;
        }

        // S-Heuristic: Pick column with smallest size
        let mut c = self.nodes[0].right;
        let mut best_col = c;
        let mut min_size = self.nodes[c].size;

        let mut curr = self.nodes[0].right;
        while curr != 0 {
            if self.nodes[curr].size < min_size {
                min_size = self.nodes[curr].size;
                best_col = curr;
            }
            if min_size == 0 {
                return false;
            } // Prune: impossible column
            curr = self.nodes[curr].right;
        }
        c = best_col;

        self.cover(c);

        let mut r = self.nodes[c].down;
        while r != c {
            self.answers.push(self.nodes[r].row_idx);

            let mut j = self.nodes[r].right;
            while j != r {
                let col_to_cover = self.nodes[j].col;
                self.cover(col_to_cover);
                j = self.nodes[j].right;
            }

            if self.search() {
                return true;
            }

            let mut j = self.nodes[r].left;
            while j != r {
                let col_to_uncover = self.nodes[j].col;
                self.uncover(col_to_uncover);
                j = self.nodes[j].left;
            }

            self.answers.pop();
            r = self.nodes[r].down;
        }

        self.uncover(c);
        false
    }
}

// Wrapper to match your requested calling style
pub fn solve_exact_cover(
    groups: &Vec<Vec<usize>>,
    student_count: usize,
    target: usize,
) -> Option<Vec<Vec<usize>>> {
    let mut solver = DlxSolver::new(groups, student_count, target);
    if let Some(indices) = solver.solve() {
        return Some(indices.iter().map(|&i| groups[i].clone()).collect());
    }
    None
}
