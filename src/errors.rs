#[derive(Debug)]
pub enum NelsonError {
    Internal(String),
    BackendUnreachable(String, String),
    Http(u16),
    InvalidResponse(String),
    ModelError(String),
    EmptyResponse,
}

impl std::fmt::Display for NelsonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NelsonError::Internal(err) => {
                write!(f, "Internal nelson error: {}", err)
            }

            NelsonError::BackendUnreachable(provider, url) => {
                write!(
                    f,
                    "Could not connect to {}. Is it running at {}?",
                    provider, url
                )
            }

            NelsonError::Http(400) => {
                write!(
                    f,
                    "Backend rejected the request (HTTP 400). Check your model name and configuration."
                )
            }

            NelsonError::Http(code) => {
                write!(
                    f,
                    "Backend returned an unexpected HTTP error (status {}).",
                    code
                )
            }

            NelsonError::InvalidResponse(err) => {
                write!(
                    f,
                    "Received an invalid response. The server may have crashed or returned malformed data. Error: {}",
                    err
                )
            }

            NelsonError::ModelError(err) => {
                write!(f, "Model error: {}. Is the model available?", err)
            }

            NelsonError::EmptyResponse => {
                write!(f, "The model returned no output.")
            }
        }
    }
}
