use serde::{Deserialize, Serialize};

use super::device::{Info, LastData};

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct WsDevice {
	#[serde(rename = "apiKey")]
	pub api_key: String,
	pub info: Info,
	#[serde(rename = "lastData")]
	pub last_data: LastData,
	#[serde(rename = "macAddress")]
	pub mac_address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WsSubscribedPayload {
	pub devices: Vec<WsDevice>,
	pub method: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsDeviceData {
	/// Absolute barometric pressure in inches of mercury.
	pub baromabsin: f64,
	/// Relative barometric pressure in inches of mercury.
	pub baromrelin: f64,
	/// Outdoor battery status: 1 = OK, 0 = Low. For Meteobridge users: 1 = Low,
	/// 0 = OK.
	pub battout: i64,
	/// Daily rainfall in inches.
	pub dailyrainin: i64,
	/// Human-readable date (converted on server from `dateutc`).
	pub date: String,
	/// Date and time in milliseconds from 01-01-1970, rounded down to the
	/// nearest minute.
	pub dateutc: i64,
	/// Dew point in degrees Fahrenheit (calculated on server).
	pub dew_point: f64,
	/// Indoor dew point in degrees Fahrenheit.
	pub dew_pointin: f64,
	/// Rainfall for the current event in inches.
	pub eventrainin: i64,
	/// Feels like temperature in degrees Fahrenheit (calculated on server,
	/// applies formula based on temperature).
	pub feels_like: f64,
	/// Indoor feels like temperature in degrees Fahrenheit.
	pub feels_likein: f64,
	/// Hourly rainfall in inches.
	pub hourlyrainin: i64,
	/// Outdoor humidity percentage.
	pub humidity: i64,
	/// Indoor humidity percentage.
	pub humidityin: i64,
	/// MAC address of the device.
	pub mac_address: String,
	/// Maximum wind gust speed in the last day in miles per hour.
	pub maxdailygust: f64,
	/// Monthly rainfall in inches.
	pub monthlyrainin: i64,
	/// Solar radiation in watts per square meter.
	pub solarradiation: f64,
	/// Outdoor temperature in degrees Fahrenheit.
	pub tempf: f64,
	/// Indoor temperature in degrees Fahrenheit.
	pub tempinf: f64,
	/// Total rainfall in inches since the last factory reset.
	pub totalrainin: i64,
	/// Ultra-Violet radiation index (integer, except on model WS-8478).
	pub uv: i64,
	/// Weekly rainfall in inches.
	pub weeklyrainin: i64,
	/// Instantaneous wind direction in degrees (0-360).
	pub winddir: i64,
	/// Maximum wind speed in the last 10 minutes in miles per hour.
	pub windgustmph: i64,
	/// Instantaneous wind speed in miles per hour.
	pub windspeedmph: i64,
	/// Yearly rainfall in inches.
	pub yearlyrainin: i64,
}
