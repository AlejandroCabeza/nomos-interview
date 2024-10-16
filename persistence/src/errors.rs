use thiserror::Error;

#[derive(Debug)]
pub struct AffectedRows {
    count: usize,
    expected: usize,
}

impl AffectedRows {
    pub fn new(count: usize, expected: usize) -> Self {
        Self { count, expected }
    }
}

#[derive(Error, Debug)]
pub enum PersistenceError {
    #[error("unexpected affected rows")]
    UnexpectedAffectedRows(AffectedRows),
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
}
