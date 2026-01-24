use std::fmt;

pub enum NelsonError {
    Internal(String),
    BackendUnreachable(String, u16),
    Http(u16),
    InvalidResponse,
    ModelError(String),
    EmptyResponse,
}

impl fmt::Display for NelsonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NelsonError::Internal(err) => {
                write!(f, "internal nelson error: {}", err)
            }

            NelsonError::BackendUnreachable(host, port) => {
                write!(
                    f,
                    "Could not connect to Ollama. Is it running at {}:{}?",
                    host, port
                )
            }

            NelsonError::Http(400) => {
                write!(
                    f,
                    "Ollama rejected the request (HTTP 400). Check your model name and configuration."
                )
            }

            NelsonError::Http(code) => {
                write!(
                    f,
                    "Ollama returned an unexpected HTTP error (status {}).",
                    code
                )
            }

            NelsonError::InvalidResponse => {
                write!(
                    f,
                    "Received an invalid response from Ollama. The server may have crashed or returned malformed data."
                )
            }

            NelsonError::ModelError(err) => {
                write!(
                    f,
                    "Model error: {}. Is the model pulled and available?",
                    err
                )
            }

            NelsonError::EmptyResponse => {
                write!(
                    f,
                    "The model returned no output."
                )
            }
        }
    }
}

