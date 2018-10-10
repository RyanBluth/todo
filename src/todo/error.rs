
#[derive(Debug, Fail)]
pub enum ToDoError {
    #[fail(display = "Invalid ID '{}'. Expected value from {} to {}", id, id_start, id_end)]
    InvalidIDError {
        id: usize,
        id_start: usize,
        id_end: usize
    },
}