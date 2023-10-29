#[cfg(feature = "qrcode")]
use qrcode::types::QrError;

#[allow(clippy::enum_variant_names)]
#[derive(Debug)]
pub enum InviteifyError {
    AuthenticationError,
    APIError,
    ReqwestError(reqwest::Error),
    #[cfg(feature = "qrcode")]
    QrError(QrError),
}

impl std::error::Error for InviteifyError {}

impl std::fmt::Display for InviteifyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
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
