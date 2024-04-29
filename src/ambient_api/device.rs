use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiDevice {
	pub mac_address: String,
	pub last_data: LastData,
	pub info: Info,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Info {
	pub name: String,
	pub coords: InfoCoords,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfoCoords {
	pub coords: CoordsCoords,
	pub address: String,
	pub location: String,
	pub elevation: f64,
	pub address_components: Vec<AddressComponent>,
	pub geo: Geo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressComponent {
	pub long_name: String,
	pub short_name: String,
	pub types: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoordsCoords {
	pub lat: f64,
	pub lon: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Geo {
	#[serde(rename = "type")]
	pub geo_type: String,
	pub coordinates: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LastData {
	pub dateutc: f64,
	pub tempf: f64,
	pub humidity: f64,
	pub windspeedmph: f64,
	pub windgustmph: f64,
	pub maxdailygust: f64,
	pub winddir: f64,
	pub uv: f64,
	pub solarradiation: f64,
	pub hourlyrainin: f64,
	pub eventrainin: f64,
	pub dailyrainin: f64,
	pub weeklyrainin: f64,
	pub monthlyrainin: f64,
	pub yearlyrainin: f64,
	pub totalrainin: f64,
	pub battout: f64,
	pub tempinf: f64,
	pub humidityin: f64,
	pub baromrelin: f64,
	pub baromabsin: f64,
	pub feels_like: f64,
	pub dew_point: f64,
	pub feels_likein: f64,
	pub dew_pointin: f64,
	pub tz: String,
	pub date: String,
}
