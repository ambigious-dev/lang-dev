pub trait ErrorReporter {
    fn error(&mut self, line: u32, col: u32, file: String, kind: String, message: String);
}
