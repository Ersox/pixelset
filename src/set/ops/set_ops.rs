use crate::{Pixel, PixelSet};
use crate::set::Run;
use super::run_ops::{difference_row_runs, end_of_row, merge_row_runs, push_run, xor_row_runs};

// All set operations below use the same merge-scan strategy: maintain one index into
// `self.runs` and one into `other.runs`, and advance whichever points to the smaller `y`
// (breaking ties by `x_start`). Because both lists are sorted by `(y, x_start)` and contain
// no overlapping or adjacent runs, this single linear pass is sufficient to compare every
// relevant pair of runs exactly once. Per-row logic is delegated to `run_ops`.

impl PixelSet {
    /// Returns `true` if every pixel in this set is also present in `other`.
    ///
    /// Complexity: `O(k1 + k2)` where k1, k2 are the number of runs in each set.
    pub fn is_subset(&self, other: &PixelSet) -> bool {
        let mut other_idx = 0;

        for self_run in &self.runs {
            // Advance past other runs on earlier rows.
            while other_idx < other.runs.len() && other.runs[other_idx].y < self_run.y {
                other_idx += 1;
            }
            // Advance past other runs on the same row that end before self_run starts.
            while other_idx < other.runs.len()
                && other.runs[other_idx].y == self_run.y
                && other.runs[other_idx].x_end() < self_run.x_start
            {
                other_idx += 1;
            }

            if other_idx >= other.runs.len() || other.runs[other_idx].y != self_run.y {
                return false;
            }

            let cover = other.runs[other_idx];
            if cover.x_start > self_run.x_start || cover.x_end() < self_run.x_end() {
                return false;
            }
        }

        true
    }

    /// Returns `true` if this set shares any pixel with another set.
    ///
    /// Complexity: `O(k1 + k2)`.
    pub fn intersects(&self, other: &Self) -> bool {
        let mut self_idx = 0;
        let mut other_idx = 0;

        while self_idx < self.runs.len() && other_idx < other.runs.len() {
            let self_run = self.runs[self_idx];
            let other_run = other.runs[other_idx];

            if self_run.y < other_run.y {
                self_idx += 1;
            } else if self_run.y > other_run.y {
                other_idx += 1;
            } else if self_run.overlaps(other_run) {
                return true;
            } else if self_run.x_end() < other_run.x_start {
                self_idx += 1;
            } else {
                other_idx += 1;
            }
        }

        false
    }

    /// Inserts a single new pixel into the set while maintaining sorted
    /// order and uniqueness.
    ///
    /// Worst-case complexity: `O(k)` due to run insertion/splitting.
    pub fn add(&mut self, pixel: Pixel) {
        if self.find_run_for(pixel).is_some() {
            return;
        }

        let idx = self.runs.partition_point(|r| r.key() < pixel.key());

        let prev_run = idx.checked_sub(1).and_then(|i| {
            if self.runs[i].y == pixel.y { Some(self.runs[i]) } else { None }
        });

        let next_run = if idx < self.runs.len() && self.runs[idx].y == pixel.y {
            Some(self.runs[idx])
        } else {
            None
        };

        match (prev_run, next_run) {
            (None, None) => {
                self.runs.insert(idx, Run { y: pixel.y, x_start: pixel.x, length: 1 });
            }
            (Some(prev), None) if prev.x_end() + 1 == pixel.x => {
                self.runs[idx - 1].length += 1;
            }
            (None, Some(next)) if pixel.x + 1 == next.x_start => {
                self.runs[idx].x_start = pixel.x;
                self.runs[idx].length += 1;
            }
            (Some(prev), Some(next))
                if prev.x_end() + 1 == pixel.x && pixel.x + 1 == next.x_start =>
            {
                self.runs[idx - 1] = Run::from_span(prev.y, prev.x_start, next.x_end());
                self.runs.remove(idx);
            }
            (Some(prev), Some(_)) if prev.x_end() + 1 == pixel.x => {
                self.runs[idx - 1].length += 1;
            }
            (Some(_), Some(next)) if pixel.x + 1 == next.x_start => {
                self.runs[idx].x_start = pixel.x;
                self.runs[idx].length += 1;
            }
            _ => {
                self.runs.insert(idx, Run { y: pixel.y, x_start: pixel.x, length: 1 });
            }
        }
    }

    /// Removes a pixel from the set, maintaining sort order.
    ///
    /// Worst-case complexity: `O(k)` due to run splitting.
    pub fn discard(&mut self, pixel: Pixel) {
        let run_idx = match self.find_run_for(pixel) {
            Some(i) => i,
            None => return,
        };

        let run = &mut self.runs[run_idx];

        if run.length == 1 {
            self.runs.remove(run_idx);
        } else if pixel.x == run.x_start {
            run.x_start += 1;
            run.length -= 1;
        } else if pixel.x == run.x_end() {
            run.length -= 1;
        } else {
            let new_run = Run::from_span(run.y, pixel.x + 1, run.x_end());
            run.length = pixel.x - run.x_start;
            self.runs.insert(run_idx + 1, new_run);
        }
    }

    /// Returns a new `PixelSet` containing only the pixels that appear in
    /// both sets, performing a set intersection.
    ///
    /// Complexity: `O(k1 + k2)`.
    pub fn and(&self, other: &Self) -> Self {
        let mut result = Vec::with_capacity(self.runs.len().min(other.runs.len()));
        let mut self_idx = 0;
        let mut other_idx = 0;

        while self_idx < self.runs.len() && other_idx < other.runs.len() {
            let self_run = self.runs[self_idx];
            let other_run = other.runs[other_idx];

            if self_run.y < other_run.y {
                self_idx += 1;
            } else if self_run.y > other_run.y {
                other_idx += 1;
            } else {
                let x_start = self_run.x_start.max(other_run.x_start);
                let x_end = self_run.x_end().min(other_run.x_end());

                if x_start <= x_end {
                    result.push(Run::from_span(self_run.y, x_start, x_end));
                }

                if self_run.x_end() <= other_run.x_end() {
                    self_idx += 1;
                } else {
                    other_idx += 1;
                }
            }
        }

        Self::from_runs_unchecked(result)
    }

    /// Returns a new `PixelSet` representing the union of this set and another.
    ///
    /// Complexity: `O(k1 + k2)`.
    pub fn or(&self, other: &Self) -> Self {
        if self.is_empty() {
            return other.clone();
        }
        if other.is_empty() {
            return self.clone();
        }

        let mut result = Vec::with_capacity(self.runs.len() + other.runs.len());
        let mut self_idx = 0;
        let mut other_idx = 0;

        while self_idx < self.runs.len() && other_idx < other.runs.len() {
            let self_run = self.runs[self_idx];
            let other_run = other.runs[other_idx];

            if self_run.y < other_run.y {
                result.push(self_run);
                self_idx += 1;
            } else if self_run.y > other_run.y {
                result.push(other_run);
                other_idx += 1;
            } else {
                let row_y = self_run.y;
                let s_end = end_of_row(&self.runs, self_idx, row_y);
                let o_end = end_of_row(&other.runs, other_idx, row_y);
                merge_row_runs(
                    &self.runs[self_idx..s_end],
                    &other.runs[other_idx..o_end],
                    row_y,
                    &mut result,
                );
                self_idx = s_end;
                other_idx = o_end;
            }
        }

        result.extend_from_slice(&self.runs[self_idx..]);
        result.extend_from_slice(&other.runs[other_idx..]);

        Self::from_runs_unchecked(result)
    }

    /// Returns the symmetric difference of two sets: pixels that appear in exactly one of the sets.
    ///
    /// Complexity: `O(k1 + k2)`.
    pub fn xor(&self, other: &Self) -> Self {
        let mut result = Vec::with_capacity(self.runs.len() + other.runs.len());
        let mut self_idx = 0;
        let mut other_idx = 0;

        while self_idx < self.runs.len() && other_idx < other.runs.len() {
            let a = self.runs[self_idx];
            let b = other.runs[other_idx];

            if a.y < b.y {
                push_run(&mut result, a);
                self_idx += 1;
            } else if a.y > b.y {
                push_run(&mut result, b);
                other_idx += 1;
            } else {
                let row_y = a.y;
                let s_end = end_of_row(&self.runs, self_idx, row_y);
                let o_end = end_of_row(&other.runs, other_idx, row_y);
                xor_row_runs(
                    &self.runs[self_idx..s_end],
                    &other.runs[other_idx..o_end],
                    row_y,
                    &mut result,
                );
                self_idx = s_end;
                other_idx = o_end;
            }
        }

        for run in &self.runs[self_idx..] {
            push_run(&mut result, *run);
        }
        for run in &other.runs[other_idx..] {
            push_run(&mut result, *run);
        }

        Self::from_runs_unchecked(result)
    }

    /// Returns a new `PixelSet` with pixels in this set that are not in `other`,
    /// performing a set difference.
    ///
    /// Complexity: `O(k1 + k2)`.
    pub fn difference(&self, other: &Self) -> Self {
        let mut result = Vec::with_capacity(self.runs.len());
        let mut self_idx = 0;
        let mut other_idx = 0;

        while self_idx < self.runs.len() {
            let self_run = self.runs[self_idx];

            while other_idx < other.runs.len() && other.runs[other_idx].y < self_run.y {
                other_idx += 1;
            }

            if other_idx >= other.runs.len() || other.runs[other_idx].y > self_run.y {
                result.push(self_run);
            } else {
                let o_end = end_of_row(&other.runs, other_idx, self_run.y);
                difference_row_runs(self_run, &other.runs[other_idx..o_end], &mut result);
            }

            self_idx += 1;
        }

        Self::from_runs_unchecked(result)
    }
}
