#[derive(Default)]
pub struct IndentedWriter {
    buffer: String,
    indent: usize,
}

#[allow(dead_code)]
impl IndentedWriter {
    pub fn push_indent(&mut self) {
        self.indent += 1;
    }

    pub fn pop_indent(&mut self) {
        self.indent = self.indent.saturating_sub(1);
    }

    pub fn write_indent(&mut self) {
        let spaces = self.indent * 4;
        self.buffer.extend(std::iter::repeat(' ').take(spaces));
    }

    pub fn write(&mut self, string: &str) {
        self.buffer.push_str(string);
    }

    pub fn write_line(&mut self, string: &str) {
        self.buffer.push_str(string);
        self.next_line();
    }

    pub fn write_string<S: ToString>(&mut self, string: S) {
        self.buffer.push_str(string.to_string().as_str());
    }

    pub fn write_string_line<S: ToString>(&mut self, string: S) {
        self.buffer.push_str(string.to_string().as_str());
        self.next_line();
    }

    pub fn write_indented<S: ToString>(&mut self, string: S) {
        self.write_indent();
        self.buffer.push_str(string.to_string().as_str());
    }

    pub fn write_indented_line<S: ToString>(&mut self, string: S) {
        self.write_indented(string.to_string());
        self.next_line();
    }

    pub fn next_line(&mut self) {
        self.buffer.push('\n');
    }

    pub fn to_string(self) -> String {
        self.buffer
    }
}
