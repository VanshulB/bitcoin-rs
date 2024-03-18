pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Invalid network.
    #[error("Invalid network: {0}")]
    InvalidNetwork(String),
}
