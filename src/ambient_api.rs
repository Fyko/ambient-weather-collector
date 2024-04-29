pub mod device;
pub mod device_data;
pub mod ws;

use reqwest::Client;
use secrecy::{ExposeSecret, Secret};

use self::device_data::ApiDeviceData;
use crate::error::BoxDynError;

#[derive(Debug)]
pub struct AmbientApiClient {
	client: Client,
	api_key: Secret<String>,
	application_id: Secret<String>,
}

impl AmbientApiClient {
	pub fn new(api_key: String, application_id: String) -> Self {
		Self {
			client: Client::new(),
			api_key: Secret::new(api_key),
			application_id: Secret::new(application_id),
		}
	}

	pub async fn get_devices(&self) -> Result<Vec<device::ApiDevice>, BoxDynError> {
		let res = self
			.client
			.get("https://rt.ambientweather.net/v1/devices")
			.query(&[
				("apiKey", &self.api_key.expose_secret()),
				("applicationKey", &self.application_id.expose_secret()),
			])
			.send()
			.await?;

		let devices: Vec<device::ApiDevice> = res.json().await?;

		Ok(devices)
	}

	pub async fn get_device_data(&self, device_mac: &str) -> Result<Vec<ApiDeviceData>, BoxDynError> {
		let res = self
			.client
			.get(&format!("https://rt.ambientweather.net/v1/devices/{}", device_mac))
			.query(&[
				("apiKey", &self.api_key.expose_secret()),
				("applicationKey", &self.application_id.expose_secret()),
			])
			.send()
			.await?;

		let device_data: Vec<ApiDeviceData> = res.json().await?;

		Ok(device_data)
	}
}
