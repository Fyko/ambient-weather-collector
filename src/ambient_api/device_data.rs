use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiDeviceData {
	pub baromabsin: f64,
	pub baromrelin: f64,
	pub battout: i64,
	pub dailyrainin: i64,
	pub date: String,
	pub dateutc: i64,
	pub dew_point: f64,
	pub dew_pointin: f64,
	pub eventrainin: i64,
	pub feels_like: f64,
	pub feels_likein: f64,
	pub hourlyrainin: i64,
	pub humidity: i64,
	pub humidityin: i64,
	pub maxdailygust: f64,
	pub monthlyrainin: i64,
	pub solarradiation: f64,
	pub tempf: f64,
	pub tempinf: f64,
	pub totalrainin: i64,
	pub uv: i64,
	pub weeklyrainin: i64,
	pub winddir: i64,
	pub windgustmph: i64,
	pub windspeedmph: i64,
	pub yearlyrainin: i64,
}
