#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Engine {
    pub id: String,
    pub name: String,
    pub equipmentType: String,
    pub compressionRatio: Option<f32>,
    pub cylinder: Option<u8>,
    pub size: Option<f32>,
    pub displacement: Option<u16>,
    pub configuration: Option<String>,
    pub fuelType: Option<String>,
    pub horsepower: Option<u16>,
    pub torque: Option<u16>,
    pub totalValves: Option<u8>,
    pub manufacturerEngineCode: Option<String>,
    #[serde(rename="type")]
    pub engine_type: String,
    pub code: String,
    pub compressorType: String,
    pub rpm: Option<RPM>,
    pub valve: Option<Valve>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RPM {
    pub horsepower: Option<u32>,
    pub torque: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Valve {
    pub timing: Option<String>,
    pub gear: Option<String>,
}
