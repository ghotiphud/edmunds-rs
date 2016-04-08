#![allow(non_snake_case)]

pub use self::engine::*;
pub use self::car_option::*;
pub use self::color::*;
pub use self::transmission::*;
pub use self::equipment::*;

mod engine;
mod car_option;
mod color;
mod transmission;
mod equipment;

use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Make {
    pub id: u32,
    pub name: String,
    #[serde(rename="niceName")]
    pub nice_name: String,
    pub models: Vec<Arc<Model>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Model {
    pub id: String,
    pub name: String,
    #[serde(rename="niceName")]
    pub nice_name: String,
    pub years: Vec<Arc<Year>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Year {
    pub id: u32,
    pub year: u16,
    #[serde(default)]
    pub styles: Vec<Arc<Style>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Style {
    pub id: u32,
    pub name: String,
    pub trim: String,
    pub submodel: Submodel,

    pub drivenWheels: Option<String>,
    pub numOfDoors: Option<String>,
    pub engine: Option<Engine>,
    pub transmission: Option<Transmission>,
    #[serde(default)]
    pub options: Vec<CarOption>,
    #[serde(default)]
    pub colors: Vec<Color>,
    pub manufacturerCode: Option<String>,
    pub price: Option<Price>,
    pub categories: Option<Categories>,
    #[serde(default)]
    pub squishVins: Vec<String>,
    pub MPG: Option<MPG>,
    #[serde(default)]
    pub equipment: Vec<Equipment>,
}

impl Style {
    pub fn get_equipment(&self, name: &str) -> Option<&Equipment> {
        self.equipment.iter().find(|e| e.name == name)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MPG {
    pub highway: u32,
    pub city: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Categories {
    market: String,
    EPAClass: Option<String>,
    vehicleSize: String,
    crossover: Option<String>,
    primaryBodyType: String,
    vehicleStyle: String,
    vehicleType: String,
    manufacturerCabType: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Submodel {
    pub body: Option<String>,
    pub fuel: Option<String>,
    #[serde(rename="modelName")]
    pub model_name: String,
    #[serde(rename="niceName")]
    pub nice_name: String,
    pub tuner: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Price {
    pub baseMSRP: u32,
    pub baseInvoice: Option<u32>,
    pub deliveryCharges: Option<u32>,
    pub usedTmvRetail: Option<u32>,
    pub usedPrivateParty: Option<u32>,
    pub usedTradeIn: Option<u32>,
    pub estimateTmv: bool,
    pub tmvRecommendedRating: Option<u32>,
}

#[derive(Debug)]
pub enum State {
    New,
    Used,
    Future,
}

#[derive(Debug)]
pub enum View {
    Basic,
    Full,
}

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use serde_json;

    #[test]
    fn deserialize_make() {
        let input = include_str!("../../test-samples/make_basic.json");

        let expected = Make {
            id: 200002038,
            name: "Acura".to_string(),
            nice_name: "acura".to_string(),
            models: vec![],
        };

        let make: Make = serde_json::from_str(&input).unwrap();

        assert_eq!(expected.id, make.id);
        assert_eq!(expected.nice_name, make.nice_name);
        assert_eq!(expected.name, make.name);
        assert_eq!(9, make.models.len());
    }

    #[test]
    fn deserialize_model() {
        let input = include_str!("../../test-samples/model_basic.json");

        let expected = Model {
            id: "Acura_ILX".to_string(),
            name: "ILX".to_string(),
            nice_name: "ilx".to_string(),
            years: vec![],
        };

        let make: Model = serde_json::from_str(&input).unwrap();

        assert_eq!(expected.id, make.id);
        assert_eq!(expected.nice_name, make.nice_name);
        assert_eq!(expected.name, make.name);
        assert_eq!(3, make.years.len());
    }
}
