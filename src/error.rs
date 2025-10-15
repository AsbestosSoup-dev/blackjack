use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Invalid number of decks: {0}")]
    InvalidDecks(u8),

    #[error("Invalid cut position: {0}, must be between 1 and {1}")]
    InvalidCutPosition(usize, usize),

    #[error("Invalid bet limits: must be between {min} and {max}")]
    InvalidBetLimits { min: u32, max: u32 },

    #[error("Invalid port: {0}")]
    InvalidPort(u16),

    #[error("Invalid configuration: {0}")]
    Other(String),
}

#[derive(Error, Debug)]
pub enum GameError {
    #[error("Shoe needs reshuffling")]
    ShoeNeedsReshuffling,

    #[error("Bet too low: {bet} < min {min}")]
    BetTooLow { bet: u32, min: u32 },

    #[error("Bet too high: {bet} > max {max}")]
    BetTooHigh { bet: u32, max: u32 },

    #[error("Insufficient credits: need {bet}, have {credits}")]
    InsufficientCredits { bet: u32, credits: u32 },

    #[error("Game is full")]
    GameFull,
}

#[derive(Error, Debug)]
pub enum NetworkError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Connection Closed")]
    ConnectionClosed,

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Authentication error: {0}")]
    AuthError(String),

    #[error("Discovery timeout")]
    DiscoveryTimeout,
}

#[derive(Error, Debug)]
pub enum PersistenceError {
    #[error("Failed to save: {0}")]
    SaveError(#[from] std::io::Error),

    #[error("Failed to load: {0}")]
    LoadError(String),

    #[error("Corrupted save file: {0}")]
    CorruptedSaveError(String),

    #[error("Session not found: {0}")]
    SessionNotFoundError(String),
}

pub type ConfigResult<T> = std::result::Result<T, ConfigError>;
pub type GameResult<T> = std::result::Result<T, GameError>;
pub type NetworkResult<T> = std::result::Result<T, NetworkError>;
pub type PersistenceResult<T> = std::result::Result<T, PersistenceError>;
