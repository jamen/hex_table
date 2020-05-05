pub struct HexTable {
    pub base: usize,
    pub columns: usize,
    pub row_limit: Option<usize>,
}

impl Default for HexTable {
    fn default() -> Self {
        HexTable { base: 0, columns: 16, row_limit: None }
    }
}

impl HexTable {
    pub fn new(base: usize, columns: usize, row_limit: Option<usize>) -> HexTable {
        HexTable { base, columns, row_limit }
    }

    pub fn format(&self, data: &[u8]) -> String {
        let data_len = data.len();
        let columns_remaining = data_len % self.columns;
        let columns_empty = if columns_remaining == 0 { 0 } else { self.columns - columns_remaining };
        let table_len_needed = data_len + columns_empty;
        let offset_str_len = format!("{:x}", self.base + data.len() - columns_remaining).len();
        let col_pad_len = format!("{:x}", self.columns).len().max(2);

        let mut output = String::new();

        // Create column header
        self.column_header(&mut output, offset_str_len, col_pad_len);

        // Create rows of bytes
        let mut col_pos = 0;
        let mut row_pos = self.base;

        let rows_iter = match self.row_limit {
            Some(lm) => (0..lm).into_iter(),
            None => (0..(table_len_needed / self.columns)).into_iter()
        };

        for _ in rows_iter {
            output.push_str(&format!("{:0>1$x}: ", row_pos, offset_str_len));

            for _ in 0..self.columns {
                output.push_str(&format!("{:0>1$x} ", data[col_pos], col_pad_len));

                col_pos += 1;

                if col_pos >= data_len { break }
            }

            output.push_str("\n");

            row_pos += self.columns;

            if row_pos >= data_len { break }
        }

        output
    }

    pub fn format_with_ascii(&self, data: &[u8]) -> String {
        let data_len = data.len();
        let columns_remaining = data_len % self.columns;
        let columns_empty = if columns_remaining == 0 { 0 } else { self.columns - columns_remaining };
        let table_len_needed = data_len + columns_empty;
        let offset_str_len = format!("{:x}", self.base + data.len() - columns_remaining).len();
        let col_pad_len = format!("{:x}", self.columns).len().max(2);
        let last_ascii_pad_len = columns_empty * (col_pad_len + 1);

        let mut output = String::new();

        // Create column header
        self.column_header(&mut output, offset_str_len, col_pad_len);

        // Create rows of bytes
        let mut col_pos = 0;
        let mut ascii_pos = 0;
        let mut row_pos = self.base;

        let rows_iter = match self.row_limit {
            Some(lm) => (0..lm).into_iter(),
            None => (0..(table_len_needed / self.columns)).into_iter()
        };

        for _ in rows_iter {
            output.push_str(&format!("{:0>1$x}: ", row_pos, offset_str_len));

            for _ in 0..self.columns {
                output.push_str(&format!("{:0>1$x} ", data[col_pos], col_pad_len));

                col_pos += 1;

                if col_pos >= data_len { break }
            }

            if col_pos >= data_len {
                output.push_str(&format!("{}", " ".repeat(last_ascii_pad_len)));
            }

            for _ in 0..self.columns {
                if data[ascii_pos].is_ascii_alphanumeric() {
                    output.push_str(&format!("{}", data[ascii_pos] as char));
                } else {
                    output.push_str(&format!("."));
                }

                ascii_pos += 1;

                if ascii_pos >= data_len { break }
            }

            output.push_str("\n");

            row_pos += self.columns;

            if row_pos >= data_len { break }
        }

        output
    }

    fn column_header(&self, output: &mut String, offset_str_len: usize, col_pad_len: usize) {
        output.push_str(&format!("{}", " ".repeat(offset_str_len + 2)));

        for i in 0..self.columns {
            output.push_str(&format!("{:0>1$x} ", i, col_pad_len));
        }

        output.push_str("\n");
    }
}