//! Low-level helpers for building and combining RLE run lists.
//!
//! # The merge-scan pattern
//!
//! Every set operation (`and`, `or`, `xor`, `difference`, `is_subset`, `intersects`) works by
//! scanning both run lists simultaneously with two indices — one into `self.runs`, one into
//! `other.runs`. Because both lists are always sorted by `(y, x_start)`, we can advance the
//! index that points to the smaller `y` (or smaller `x_start` within the same row) and be
//! certain we never miss a relevant pair. This gives O(k1 + k2) instead of O(k1 * k2).
//!
//! When both indices land on the same row, the per-row helpers in this file take over.
//!
//! # Per-row helpers
//!
//! Within a row, three operations are needed:
//!
//! - [`merge_row_runs`] (union): always consumes whole runs in x order, so [`pick_next`] —
//!   which simply picks the leftmost run from either slice — is sufficient.
//!
//! - [`xor_row_runs`] and [`difference_row_runs`] (symmetric difference / subtraction):
//!   a single run from one slice may be split at an interior x-coordinate by a run from the
//!   other slice. These helpers use [`RunCursor`], which tracks a position *within* the current
//!   run so that partially-consumed runs can be resumed.
//!
//! [`push_run`] maintains the non-adjacent invariant on the output buffer: when two consecutive
//! output runs on the same row happen to be adjacent, it merges them instead of appending.

use std::cmp::Ordering;
use crate::set::Run;

/// Push `run` onto `runs`, merging with the last run if they are adjacent on the same row.
pub(super) fn push_run(runs: &mut Vec<Run>, run: Run) {
    if let Some(last) = runs.last_mut()
        && last.y == run.y && last.x_start + last.length == run.x_start
    {
        last.length += run.length;
        return;
    }
    runs.push(run);
}

/// Pick the next run in x-sorted order from `a_runs[*ai]` or `b_runs[*bi]`,
/// advancing the chosen index. Returns `None` when both are exhausted.
fn pick_next(a_runs: &[Run], ai: &mut usize, b_runs: &[Run], bi: &mut usize) -> Option<Run> {
    match (a_runs.get(*ai), b_runs.get(*bi)) {
        (Some(a), Some(b)) if a.x_start <= b.x_start => { *ai += 1; Some(*a) }
        (Some(_), Some(b)) => { *bi += 1; Some(*b) }
        (Some(a), None)    => { *ai += 1; Some(*a) }
        (None, Some(b))    => { *bi += 1; Some(*b) }
        (None, None)       => None,
    }
}

/// Tracks a current x position within a slice of runs, allowing partial consumption.
struct RunCursor<'a> {
    runs: &'a [Run],
    idx: usize,
    pos: u16,
}

impl<'a> RunCursor<'a> {
    fn new(runs: &'a [Run]) -> Self {
        let pos = runs.first().map_or(0, |r| r.x_start);
        Self { runs, idx: 0, pos }
    }

    /// Returns `(current_pos, run_x_end)`, or `None` if exhausted.
    fn peek(&self) -> Option<(u16, u16)> {
        self.runs.get(self.idx).map(|r| (self.pos, r.x_end()))
    }

    /// Mark everything through `end` as consumed. Advances to the next run when
    /// the current run is fully consumed.
    fn consume_through(&mut self, end: u16) {
        let Some(run) = self.runs.get(self.idx) else { return };
        if end >= run.x_end() {
            self.idx += 1;
            self.pos = self.runs.get(self.idx).map_or(0, |r| r.x_start);
        } else {
            self.pos = end + 1;
        }
    }
}

/// Merge all runs from `a_runs` and `b_runs` (both on row `y`) into a union, appending to `out`.
/// Both slices must be sorted by x_start and non-overlapping within each slice.
pub(super) fn merge_row_runs(a_runs: &[Run], b_runs: &[Run], y: u16, out: &mut Vec<Run>) {
    let mut ai = 0;
    let mut bi = 0;

    let Some(first) = pick_next(a_runs, &mut ai, b_runs, &mut bi) else { return };
    let mut curr_start = first.x_start;
    let mut curr_end = first.x_end();

    while let Some(next) = pick_next(a_runs, &mut ai, b_runs, &mut bi) {
        if next.x_start <= curr_end + 1 {
            curr_end = curr_end.max(next.x_end());
        } else {
            out.push(Run::from_span(y, curr_start, curr_end));
            curr_start = next.x_start;
            curr_end = next.x_end();
        }
    }

    out.push(Run::from_span(y, curr_start, curr_end));
}

/// Compute the symmetric difference of `a_runs` and `b_runs` (both on row `y`), appending to `out`.
/// Both slices must be sorted by x_start and non-overlapping within each slice.
pub(super) fn xor_row_runs(a_runs: &[Run], b_runs: &[Run], y: u16, out: &mut Vec<Run>) {
    let mut a = RunCursor::new(a_runs);
    let mut b = RunCursor::new(b_runs);

    loop {
        match (a.peek(), b.peek()) {
            (None, None) => break,
            (Some((ax, a_end)), None) => {
                push_run(out, Run::from_span(y, ax, a_end));
                a.consume_through(a_end);
            }
            (None, Some((bx, b_end))) => {
                push_run(out, Run::from_span(y, bx, b_end));
                b.consume_through(b_end);
            }
            (Some((ax, a_end)), Some((bx, b_end))) => match ax.cmp(&bx) {
                Ordering::Less => {
                    let end = a_end.min(bx - 1);
                    push_run(out, Run::from_span(y, ax, end));
                    a.consume_through(end);
                }
                Ordering::Greater => {
                    let end = b_end.min(ax - 1);
                    push_run(out, Run::from_span(y, bx, end));
                    b.consume_through(end);
                }
                Ordering::Equal => {
                    // Both sets cover this point — skip the shared prefix.
                    let end = a_end.min(b_end);
                    a.consume_through(end);
                    b.consume_through(end);
                }
            },
        }
    }
}

/// Subtract `other_runs` from `self_run`, appending remaining segments to `out`.
/// `other_runs` must be sorted by x_start, non-overlapping, and on the same row as `self_run`.
pub(super) fn difference_row_runs(self_run: Run, other_runs: &[Run], out: &mut Vec<Run>) {
    let mut self_x = self_run.x_start;

    for other_run in other_runs {
        if other_run.x_end() < self_x {
            continue;
        }
        if self_x < other_run.x_start {
            let end = self_run.x_end().min(other_run.x_start - 1);
            out.push(Run::from_span(self_run.y, self_x, end));
            self_x = end + 1;
        }
        self_x = self_x.max(other_run.x_end() + 1);
        if self_x > self_run.x_end() {
            return;
        }
    }

    if self_x <= self_run.x_end() {
        out.push(Run::from_span(self_run.y, self_x, self_run.x_end()));
    }
}

/// Return the index in `runs` just past the last run on row `y`, starting from `idx`.
pub(super) fn end_of_row(runs: &[Run], idx: usize, y: u16) -> usize {
    let mut i = idx;
    while i < runs.len() && runs[i].y == y {
        i += 1;
    }
    i
}
