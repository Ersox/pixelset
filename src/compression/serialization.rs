use std::collections::HashMap;
use std::io::{Error, ErrorKind, Result as IoResult};

#[derive(Clone, Debug)]
pub(super) struct Rectangle {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

impl Rectangle {
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        Self { x, y, width, height }
    }
}

/// Serialize rectangles to raw binary format using dimension lookup tables.
pub(super) fn to_bytes(rectangles: &[Rectangle]) -> Vec<u8> {
    let count = rectangles.len();

    let mut width_map: HashMap<u16, u8> = HashMap::new();
    let mut unique_widths = Vec::new();
    for rect in rectangles {
        if !width_map.contains_key(&rect.width) {
            width_map.insert(rect.width, unique_widths.len() as u8);
            unique_widths.push(rect.width);
        }
    }

    let mut height_map: HashMap<u16, u8> = HashMap::new();
    let mut unique_heights = Vec::new();
    for rect in rectangles {
        if !height_map.contains_key(&rect.height) {
            height_map.insert(rect.height, unique_heights.len() as u8);
            unique_heights.push(rect.height);
        }
    }

    let mut bytes = Vec::new();

    bytes.extend_from_slice(&(count as u32).to_le_bytes());

    bytes.extend_from_slice(&(unique_widths.len() as u16).to_le_bytes());
    for &w in &unique_widths {
        bytes.extend_from_slice(&w.to_le_bytes());
    }

    bytes.extend_from_slice(&(unique_heights.len() as u16).to_le_bytes());
    for &h in &unique_heights {
        bytes.extend_from_slice(&h.to_le_bytes());
    }

    for rect in rectangles {
        bytes.extend_from_slice(&rect.x.to_le_bytes());
    }

    for rect in rectangles {
        bytes.extend_from_slice(&rect.y.to_le_bytes());
    }

    for rect in rectangles {
        bytes.push(width_map[&rect.width]);
    }

    for rect in rectangles {
        bytes.push(height_map[&rect.height]);
    }

    bytes
}

/// Deserialize rectangles from raw binary format with dimension lookup tables.
pub(super) fn from_bytes(bytes: &[u8]) -> IoResult<Vec<Rectangle>> {
    if bytes.len() < 4 {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "Insufficient data for rectangle count",
        ));
    }

    let mut offset = 0;
    let count = u32::from_le_bytes([bytes[offset], bytes[offset + 1], bytes[offset + 2], bytes[offset + 3]]) as usize;
    offset += 4;

    if offset + 2 > bytes.len() {
        return Err(Error::new(ErrorKind::InvalidData, "Insufficient data for width count"));
    }
    let num_widths = u16::from_le_bytes([bytes[offset], bytes[offset + 1]]) as usize;
    offset += 2;

    let mut widths_table = Vec::with_capacity(num_widths);
    for _ in 0..num_widths {
        if offset + 2 > bytes.len() {
            return Err(Error::new(ErrorKind::InvalidData, "Insufficient data for widths"));
        }
        widths_table.push(u16::from_le_bytes([bytes[offset], bytes[offset + 1]]));
        offset += 2;
    }

    if offset + 2 > bytes.len() {
        return Err(Error::new(ErrorKind::InvalidData, "Insufficient data for height count"));
    }
    let num_heights = u16::from_le_bytes([bytes[offset], bytes[offset + 1]]) as usize;
    offset += 2;

    let mut heights_table = Vec::with_capacity(num_heights);
    for _ in 0..num_heights {
        if offset + 2 > bytes.len() {
            return Err(Error::new(ErrorKind::InvalidData, "Insufficient data for heights"));
        }
        heights_table.push(u16::from_le_bytes([bytes[offset], bytes[offset + 1]]));
        offset += 2;
    }

    let mut xs = Vec::with_capacity(count);
    for _ in 0..count {
        if offset + 2 > bytes.len() {
            return Err(Error::new(ErrorKind::InvalidData, "Insufficient data for x values"));
        }
        xs.push(u16::from_le_bytes([bytes[offset], bytes[offset + 1]]));
        offset += 2;
    }

    let mut ys = Vec::with_capacity(count);
    for _ in 0..count {
        if offset + 2 > bytes.len() {
            return Err(Error::new(ErrorKind::InvalidData, "Insufficient data for y values"));
        }
        ys.push(u16::from_le_bytes([bytes[offset], bytes[offset + 1]]));
        offset += 2;
    }

    let mut width_indices = Vec::with_capacity(count);
    for _ in 0..count {
        if offset >= bytes.len() {
            return Err(Error::new(ErrorKind::InvalidData, "Insufficient data for width indices"));
        }
        width_indices.push(bytes[offset] as usize);
        offset += 1;
    }

    let mut height_indices = Vec::with_capacity(count);
    for _ in 0..count {
        if offset >= bytes.len() {
            return Err(Error::new(ErrorKind::InvalidData, "Insufficient data for height indices"));
        }
        height_indices.push(bytes[offset] as usize);
        offset += 1;
    }

    let mut rectangles = Vec::with_capacity(count);
    for i in 0..count {
        let width = widths_table
            .get(width_indices[i])
            .copied()
            .ok_or_else(|| Error::new(ErrorKind::InvalidData, "Invalid width index"))?;
        let height = heights_table
            .get(height_indices[i])
            .copied()
            .ok_or_else(|| Error::new(ErrorKind::InvalidData, "Invalid height index"))?;
        rectangles.push(Rectangle::new(xs[i], ys[i], width, height));
    }

    Ok(rectangles)
}
