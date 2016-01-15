use super::Price;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CarOption {
    category: String,
    options: Vec<CarOptionDetail>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CarOptionDetail {
    id: String,
    name: String,
    description: Option<String>,
    equipmentType: String,
    price: Price,
    manufactureOptionName: Option<String>,
    manufactureOptionCode: Option<String>,
}