use crate::{Pixel, PixelSet};
use crate::set::Run;

impl PixelSet {
    /// Returns `true` if every pixel in this set is also present in `other`.
    ///
    /// Complexity: `O(k1 + k2)` where k1, k2 are the number of runs in each set.
    pub fn is_subset(&self, other: &PixelSet) -> bool {
        let mut self_idx = 0;
        let mut other_idx = 0;

        while self_idx < self.runs.len() && other_idx < other.runs.len() {
            let self_run = self.runs[self_idx];
            let other_run = other.runs[other_idx];

            if self_run.y < other_run.y {
                return false;
            } else if self_run.y > other_run.y {
                other_idx += 1;
            } else {
                if !other.runs[other_idx..].iter().any(|&r| {
                    r.y == self_run.y
                        && r.x_start <= self_run.x_start
                        && r.x_end() >= self_run.x_end()
                }) {
                    return false;
                }
                self_idx += 1;
                while other_idx < other.runs.len() && other.runs[other_idx].y == self_run.y {
                    other_idx += 1;
                }
            }
        }

        self_idx == self.runs.len()
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
            } else {
                if self_run.x_start <= other_run.x_end() && self_run.x_end() >= other_run.x_start {
                    return true;
                }
                if self_run.x_end() < other_run.x_start {
                    self_idx += 1;
                } else {
                    other_idx += 1;
                }
            }
        }

        false
    }

    /// Inserts a single new pixel into the set while maintaining sorted
    /// order and uniqueness.
    ///
    /// Worst-case complexity: `O(k)` due to run insertion/splitting.
    pub fn add(&mut self, pixel: Pixel) {
        let key = pixel.key();
        let idx = self.runs.partition_point(|r| r.key() < key);

        if idx < self.runs.len() && self.runs[idx].y == pixel.y && self.runs[idx].contains_x(pixel.x) {
            return;
        }

        let prev_run = idx.checked_sub(1).and_then(|i| {
            if self.runs[i].y == pixel.y {
                Some(self.runs[i])
            } else {
                None
            }
        });

        let next_run = if idx < self.runs.len() && self.runs[idx].y == pixel.y {
            Some(self.runs[idx])
        } else {
            None
        };

        match (prev_run, next_run) {
            (None, None) => {
                self.runs.insert(idx, Run {
                    y: pixel.y,
                    x_start: pixel.x,
                    length: 1,
                });
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
                self.runs[idx - 1].length = next.x_end() - prev.x_start + 1;
                self.runs.remove(idx);
            }
            (Some(prev), Some(_)) if prev.x_end() + 1 == pixel.x => {
                // Extend prev even though next exists but isn't adjacent
                self.runs[idx - 1].length += 1;
            }
            (Some(_), Some(next)) if pixel.x + 1 == next.x_start => {
                // Prepend to next even though prev exists but isn't adjacent
                self.runs[idx].x_start = pixel.x;
                self.runs[idx].length += 1;
            }
            _ => {
                self.runs.insert(idx, Run {
                    y: pixel.y,
                    x_start: pixel.x,
                    length: 1,
                });
            }
        }
    }

    /// Removes a pixel from the set, maintaining sort order.
    ///
    /// Worst-case complexity: `O(k)` due to run splitting.
    pub fn discard(&mut self, pixel: Pixel) {
        let key = pixel.key();
        let idx = self.runs.partition_point(|r| r.key() < key);

        // Check if pixel is in the run at idx, or in the previous run (if it shares the same y)
        let run_idx = if idx < self.runs.len() && self.runs[idx].y == pixel.y && self.runs[idx].contains_x(pixel.x) {
            idx
        } else if idx > 0 && self.runs[idx - 1].y == pixel.y && self.runs[idx - 1].contains_x(pixel.x) {
            idx - 1
        } else {
            return;
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
            let new_run = Run {
                y: run.y,
                x_start: pixel.x + 1,
                length: run.x_end() - pixel.x,
            };
            run.length = pixel.x - run.x_start;
            self.runs.insert(run_idx + 1, new_run);
        }
    }

    /// Returns a new `PixelSet` containing only the pixels that appear in
    /// both sets, performing a set intersection.
    ///
    /// Complexity: `O(k1 + k2)`.
    pub fn and(&self, other: &Self) -> Self {
        let mut result = Vec::new();
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
                    result.push(Run {
                        y: self_run.y,
                        x_start,
                        length: x_end - x_start + 1,
                    });
                }

                if self_run.x_end() < other_run.x_end() {
                    self_idx += 1;
                } else if self_run.x_end() > other_run.x_end() {
                    other_idx += 1;
                } else {
                    self_idx += 1;
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

        let mut result = Vec::new();
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
                let mut merged = Vec::new();
                let mut s_idx = self_idx;
                let mut o_idx = other_idx;

                // Determine which run comes first by x coordinate
                let first_run = if self.runs[s_idx].x_start <= other.runs[o_idx].x_start {
                    let r = self.runs[s_idx];
                    s_idx += 1;
                    r
                } else {
                    let r = other.runs[o_idx];
                    o_idx += 1;
                    r
                };

                let mut curr_x_start = first_run.x_start;
                let mut curr_x_end = first_run.x_end();

                while (s_idx < self.runs.len() || o_idx < other.runs.len())
                    && (s_idx < self.runs.len() && self.runs[s_idx].y == self_run.y
                        || o_idx < other.runs.len() && other.runs[o_idx].y == other_run.y)
                {
                    let next_run = if s_idx < self.runs.len()
                        && self.runs[s_idx].y == self_run.y
                        && (o_idx >= other.runs.len()
                            || other.runs[o_idx].y != other_run.y
                            || self.runs[s_idx].x_start <= other.runs[o_idx].x_start)
                    {
                        let r = self.runs[s_idx];
                        s_idx += 1;
                        r
                    } else if o_idx < other.runs.len() && other.runs[o_idx].y == other_run.y {
                        let r = other.runs[o_idx];
                        o_idx += 1;
                        r
                    } else {
                        break;
                    };

                    if next_run.x_start <= curr_x_end + 1 {
                        curr_x_end = curr_x_end.max(next_run.x_end());
                    } else {
                        merged.push(Run {
                            y: self_run.y,
                            x_start: curr_x_start,
                            length: curr_x_end - curr_x_start + 1,
                        });
                        curr_x_start = next_run.x_start;
                        curr_x_end = next_run.x_end();
                    }
                }

                merged.push(Run {
                    y: self_run.y,
                    x_start: curr_x_start,
                    length: curr_x_end - curr_x_start + 1,
                });

                result.extend(merged);
                self_idx = s_idx;
                other_idx = o_idx;
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
        let mut result = Vec::new();
        let mut self_idx = 0;
        let mut other_idx = 0;

        while self_idx < self.runs.len() && other_idx < other.runs.len() {
            let a = self.runs[self_idx];
            let b = other.runs[other_idx];

            if a.y < b.y {
                result.push(a);
                self_idx += 1;
            } else if a.y > b.y {
                result.push(b);
                other_idx += 1;
            } else {
                let mut a_x = a.x_start;
                let mut b_x = b.x_start;

                while a_x <= a.x_end() && b_x <= b.x_end() {
                    if a_x < b_x {
                        let end = (a.x_end()).min(b_x - 1);
                        result.push(Run {
                            y: a.y,
                            x_start: a_x,
                            length: end - a_x + 1,
                        });
                        a_x = end + 1;
                    } else if b_x < a_x {
                        let end = (b.x_end()).min(a_x - 1);
                        result.push(Run {
                            y: a.y,
                            x_start: b_x,
                            length: end - b_x + 1,
                        });
                        b_x = end + 1;
                    } else {
                        a_x += 1;
                        b_x += 1;
                    }
                }

                if a_x <= a.x_end() {
                    result.push(Run {
                        y: a.y,
                        x_start: a_x,
                        length: a.x_end() - a_x + 1,
                    });
                }
                if b_x <= b.x_end() {
                    result.push(Run {
                        y: a.y,
                        x_start: b_x,
                        length: b.x_end() - b_x + 1,
                    });
                }

                self_idx += 1;
                other_idx += 1;
            }
        }

        result.extend_from_slice(&self.runs[self_idx..]);
        result.extend_from_slice(&other.runs[other_idx..]);

        Self::from_runs_unchecked(result)
    }

    /// Returns a new `PixelSet` with pixels in this set that are not in `other`,
    /// performing a set difference.
    ///
    /// Complexity: `O(k1 + k2)`.
    pub fn difference(&self, other: &Self) -> Self {
        let mut result = Vec::new();
        let mut self_idx = 0;
        let mut other_idx = 0;

        while self_idx < self.runs.len() {
            let self_run = self.runs[self_idx];

            while other_idx < other.runs.len() && other.runs[other_idx].y < self_run.y {
                other_idx += 1;
            }

            if other_idx >= other.runs.len() || other.runs[other_idx].y > self_run.y {
                result.push(self_run);
                self_idx += 1;
                continue;
            }

            let mut self_x = self_run.x_start;
            let mut curr_other_idx = other_idx;

            while self_x <= self_run.x_end() && curr_other_idx < other.runs.len()
                && other.runs[curr_other_idx].y == self_run.y
            {
                let other_run = other.runs[curr_other_idx];

                if other_run.x_end() < self_x {
                    curr_other_idx += 1;
                    continue;
                }

                if self_x < other_run.x_start {
                    let end = (self_run.x_end()).min(other_run.x_start - 1);
                    result.push(Run {
                        y: self_run.y,
                        x_start: self_x,
                        length: end - self_x + 1,
                    });
                    self_x = end + 1;
                }

                self_x = self_x.max(other_run.x_end() + 1);
                curr_other_idx += 1;
            }

            if self_x <= self_run.x_end() {
                result.push(Run {
                    y: self_run.y,
                    x_start: self_x,
                    length: self_run.x_end() - self_x + 1,
                });
            }

            self_idx += 1;
        }

        Self::from_runs_unchecked(result)
    }
}
