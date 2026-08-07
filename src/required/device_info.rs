#[derive(Eq, PartialEq, Debug, Clone)]
pub struct DeviceInfo {
    pub id: &'static str,
    pub name: &'static str,
    pub languages: Option<Vec<&'static str>>,
}
