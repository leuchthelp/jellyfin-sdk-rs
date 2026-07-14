use openapi::apis::configuration::{ApiKey, BasicAuth, Configuration};
use reqwest::header::{HeaderMap, HeaderValue};
use url::Url;

use crate::{
    constants::headers::{ACCEPT_LANGUAGE_HEADER, AUTHORIZATION_HEADER},
    required::{ClientInfo, DeviceInfo},
    utils::authentication::get_authorization_header,
};

#[bon::builder]
pub fn configure(
    base_url: Url,
    client_info: &ClientInfo,
    device_info: &DeviceInfo,
    access_token: &Option<String>,
    basic_auth: Option<BasicAuth>,
    oauth_access_token: Option<String>,
    bearer_access_token: Option<String>,
    api_key: Option<ApiKey>,
) -> Result<Configuration, Box<dyn std::error::Error>> {
    let user_agent = format!("{}: {}", client_info.name, client_info.version);

    let auth_header = get_authorization_header()
        .client_info(client_info)
        .device_info(device_info)
        .access_token(access_token)
        .call()
        .unwrap();

    let mut headers = HeaderMap::new();
    headers.append(AUTHORIZATION_HEADER, HeaderValue::from_str(&auth_header)?);

    let _ = match &device_info.languages {
        Some(languages) => headers.append(
            ACCEPT_LANGUAGE_HEADER,
            HeaderValue::from_str(&languages.join(", "))?,
        ),
        _ => false,
    };

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();

    let config = Configuration {
        base_path: base_url.to_string(),
        user_agent: Some(user_agent),
        client: client,
        basic_auth: basic_auth,
        oauth_access_token: oauth_access_token,
        bearer_access_token: bearer_access_token,
        api_key: api_key,
    };

    Ok(config)
}
