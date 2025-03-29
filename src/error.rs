use thiserror::Error;

#[derive(Error, Debug)]
pub enum GhHeatError {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
    
    #[error("GitHub API error: {0}")]
    Api(String),
    
    #[error("Failed to parse data: {0}")]
    Parse(String),
    
    #[error("Invalid date format: {0}")]
    InvalidDate(String),
    
    #[error("Rate limit exceeded. Please try again later.")]
    RateLimit,
    
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
