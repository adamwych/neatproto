#[derive(Default)]
pub struct IndentedWriter {
    buffer: String,
    indent: usize,
}

impl IndentedWriter {
    pub fn push_indent(&mut self) {
        self.indent += 1;
    }

    pub fn pop_indent(&mut self) {
        self.indent = self.indent.saturating_sub(1);
    }

    pub fn write_string(&mut self, string: String) {
        let spaces = self.indent * 4;
        self.buffer.extend(std::iter::repeat(' ').take(spaces));
        self.buffer.push_str(&string);
    }

    pub fn write_line<S: ToString>(&mut self, string: S) {
        self.write_string(string.to_string());
        self.buffer.push('\n');
    }

    pub fn to_string(self) -> String {
        self.buffer
    }
}
