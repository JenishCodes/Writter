use crate::{editor::Position, Row};
use std::{
    fs,
    io::{Error, Write},
};

#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
    pub file_name: Option<String>,
    dirty: bool,
}

impl Document {
    pub fn open(file_name: &str) -> Result<Self, std::io::Error> {
        let content = fs::read_to_string(file_name)?;
        let mut rows = Vec::new();

        for value in content.lines() {
            rows.push(Row::from(value));
        }

        Ok(Self {
            rows,
            file_name: Some(file_name.to_string()),
            dirty: false,
        })
    }

    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }

    pub fn insert(&mut self, at: &Position, c: char) {
        if at.y > self.rows.len() {
            return;
        }
        self.dirty = true;

        if c == '\n' {
            self.insert_newline(at);
            return;
        }

        if at.y == self.rows.len() {
            let mut row = Row::default();
            row.insert(0, c);
            self.rows.push(row);
        } else {
            let row = &mut self.rows[at.y];
            row.insert(at.x, c);
        }
    }

    pub fn delete(&mut self, at: &Position) {
        let len = self.rows.len();
        if at.y >= len {
            return;
        }
        self.dirty = true;

        if at.x == self.rows[at.y].len() && at.y + 1 < len {
            let next_row = self.rows.remove(at.y + 1);
            let row = &mut self.rows[at.y];
            row.append(&next_row);
        }
        let row = &mut self.rows[at.y];
        row.delete(at.x);
    }

    pub fn insert_newline(&mut self, at: &Position) {
        if at.y >  self.rows.len() {
            return;
        }

        if at.y == self.rows.len() {
            self.rows.push(Row::default());
        }

        let new_row = self.rows[at.y].split(at.x);

        self.rows.insert(at.y.saturating_add(1), new_row);
    }

    pub fn save(&mut self) -> Result<(), Error> {
        if let Some(file_name) = &self.file_name {
            let mut file = fs::File::create(file_name)?;
            for row in &self.rows {
                file.write_all(row.as_bytes())?;
                file.write_all(b"\n")?;
            }
            self.dirty = false;
        }

        Ok(())
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }
}
