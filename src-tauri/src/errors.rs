use reqwest::StatusCode;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("HTTP error")]
    Http(#[from] reqwest::Error),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error("Backend not found: {0}")]
    BackendNotFound(String),
    #[error("Error starting backend '{backend:?}': {reason:?}")]
    BackendBoot{reason: String, backend: String},
    #[error("Model '{model:?}' not found in backend '{backend:?}'")]
    ModelNotFound{model: String, backend: String},
    #[error("Internal error - There is a bug: {0}")]
    Internal(String),
    #[error("Internal error")]
    Unknown
}

pub fn internal(msg: impl ToString) -> Error {
    Error::Internal(msg.to_string())
}

#[derive(serde::Serialize)]
#[serde(tag = "kind", content = "message")]
#[serde(rename_all = "camelCase")]
pub enum ErrorKind {
    Io(String),
    Http{status_code: u16, status_msg: String},
    BackendNotFound(String),
    BackendBoot{reason: String, backend: String},
    ModelNotFound{model: String, backend: String},
    Internal(String)
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let error_kind = match self {
            Self::Io(e) => {
                let error_msg = e.to_string();
                ErrorKind::Io(error_msg)
            },
            Self::SerdeJson(e) => {
                let error_msg = e.to_string();
                ErrorKind::Io(error_msg)
            }
            Self::Http(e) => {
                ErrorKind::Http {
                    status_code: e.status().unwrap_or(StatusCode::OK).as_u16(),
                    status_msg: e.to_string()
                }
            },
            Self::BackendNotFound(e) => {
                ErrorKind::BackendNotFound(e.to_owned())
            }
            Self::BackendBoot{reason, backend} => {
                ErrorKind::BackendBoot {reason: reason.to_owned(), backend: backend.to_owned()}
            }
            Self::ModelNotFound { model, backend } => {
                ErrorKind::ModelNotFound { model: model.to_owned(), backend: backend.to_owned() }
            }
            Self::Internal(msg) => {
                ErrorKind::Internal(msg.to_owned())
            },
            Self::Unknown => {
                ErrorKind::Internal(self.to_string())
            }
        };
        error_kind.serialize(serializer)
    }
}
