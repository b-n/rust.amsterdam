#[cfg(feature = "qrcode")]
use qrcode::types::QrError;

#[allow(clippy::enum_variant_names)]
#[derive(Debug)]
pub enum InviteifyError {
    AuthenticationError,
    APIError(APIError),
    ReqwestError(reqwest::Error),
    #[cfg(feature = "qrcode")]
    QrError(QrError),
}

impl std::error::Error for InviteifyError {}

impl std::fmt::Display for InviteifyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AuthenticationError => write!(f, "Authentication Error"),
            Self::APIError(err) => write!(f, "{}", err),
            Self::ReqwestError(err) => write!(f, "{}", err),
            #[cfg(feature = "qrcode")]
            Self::QrError(err) => write!(f, "{}", err),
        }
    }
}

impl From<APIError> for InviteifyError {
    fn from(err: APIError) -> Self {
        Self::APIError(err)
    }
}

impl From<reqwest::Error> for InviteifyError {
    fn from(err: reqwest::Error) -> Self {
        Self::ReqwestError(err)
    }
}

#[cfg(feature = "qrcode")]
impl From<QrError> for InviteifyError {
    fn from(err: QrError) -> Self {
        Self::QrError(err)
    }
}

#[derive(Debug)]
pub struct APIError {
    url: String,
    status: String,
    body: String,
}

impl APIError {
    pub async fn from_reqwest(res: reqwest::Response) -> Self {
        let url = format!("{}", res.url());
        let status = format!("{}", res.status());
        let body = res
            .text()
            .await
            .unwrap_or("Could not decode body to text".to_string());

        Self { url, status, body }
    }
}

impl std::error::Error for APIError {}

impl std::fmt::Display for APIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "URL: {}, Status: {}, Body: {}",
            self.url, self.status, self.body
        )
    }
}
