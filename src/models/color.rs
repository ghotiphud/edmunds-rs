use super::Price;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Color {
    category: String,
    options: Vec<ColorOption>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ColorOption {
    id: String,
    name: String,
    equipmentType: String,
    price: Option<Price>,
    manufactureOptionName: String,
    manufactureOptionCode: String,
    colorChips: Option<ColorChips>,
    #[serde(default)]
    fabricTypes: Vec<FabricType>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ColorDetail {
    r: u16,
    g: u16,
    b: u16,
    hex: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ColorChips {
    primary: Option<ColorDetail>,
    secondary: Option<ColorDetail>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FabricType {
    name: String,
    value: String,
}
