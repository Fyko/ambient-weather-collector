use std::sync::Arc;
use std::time::Duration;

use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::{ConnectOptions, PgPool};

use crate::ambient_api::ws::WsDeviceData;
use crate::error::BoxDynError;

pub async fn create_db(database_url: &String) -> Arc<PgPool> {
	let pool = PgPoolOptions::new()
		.max_connections(100)
		.min_connections(5)
		.acquire_timeout(Duration::from_secs(8))
		.idle_timeout(Duration::from_secs(8))
		.max_lifetime(Duration::from_secs(60));

	tracing::info!("connecting to database {}", database_url);

	let mut opts: PgConnectOptions = database_url.parse().expect("failed to parse database url");
	opts = opts.log_statements(tracing::log::LevelFilter::Trace);

	let db = pool.connect_with(opts).await.expect("Failed to connect to database");

	Arc::new(db)
}

pub async fn insert_ws_data(db: &PgPool, payload: &WsDeviceData) -> Result<(), BoxDynError> {
	sqlx::query!(r#"
				insert into sensor_data(
					mac_address,
					baromabsin,
					baromrelin,
					battout,
					dailyrainin,
					dew_point,
					dew_pointin,
					eventrainin,
					feels_like,
					feels_likein,
					hourlyrainin,
					humidity,
					humidityin,
					maxdailygust,
					monthlyrainin,
					solarradiation,
					tempf,
					tempinf,
					totalrainin,
					uv,
					weeklyrainin,
					winddir,
					windgustmph,
					windspeedmph,
					yearlyrainin
				) values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25)
				returning *;"#,
				payload.mac_address,
				payload.baromabsin,
				payload.baromrelin,
				payload.battout,
				payload.dailyrainin,
				payload.dew_point,
				payload.dew_pointin,
				payload.eventrainin,
				payload.feels_like,
				payload.feels_likein,
				payload.hourlyrainin,
				payload.humidity,
				payload.humidityin,
				payload.maxdailygust,
				payload.monthlyrainin,
				payload.solarradiation,
				payload.tempf,
				payload.tempinf,
				payload.totalrainin,
				payload.uv,
				payload.weeklyrainin,
				payload.winddir,
				payload.windgustmph,
				payload.windspeedmph,
				payload.yearlyrainin
			).fetch_one(db).await.expect("failed to insert data");

	Ok(())
}
