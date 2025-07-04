use reqwest::StatusCode;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("HTTP error")]
    Http(#[from] reqwest::Error)
}

#[derive(serde::Serialize)]
#[serde(tag = "kind", content = "message")]
#[serde(rename_all = "camelCase")]
pub enum ErrorKind {
    Io(String),
    Http{status_code: u16, status_msg: String}
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
            Self::Http(e) => {
                ErrorKind::Http {
                    status_code: e.status().unwrap_or(StatusCode::OK).as_u16(),
                    status_msg: e.to_string()
                }
            }
        };
        error_kind.serialize(serializer)
    }
}
