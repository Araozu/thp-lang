
/// Represents an error in the scanning process
#[derive(Debug)]
pub struct LexError {
    /// Position where the offending char was found
    pub position: usize,
    /// Reason of the errror
    pub reason: String,
}
