pub enum Error {
    Unauthorized,
    Forbidden,
    NotFound,
    NotUnique(String),
    Other(String),
}
