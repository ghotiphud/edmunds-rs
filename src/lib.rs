#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(serde_macros)]

extern crate hyper;
extern crate serde;
extern crate serde_json;

mod models;
pub use models::*;

use std::io::Read;

use hyper::{ Client, Result };
use hyper::client::{ Request, Response };
use hyper::header::Connection;


const BASE_URL: &'static str = "https://api.edmunds.com";

pub struct Edmunds {
    api_key: &'static str,
    client: Client,
}

impl Edmunds {
    pub fn new(api_key: &'static str) -> Self {
        Edmunds::with_client(api_key, Client::new())
    }

    pub fn call(&self, endpoint: &str, parameters: &str) -> Result<Response> {
        let url = BASE_URL.to_string() + endpoint + "?fmt=json&api_key=" + &self.api_key + "&" + parameters;

        println!("{:?}", &url);

        let request = self.client.get(&url)
            .header(Connection::close());

        let response = try!(request.send());

        Ok(response)
    }

    pub fn all_makes(&self, state: State) -> Result<Vec<Make>> {
        #[derive(Deserialize)]
        struct AllMakes {
            makes: Vec<Make>,
            #[serde(rename="makesCount")]
            makes_count: u32,
        }

        let parameters = "state=".to_string() + &format!("{:?}", state).to_lowercase();
        let mut res = try!(self.call("/api/vehicle/v2/makes", &parameters));

        let all_makes: AllMakes = serde_json::from_reader(res).expect("deserialization failed");

        Ok(all_makes.makes)
    }

    pub fn models_by_make(&self, make_nice_name: &str) -> Result<Vec<Model>>  {
        #[derive(Deserialize)]
        struct AllModels {
            models: Vec<Model>,
            #[serde(rename="modelsCount")]
            models_count: u32,
        }

        let parameters = "state=new";
        let url = format!("/api/vehicle/v2/{make}/models", make = make_nice_name);
        let mut res = try!(self.call(&url, &parameters));

        let all_models: AllModels = serde_json::from_reader(res).expect("deserialization failed");

        Ok(all_models.models)
    }

    pub fn styles_by_make_model_year(&self, make_nice_name: &str, model_nice_name: &str, year: u16) -> Result<Vec<Style>> {
        #[derive(Deserialize)]
        struct AllStyles {
            styles: Vec<Style>,
            #[serde(rename="stylesCount")]
            styles_count: u32,
        }

        let parameters = "state=new&view=full";
        let url = format!("/api/vehicle/v2/{make}/{model}/{year}/styles", 
            make = make_nice_name,
            model = model_nice_name,
            year = year);

        let mut res = try!(self.call(&url, &parameters));

        let all_styles: AllStyles = serde_json::from_reader(res).expect("deserialization failed");

        Ok(all_styles.styles)
    }

    pub fn equipment_by_style(&self, style_id: u32) -> Result<Vec<Style>> {
        unimplemented!()
    }

    // private fns
    /// For testing pass a mock client
    fn with_client(api_key: &'static str, client: Client) -> Self {
        Edmunds { api_key: api_key, client: client }
    }
}


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
