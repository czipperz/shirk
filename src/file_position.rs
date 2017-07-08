use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct FilePosition {
    pub filename: Rc<String>,
    pub line: usize,
    pub column: usize,
}
