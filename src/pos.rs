#[derive(Default)]
pub struct AutoReposition {
    row: i32,
    col: i32,
    max_row: i32,
    max_col: i32,
}

impl AutoReposition {
    pub fn new() -> AutoReposition {
        AutoReposition::default()
    }

    pub fn row(&self) -> i32 {
        self.row
    }

    pub fn col(&self) -> i32 {
        self.col
    }

    pub fn max_row(&self) -> i32 {
        self.max_row
    }
    pub fn max_col(&self) -> i32 {
        self.max_col
    }

    pub fn next_col(&mut self) -> i32 {
        self.col += 1;
        self.max_col = self.max_col.max(self.col);
        self.col
    }

    pub fn next_row(&mut self) -> i32 {
        self.row += 1;
        self.max_row = self.max_row.max(self.row);
        self.col = 0;
        self.row
    }
}
