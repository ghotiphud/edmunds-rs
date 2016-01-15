use super::Price;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Color {
    category: String,
    options: Vec<ColorOption>
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
    fabricTypes: Vec<FabricType>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ColorDetail {
    r: u8,
    g: u8,
    b: u8,
    hex: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ColorChips {
    primary: ColorDetail,
    secondary: Option<ColorDetail>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FabricType {
    name: String,
    value: String,
}