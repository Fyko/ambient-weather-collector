use std::env;
use std::sync::Arc;

use ambient_api::ws::{WsDeviceData, WsSubscribedPayload};
use error::BoxDynError;
use futures_util::FutureExt;
use rust_socketio::asynchronous::ClientBuilder;
use rust_socketio::{Payload, TransportType};
use serde_json::json;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt, EnvFilter, Registry};

pub mod ambient_api;
pub mod error;

#[tokio::main]
async fn main() -> Result<(), BoxDynError> {
	Registry::default()
		.with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "ambient_weather_collector=debug,info".into()))
		.with(fmt::layer())
		.init();

	let api_key = Arc::new(env::var("AMBIENT_WEATHER_API_KEY").expect("AMBIENT_WEATHER_API_KEY must be set"));
	let application_key =
		env::var("AMBIENT_WEATHER_APPLICATION_KEY").expect("AMBIENT_WEATHER_APPLICATION_KEY must be set");

	// let client = AmbientApiClient::new(api_key.clone(), application_key.clone());
	// let devices = client.get_devices().await?;
	// tracing::info!("devices: {devices:#?}");
	// tokio::time::sleep(Duration::from_millis(1_500)).await;

	// let mac_address = devices[0].mac_address.clone();
	// let data = client.get_device_data(&mac_address).await?;
	// tracing::info!("data: {data:#?}");

	let socket = ClientBuilder::new(format!(
		"https://rt2.ambientweather.net/?api=1&applicationKey={application_key}"
	))
	.transport_type(TransportType::Websocket)
	.on("subscribed", |data, _| {
		async move {
			let payload = match data {
				Payload::Text(value) => {
					let first = &value[0];
					serde_json::from_value::<WsSubscribedPayload>(first.clone()) // ouch
				}
				Payload::Binary(bytes) => serde_json::from_slice::<WsSubscribedPayload>(&bytes),
				_ => panic!("unexpected payload"),
			}
			.expect("failed to parse payload");

			let macs = payload
				.devices
				.iter()
				.map(|d| d.mac_address.as_ref())
				.collect::<Vec<_>>();

			tracing::info!("successfully subscribed to {}", macs.join(", "));
		}
		.boxed()
	})
	.on("data", |data, _| {
		async move {
			tracing::debug!("received data: {data:#?}");
			let payload = match data {
				Payload::Text(value) => {
					let first = &value[0];
					serde_json::from_value::<WsDeviceData>(first.clone()) // ouch
				}
				Payload::Binary(bytes) => serde_json::from_slice::<WsDeviceData>(&bytes),
				_ => panic!("unexpected payload"),
			}
			.expect("failed to parse payload");

			tracing::info!("received data from {}", payload.mac_address);
		}
		.boxed()
	})
	.on("message", |data, _| {
		async move { tracing::info!("Message: {data:#?}") }.boxed()
	})
	.on("error", |err, _| {
		async move { tracing::info!("Error: {err:#?}") }.boxed()
	})
	.on("open", {
		|_, client| {
			async move {
				let api_key = env::var("AMBIENT_WEATHER_API_KEY").expect("AMBIENT_WEATHER_API_KEY must be set");

				let subscribe_message = json!({
					"apiKeys": [api_key],
				});
				client
					.emit("subscribe", subscribe_message)
					.await
					.expect("server unreachable");

				tracing::info!("sent `subscribe` packet to ambient weather");
			}
			.boxed()
		}
	})
	.connect()
	.await
	.expect("Connection failed");
	tracing::info!("Connected to Ambient Weather socket.io server");

	shutdown_signal().await;
	socket.disconnect().await?;

	Ok(())
}

async fn shutdown_signal() {
	let ctrl_c = async {
		tokio::signal::ctrl_c().await.expect("failed to install Ctrl+C handler");
	};

	#[cfg(unix)]
	let terminate = async {
		tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
			.expect("failed to install signal handler")
			.recv()
			.await;
	};

	#[cfg(not(unix))]
	let terminate = std::future::pending::<()>();

	tokio::select! {
		_ = ctrl_c => {},
		_ = terminate => {},
	}

	tracing::info!("signal received, starting graceful shutdown");
}
