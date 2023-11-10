pub struct AutoReposition {
    row: i32,
    col: i32,
}

impl AutoReposition {
    pub fn new() -> AutoReposition {
        AutoReposition { row: 0, col: 0 }
    }
    pub fn row(&self) -> i32 {
        self.row
    }

    pub fn col(&mut self) -> i32 {
        self.col += 1;
        self.col
    }

    pub fn new_row(&mut self) -> i32 {
        self.row += 1;
        self.col = 0;
        self.row
    }
}
