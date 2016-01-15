#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transmission {
    pub id: String,
    pub name: String,
    pub equipmentType: String,
    pub automaticType: Option<String>,
    pub transmissionType: String,
    pub numberOfSpeeds: String,
}