use serde_json;

pub use self::engine::*;
pub use self::car_option::*;
pub use self::color::*;
pub use self::transmission::*;

mod engine;
mod car_option;
mod color;
mod transmission;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Make {
    pub id: u32,
    pub name: String,
    #[serde(rename="niceName")]
    pub nice_name: String,
    pub models: Vec<Model>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Model {
    pub id: String,
    pub name: String,
    #[serde(rename="niceName")]
    pub nice_name: String,
    pub years: Vec<Year>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Year {
    pub id: u32,
    pub year: u16,
    pub styles: Vec<Style>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Style {
    pub id: u32,
    pub name: String,
    pub trim: String,
    pub submodel: Submodel,

    pub drivenWheels: Option<String>,
    pub numOfDoors: Option<String>,
    pub engine: Engine,
    pub transmission: Transmission,
    pub options: Vec<CarOption>,
    pub colors: Vec<Color>,
    pub manufacturerCode: Option<String>,
    pub price: Price,
    pub categories: Categories,
    pub squishVins: Option<Vec<String>>,
    pub MPG: Option<MPG>,


    // Fields to work around serde choking on unknown fields (private because we don't want them used)

    #[serde(skip_serializing)]
    make: Option<Make>,
    #[serde(skip_serializing)]
    model: Option<Model>,
    #[serde(skip_serializing)]
    year: Option<Year>,
    #[serde(skip_serializing)]
    states: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MPG {
    highway: u32,
    city: u32,
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
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Submodel {
    pub body: String,
    pub fuel: Option<String>,
    #[serde(rename="modelName")]
    pub model_name: String,
    #[serde(rename="niceName")]
    pub nice_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Price {
    baseMSRP: u32,
    baseInvoice: Option<u32>,
    deliveryCharges: Option<u32>,
    usedTmvRetail: Option<u32>,
    usedPrivateParty: Option<u32>,
    usedTradeIn: Option<u32>,
    estimateTmv: bool,
    tmvRecommendedRating: Option<u32>,
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
    use ::serde_json;

    #[test]
    fn deserialize_make() {
        let input = include_str!("../../test-samples/make_basic.json");

        let expected = Make{ id: 200002038, name: "Acura".to_string(), nice_name: "acura".to_string(), models: vec![] };

        let make: Make = serde_json::from_str(&input).unwrap();

        assert_eq!(expected.id, make.id);
        assert_eq!(expected.nice_name, make.nice_name);
        assert_eq!(expected.name, make.name);
        assert_eq!(9, make.models.len());
    }

    #[test]
    fn deserialize_model() {
        let input = include_str!("../../test-samples/model_basic.json");

        let expected = Model{ id: "Acura_ILX".to_string(), name: "ILX".to_string(), nice_name: "ilx".to_string(), years: vec![] };

        let make: Model = serde_json::from_str(&input).unwrap();

        assert_eq!(expected.id, make.id);
        assert_eq!(expected.nice_name, make.nice_name);
        assert_eq!(expected.name, make.name);
        assert_eq!(3, make.years.len());
    }
}
