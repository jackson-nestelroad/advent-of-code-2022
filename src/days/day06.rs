use crate::common::{AocError, AocResult};

fn find_marker_position(buffer: &[u8], length: usize) -> AocResult<usize> {
    let stop_at = buffer.len() - length + 1;
    // For each potential marker starting location...
    'outer: for i in 0..stop_at {
        // For each potential character in the marker...
        for j in 0..length {
            let current = buffer[i + j];
            // Compare with each subsequent character in the marker.
            for k in (j + 1)..length {
                if current == buffer[i + k] {
                    continue 'outer;
                }
            }
        }
        return Ok(i + length);
    }
    Err(AocError::new(format!("no marker of length {length} found")))
}

pub fn solve_a(input: &str) -> AocResult<u64> {
    const MARKER_LENGTH: usize = 4;
    Ok(find_marker_position(input.as_bytes(), MARKER_LENGTH)? as u64)
}

pub fn solve_b(input: &str) -> AocResult<u64> {
    const MARKER_LENGTH: usize = 14;
    Ok(find_marker_position(input.as_bytes(), MARKER_LENGTH)? as u64)
}
