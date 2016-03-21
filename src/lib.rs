#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(serde_macros)]

extern crate hyper;
extern crate serde;
extern crate serde_json;

mod models;
pub use models::*;

use hyper::{Client, Result};
use hyper::client::Response;
use hyper::header::Connection;

const BASE_URL: &'static str = "https://api.edmunds.com";

// const ALL_MAKES: &'static str = "/api/vehicle/v2/makes";
// const MAKE_DETAIL: &'static str = "/api/vehicle/v2/{make}";

// const MODELS_BY_MAKE: &'static str = "/api/vehicle/v2/{make}/models";
// const MODEL_BY_MAKE_MODEL: &'static str = "/api/vehicle/v2/{make}/{model}";

// const YEAR_BY_MAKE_MODEL: &'static str = "/api/vehicle/v2/{make}/{model}/years";
// const YEAR_BY_MAKE_MODEL_YEAR: &'static str = "/api/vehicle/v2/{make}/{model}/{year}";

// const STYLE_BY_STYLEID: &'static str = "/api/vehicle/v2/styles/{style_id}";
// const STYLES_BY_MAKE_MODEL_YEAR: &'static str = "/api/vehicle/v2/{make}/{model}/{year}/styles";
// const STYLE_BY_SQUISHVIN: &'static str = "/api/vehicle/v2/squishvins/{squishvin}";

// const OPTIONS_BY_STYLEID: &'static str = "/api/vehicle/v2/styles/{style_id}/options";
// const OPTION_BY_OPTIONID: &'static str = "/api/vehicle/v2/options/{option_id}";
// const COLORS_BY_STYLEID: &'static str = "/api/vehicle/v2/styles/{style_id}/colors";
// const COLOR_BY_COLORID: &'static str = "/api/vehicle/v2/colors/{color_id}";

// const ENGINES_BY_STYLEID: &'static str = "/api/vehicle/v2/styles/{style_id}/engines";
// const ENGINE_BY_ENGINEID: &'static str = "/api/vehicle/v2/engines/{engine_id}";
// const TRANSMISSIONS_BY_STYLEID: &'static str = "/api/vehicle/v2/styles/{style_id}/transmissions";
// const TRANSMISSION_BY_TRANSMISSIONID: &'static str = "/api/vehicle/v2/transmissions/{transmission_id}";

// const EQUIPMENTS_BY_STYLEID: &'static str = "/api/vehicle/v2/styles/{style_id}/equipment";
// const EQUIPMENT_BY_EQUIPMENTID: &'static str = "/api/vehicle/v2/equipment/{equipment_id}";


pub struct Edmunds {
    api_key: &'static str,
    client: Client,
}

impl Edmunds {
    pub fn new(api_key: &'static str) -> Self {
        Edmunds::with_client(api_key, Client::new())
    }

    pub fn call_send(&self, endpoint: &str, parameters: &str) -> Result<Response> {
        let url = BASE_URL.to_string() + endpoint + "?fmt=json&api_key=" 
                    + &self.api_key + "&" + parameters;

        println!("{:?}", &url);

        let request = self.client
                          .get(&url)
                          .header(Connection::close());

        request.send()
    }

    pub fn call<T>(&self, endpoint: &str, parameters: &str) -> Result<T> 
        where T: serde::de::Deserialize 
    {
        let response = try!(self.call_send(endpoint, parameters));

        let result = serde_json::from_reader(response)
                                .expect("deserialization failed");

        Ok(result)
    }

    pub fn all_makes(&self, state: State) -> Result<Vec<Make>> {
        let parameters = "state=".to_string() + &format!("{:?}", state).to_lowercase();

        let all_makes: AllMakes = try!(self.call("/api/vehicle/v2/makes", &parameters));

        Ok(all_makes.makes)
    }

    pub fn models_by_make(&self, make_nicename: &str) -> Result<Vec<Model>> {
        let parameters = "state=new";
        let url = format!("/api/vehicle/v2/{make}/models", make = make_nicename);

        let all_models: AllModels = try!(self.call(&url, &parameters));

        Ok(all_models.models)
    }

    pub fn styles_by_make_model_year(&self,
                                     make_nicename: &str,
                                     model_nicename: &str,
                                     year: u16)
                                     -> Result<Vec<Style>> {
        let parameters = "state=new&view=full";
        let url = format!("/api/vehicle/v2/{make}/{model}/{year}/styles",
                          make = make_nicename,
                          model = model_nicename,
                          year = year);

        let all_styles: AllStyles = try!(self.call(&url, &parameters));

        Ok(all_styles.styles)
    }

    pub fn equipment_by_styleid(&self, style_id: u32) -> Result<Vec<Equipment>> {
        let url = format!("/api/vehicle/v2/styles/{style_id}/equipment",
                          style_id = style_id);

        let all_equipment: AllEquipment = try!(self.call(&url, &""));

        Ok(all_equipment.equipment)
    }

    // private fns
    /// For testing pass a mock client
    fn with_client(api_key: &'static str, client: Client) -> Self {
        Edmunds {
            api_key: api_key,
            client: client,
        }
    }
}

// Helper structs for deserialization
#[derive(Deserialize)]
struct AllMakes {
    makes: Vec<Make>
}        

#[derive(Deserialize)]
struct AllModels {
    models: Vec<Model>
}

#[derive(Deserialize)]
struct AllStyles {
    styles: Vec<Style>
}

#[derive(Deserialize)]
struct AllEquipment {
    equipment: Vec<Equipment>
}



// Testing
#[cfg(test)]
#[allow(dead_code)]
#[macro_use]
mod hyper_mock;

#[cfg(test)]
#[macro_use]
extern crate log;

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use super::hyper;

    #[test]
    fn all_makes() {
        mock_connector!(MockAllMakes {
            "https://api.edmunds.com" =>
                include_str!("../test-samples/http/all_makes_basic.http")
        });

        let client = hyper::Client::with_connector(MockAllMakes);

        let ed = Edmunds::with_client("api_key", client);

        let makes = ed.all_makes(State::New);

        println!("{:?}", makes);
    }

    #[test]
    fn all_bmw_models() {
        mock_connector!(MockAllBmwModels {
            "https://api.edmunds.com" =>
                include_str!("../test-samples/http/all_bmw_models_basic.http")
        });

        let client = hyper::Client::with_connector(MockAllBmwModels);

        let ed = Edmunds::with_client("api_key", client);

        let models = ed.models_by_make("bmw").unwrap();

        assert_eq!(30, models.len());

        println!("{:?}", models);
    }

    #[test]
    fn alfa_romeo_4c_2015_styles_full() {
        mock_connector!(MockStylesFull {
            "https://api.edmunds.com" =>
                include_str!("../test-samples/http/alfa_romeo_4c_2015_styles_full.http")
        });

        let client = hyper::Client::with_connector(MockStylesFull);

        let ed = Edmunds::with_client("api_key", client);

        let styles = ed.styles_by_make_model_year("alfa-romeo", "4c", 2015).unwrap();

        assert_eq!(3, styles.len());

        println!("{:?}", styles);
    }

    #[test]
    fn acura_ilx_2014_styles_full() {
        mock_connector!(MockStylesFull {
            "https://api.edmunds.com" =>
                include_str!("../test-samples/http/acura_ilx_2014_styles_full.http")
        });

        let client = hyper::Client::with_connector(MockStylesFull);

        let ed = Edmunds::with_client("api_key", client);

        let styles = ed.styles_by_make_model_year("acura", "ilx", 2014).unwrap();

        assert_eq!(4, styles.len());

        println!("{:?}", styles);
    }

    #[test]
    fn acura_ilx_2016_styles_full() {
        mock_connector!(MockStylesFull {
            "https://api.edmunds.com" =>
                include_str!("../test-samples/http/acura_ilx_2016_styles_full.http")
        });

        let client = hyper::Client::with_connector(MockStylesFull);

        let ed = Edmunds::with_client("api_key", client);

        let styles = ed.styles_by_make_model_year("acura", "ilx", 2016).unwrap();

        assert_eq!(6, styles.len());

        println!("{:?}", styles);
    }

    #[test]
    fn bmw_i3_2014_styles_full() {
        mock_connector!(MockStylesFull {
            "https://api.edmunds.com" =>
                include_str!("../test-samples/http/bmw_i3_2014_styles_full.http")
        });

        let client = hyper::Client::with_connector(MockStylesFull);

        let ed = Edmunds::with_client("api_key", client);

        let styles = ed.styles_by_make_model_year("bmw", "i3", 2014).unwrap();

        assert_eq!(2, styles.len());

        println!("{:?}", styles);
    }
}
