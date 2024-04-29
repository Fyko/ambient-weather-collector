use std::env;

use ambient_api::ws::{WsDeviceData, WsSubscribedPayload};
use error::BoxDynError;
use futures_util::FutureExt;
use rust_socketio::asynchronous::{Client as SocketIoClient, ClientBuilder};
use rust_socketio::{Payload, TransportType};
use serde_json::json;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt, EnvFilter, Registry};

pub mod ambient_api;
pub mod error;
pub mod timescale;

async fn handle_subscribed(data: Payload, _client: SocketIoClient) {
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

#[tokio::main]
async fn main() -> Result<(), BoxDynError> {
	Registry::default()
		.with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "ambient_weather_collector=debug,info".into()))
		.with(fmt::layer())
		.init();

	let application_key =
		env::var("AMBIENT_WEATHER_APPLICATION_KEY").expect("AMBIENT_WEATHER_APPLICATION_KEY must be set");
	let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

	let db = timescale::create_db(&database_url).await;

	let socket = ClientBuilder::new(format!(
		"https://rt2.ambientweather.net/?api=1&applicationKey={application_key}"
	))
	.transport_type(TransportType::Websocket)
	.on("subscribed", |data, socket| {
		async move { handle_subscribed(data, socket).await }.boxed()
	})
	.on("data", move |data, _| {
		let db_clone = db.clone();
		async move {
			let payload = match data {
				Payload::Text(value) => {
					let first = &value[0];
					serde_json::from_value::<WsDeviceData>(first.clone()) // ouch
				}
				Payload::Binary(bytes) => serde_json::from_slice::<WsDeviceData>(&bytes),
				_ => panic!("unexpected payload"),
			}
			.expect("failed to parse payload");

			timescale::insert_ws_data(&db_clone, &payload)
				.await
				.expect("failed to insert data");
			tracing::info!("received and inserted data from {}", payload.mac_address);
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
			let api_key = env::var("AMBIENT_WEATHER_API_KEY").expect("AMBIENT_WEATHER_API_KEY must be set");

			async move {
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
