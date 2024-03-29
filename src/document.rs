use crate::{Position, Row, SearchDirection, FileType};
use std::{
    fs,
    io::{Error, Write},
};

#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
    pub file_name: Option<String>,
    dirty: bool,
    file_type: FileType,
}

impl Document {
    pub fn open(file_name: &str) -> Result<Self, std::io::Error> {
        let content = fs::read_to_string(file_name)?;
        let file_type = FileType::from(file_name);
        let mut rows = Vec::new();

        for value in content.lines() {
            let mut row = Row::from(value);
            row.highlight(&file_type.highlight_options(), None);
            rows.push(row);
        }

        Ok(Self {
            rows,
            file_name: Some(file_name.to_string()),
            dirty: false,
            file_type,
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
            row.highlight(&self.file_type.highlight_options(), None);
            self.rows.push(row);
        } else {
            let row = &mut self.rows[at.y];
            row.highlight(&self.file_type.highlight_options(), None);
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
            row.highlight(&self.file_type.highlight_options(), None);
        } else {   
            let row = &mut self.rows[at.y];
            row.delete(at.x);
            row.highlight(&self.file_type.highlight_options(), None);
        }
    }

    pub fn insert_newline(&mut self, at: &Position) {
        if at.y > self.rows.len() {
            return;
        }

        if at.y == self.rows.len() {
            self.rows.push(Row::default());
            return;
        }

        let current_row = &mut self.rows[at.y];
        let mut new_row = current_row.split(at.x);

        new_row.highlight(&self.file_type.highlight_options(), None);
        current_row.highlight(&self.file_type.highlight_options(), None);

        self.rows.insert(at.y + 1, new_row);
    }

    pub fn save(&mut self) -> Result<(), Error> {
        if let Some(file_name) = &self.file_name {
            let mut file = fs::File::create(file_name)?;
            self.file_type = FileType::from(file_name);

            for row in &mut self.rows {
                file.write_all(row.as_bytes())?;
                file.write_all(b"\n")?;
                row.highlight(self.file_type.highlight_options(), None)
            }

            self.dirty = false;
        }

        Ok(())
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn find(&self, query: &str, at: &Position, direction: SearchDirection) -> Option<Position> {
        if at.y >= self.rows.len() {
            return None;
        }

        let mut position = at.clone();

        let start = if direction == SearchDirection::Forward {
            at.y
        } else {
            0
        };

        let end = if direction == SearchDirection::Forward {
            self.rows.len()
        } else {
            at.y + 1
        };

        for _ in start..end {
            if let Some(row) = self.rows.get(position.y) {
                if let Some(x) = row.find(&query, position.x, direction) {
                    position.x = x;
                    return Some(position);
                }
                if direction == SearchDirection::Forward {
                    position.y = position.y.saturating_add(1);
                    position.x = 0;
                } else {
                    position.y = position.y.saturating_sub(1);
                    position.x = self.rows[position.y].len();
                }
            } else {
                return None;
            }
        }

        None
    }

    pub fn highlight(&mut self, word: Option<&str>) {
        for row in &mut self.rows {
            row.highlight(&self.file_type.highlight_options(), word);
        }
    }

    pub fn file_type(&self) -> String {
        self.file_type.name()
    }
}
