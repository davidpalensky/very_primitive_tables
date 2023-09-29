//! Library for pretty printing tables.

use std::cmp::max;

#[derive(Debug)]
/// Holds a table to be printed.
///
/// Do not use this to store data, it just holds references to elsewhere.
/// This structure is meant to be created just when it is used.
pub struct Table<'a> {
    /// Holds references to the data being printed.
    data: Vec<Vec<&'a str>>,
    /// How many columns the table holds
    col_n: usize,
    /// Holds the lengths of each coloumns content
    col_widths: Vec<usize>,
}

#[derive(Debug, Clone, Copy)]
pub enum TableError {
    MismatchedRowWidth,
    InvalidCSV,
}

pub type TableResult<T> = Result<T, TableError>;

/// Base functions
impl<'a> Table<'a> {
    /// Creates a new table.
    pub fn new(width: usize) -> Self {
        Self {
            data: vec![],
            col_n: width,
            col_widths: vec![],
        }
    }
    /// Can error if the length of r has a mismatching number of columns.
    pub fn add_row(&mut self, r: &[&'a str]) -> TableResult<()> {
        // Check width
        use TableError as TE;
        if r.len() != self.col_n {
            return Err(TE::MismatchedRowWidth);
        }
        self.data.push(r.to_vec());
        // Update lengths
        if self.col_widths.is_empty() {
            self.col_widths = r.iter().map(|s| s.len()).collect()
        }
        for i in 0..r.len() {
            self.col_widths[i] = max(self.col_widths[i], r[i].len());
        }
        Ok(())
    }
    /// If i is not in table, it will return None.
    pub fn get_row<'b>(&'b self, i: usize) -> Option<&'b [&'a str]> {
        match i < self.data.len() {
            true => Some(self.data[i].as_slice()),
            false => None,
        }
    }
}

/// Rendering
impl Table<'_> {
    /// Renders the table prettily.
    pub fn render(&self) -> String {
        let mut s = String::new();
        self.render_border(&mut s);
        for i in 0..self.data.len() {
            self.render_row(i, &mut s);
            self.render_border(&mut s);
        }
        return s;
    }
    /// Renders a table border: "+-----+----+--+-----+".
    fn render_border(&self, s: &mut String) {
        s.push_str("+");
        for column in self.col_widths.iter() {
            s.push_str("-");
            (0..*column).into_iter().for_each(|_| s.push('-'));
            s.push_str("-+");
        }
        s.push('\n');
    }
    /// Renders a row, but without the borders.
    fn render_row(&self, i: usize, s: &mut String) {
        if i >= self.data.len() {
            panic!("Row {i} exceeds length of table.");
        }
        s.push_str("|");
        for (ci, current_s) in self.data[i].iter().enumerate() {
            s.push_str(" ");
            s.push_str(current_s);
            // Space after text.
            (0..self.col_widths[ci] - current_s.len()).for_each(|_| s.push(' '));
            s.push_str(" |");
        }
        s.push('\n');
    }
}

/// Type conversions.
impl<'a> Table<'a> {
    /// Creates from a two dimensional vector.
    ///
    /// Could return TableError::MismatchedRowWidth if there are a different number
    /// of elements in rows.
    pub fn from_vec2d(vector: &Vec<Vec<&'a str>>) -> TableResult<Table<'a>> {
        // Count columns in first row.
        let cols = Table::count_csv_cols(&vector);
        // Add rows
        let mut table = Table::new(cols);
        for row in vector {
            table.add_row(&row)?;
        }
        return Ok(table);
    }
    /// Returns the amount of elements in the first row, i.e., amount of cols.
    fn count_csv_cols(v: &Vec<Vec<&str>>) -> usize {
        v[0].len()
    }
}
