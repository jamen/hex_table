use std::io;

pub struct HexTable {
    columns: usize,
    offset: usize,
    header: bool,
    ascii: bool,
    zeros: bool,
}

impl Default for HexTable {
    fn default() -> Self {
        HexTable { columns: 16, offset: 0, header: false, ascii: true, zeros: true }
    }
}

impl HexTable {
    pub fn new(columns: usize, offset: usize, header: bool, ascii: bool, zeros: bool) -> HexTable {
        HexTable { columns, offset, header, ascii, zeros }
    }

    pub fn format<Writer: io::Write>(&self, data: &[u8], out: &mut Writer) -> io::Result<()> {
        let mut col_pos = 0;
        let mut row_pos = 0;
        let mut ascii_pos = 0;

        let data_len = data.len();
        let col_pad_len = Self::count_hex_digits(self.columns).max(2);
        let columns_remaining = data_len % self.columns;

        let columns_empty = if columns_remaining != 0 {
            self.columns - columns_remaining
        } else {
            0
        };

        let offset_str_len = Self::count_hex_digits(
            if columns_remaining != 0 {
                self.offset + data.len() - columns_remaining
            } else {
                self.offset + data.len() - columns_remaining - 1
            }
        );

        let last_row_pad_len = columns_empty * (col_pad_len + 1);
        let table_len_needed = data_len + columns_empty;

        if self.header {
            let offset = " ".repeat(offset_str_len + 2);
            let line = "-".repeat((self.columns * (col_pad_len + 1)) - 1);

            write!(out, "{}", offset)?;

            for i in 0..self.columns {
                write!(out, "{:0>1$X} ", i, col_pad_len)?;
            }

            write!(out, "\n{}{}\n", offset, line)?;
        }

        for _ in 0..(table_len_needed / self.columns) {
            write!(out, "{:0>1$X}: ", self.offset + row_pos, offset_str_len)?;

            for _ in 0..self.columns {
                if !self.zeros && data[col_pos] == 0 {
                    write!(out, "{} ", ".".repeat(col_pad_len))?;
                } else {
                    write!(out, "{:0>1$X} ", data[col_pos], col_pad_len)?;
                }

                col_pos += 1;

                if col_pos >= data_len {
                    break
                }
            }

            if self.ascii {
                if col_pos >= data_len {
                    write!(out, "{}", " ".repeat(last_row_pad_len))?;
                }

                write!(out, "| ")?;

                for _ in 0..self.columns {
                    if data[ascii_pos].is_ascii_alphanumeric() {
                        write!(out, "{}", data[ascii_pos] as char)?;
                    } else {
                        write!(out, ".")?;
                    }

                    ascii_pos += 1;

                    if ascii_pos >= data_len {
                        break
                    }
                }
            }

            row_pos += self.columns;

            if row_pos >= data_len {
                break
            }

            write!(out, "\n")?;
        }

        write!(out, "\n")?;

        Ok(())
    }

    // This was fun. It counts the number of hex digits with log16(x) + 1 for whole numbers. It uses
    // the algorithm log2(x) = size_of(x) - 1 - clz(x) then a base conversion log2(x) / log2(16).
    fn count_hex_digits(x: usize) -> usize {
        (8 * std::mem::size_of::<usize>() - 1 - x.leading_zeros() as usize) / (7 - 16u8.leading_zeros() as usize) + 1
    }
}