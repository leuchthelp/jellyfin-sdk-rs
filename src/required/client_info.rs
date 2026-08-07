#[derive(Eq, PartialEq, Debug, Clone)]
pub struct ClientInfo {
    pub name: &'static str,
    pub version: &'static str,
}
