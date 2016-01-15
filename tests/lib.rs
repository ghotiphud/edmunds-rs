extern crate edmunds;

use edmunds::{ Edmunds, State };

use std::io::Read;

const EDMUNDS_API_KEY: &'static str = include_str!("api_key");

#[test]
#[ignore]
fn all_makes() {
    let api = Edmunds::new(EDMUNDS_API_KEY);

    let makes = api.all_makes(State::New).unwrap();

    //println!("{:?}", makes);
}

#[test]
#[ignore]
fn make_models() {
    let api = Edmunds::new(EDMUNDS_API_KEY);

    let models = api.models_by_make("bmw").unwrap();

    //println!("{:?}", models);
}