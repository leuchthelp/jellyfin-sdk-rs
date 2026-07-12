use crate::models::{ClientInfo, DeviceInfo};

pub fn get_authorization_header(client_info: ClientInfo, device_info: DeviceInfo, access_token: Option<String>) -> String {

    return "MediaBrowser".to_string();
}