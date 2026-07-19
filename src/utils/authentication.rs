use crate::required::{ClientInfo, DeviceInfo};
use std::fmt::Write as _;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthHeaderError {
    #[error(transparent)]
    FailedHeaderConstructionError(#[from] std::fmt::Error),
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
