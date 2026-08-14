use crate::required::{ClientInfo, DeviceInfo};
use serde::Serialize;
use std::fmt::Write as _;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthHeaderError {
    #[error(transparent)]
    FailedHeaderConstructionError(#[from] std::fmt::Error),
}

impl Serialize for AuthHeaderError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

pub fn get_authorization_header(
    client_info: &ClientInfo,
    device_info: &DeviceInfo,
    access_token: Option<&String>,
) -> Result<String, AuthHeaderError> {
    let mut header = String::from("MediaBrowser ");

    write!(&mut header, r#"Client="{}", "#, client_info.name)?;
    write!(&mut header, r#"Device="{}", "#, device_info.name)?;
    write!(&mut header, r#"DeviceId="{}", "#, device_info.id)?;
    write!(&mut header, r#"Version="{}", "#, client_info.version)?;

    match access_token {
        Some(token) => write!(&mut header, "Token=\"{}\"", token)?,
        _ => {}
    }

    Ok(header)
}
