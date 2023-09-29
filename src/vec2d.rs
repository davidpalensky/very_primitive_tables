//! Tools for working with two dimensional vectors of strings.

/// Converts a csv to a vec.
pub fn csv_to_vec2d(s: &str) -> Vec<Vec<String>> {
    let mut vec = Vec::new();
    for line in s.lines() {
        let mut row = Vec::new();
        let mut i = 0;
        while i < line.len() {
            let next = line[i..].find(',').and_then(|index| Some(index + i));
            if let Some(next) = next {
                row.push(line[i..next].to_owned());
                if line.len() <= next + 1 {
                    break;
                }
                i = next + 1;
            }
            if next == None {
                row.push(line[i..].to_owned());
                break;
            }
        }
        vec.push(row);
    }
    vec
}

/// Converts a csv to a vec.
pub fn csv_to_vec2d_ref(s: &str) -> Vec<Vec<&str>> {
    let mut vec = Vec::new();
    for line in s.lines() {
        let mut row = Vec::new();
        let mut i = 0;
        while i < line.len() {
            let next = line[i..].find(',').and_then(|index| Some(index + i));
            if let Some(next) = next {
                row.push(&line[i..next]);
                if line.len() <= next + 1 {
                    break;
                }
                i = next + 1;
            }
            if next == None {
                row.push(&line[i..]);
                break;
            }
        }
        vec.push(row);
    }
    vec
}

/// Dereferences the individual types.
pub fn deref_inner(v: &Vec<Vec<String>>) -> Vec<Vec<&str>> {
    let mut vec = vec![];
    for row in v {
        let mut row_v = vec![];
        for cell in row {
            row_v.push(cell.as_str());
        }
        vec.push(row_v);
    }
    return vec;
}
