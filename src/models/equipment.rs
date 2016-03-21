use std::str::FromStr;
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Equipment {
    pub id: String,
    pub name: String,
    pub equipmentType: String,
    pub availability: String,
    #[serde(default)]
    pub attributes: Vec<Attribute>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Attribute {
    pub name: String,
    pub value: String,
}

impl Equipment {
    pub fn get_attr<T>(&self, name: &str) -> Option<T> 
        where T: FromStr, T::Err: Debug 
    {
        self.attributes.iter()
                        .find(|a| a.name == name)
                        .map(|a| a.value.parse::<T>().expect("parse error"))
    }
}