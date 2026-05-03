/// Diagnostic functions for debugging RLE invariants
use crate::PixelSet;

impl PixelSet {
    /// Check if the runs satisfy the RLE invariants.
    /// Returns detailed info about violations.
    pub fn validate_invariants(&self) -> Result<(), String> {
        if self.runs.is_empty() {
            return Ok(());
        }

        // Check 1: Runs must be sorted by (y, x_start)
        for i in 1..self.runs.len() {
            let prev = self.runs[i - 1];
            let curr = self.runs[i];

            if prev.y > curr.y || (prev.y == curr.y && prev.x_start > curr.x_start) {
                return Err(format!(
                    "Unsorted: runs[{}]={:?} > runs[{}]={:?}",
                    i - 1, prev, i, curr
                ));
            }

            // Check 2: No overlapping runs on same row
            if prev.y == curr.y && prev.x_start + prev.length > curr.x_start {
                return Err(format!(
                    "Overlap on row {}: [{},{}] overlaps [{},{}]",
                    prev.y, prev.x_start, prev.x_start + prev.length - 1,
                    curr.x_start, curr.x_start + curr.length - 1
                ));
            }

            // Check 3: No adjacent runs on same row (should be merged)
            if prev.y == curr.y && prev.x_start + prev.length == curr.x_start {
                return Err(format!(
                    "Adjacent on row {}: [{},{}] and [{},{}] should merge",
                    prev.y, prev.x_start, prev.x_start + prev.length - 1,
                    curr.x_start, curr.x_start + curr.length - 1
                ));
            }
        }

        // Check 4: All runs have length >= 1
        for (i, run) in self.runs.iter().enumerate() {
            if run.length == 0 {
                return Err(format!("Run[{}] has zero length", i));
            }
        }

        Ok(())
    }

    /// Count pixels via iteration
    pub fn pixel_count_slow(&self) -> usize {
        self.iter().count()
    }
}
