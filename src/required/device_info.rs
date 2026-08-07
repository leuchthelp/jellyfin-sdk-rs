use uuid::Uuid;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct DeviceInfo {
    pub id: Uuid,
    pub name: &'static str,
    pub languages: Option<Vec<&'static str>>,
}
