create extension if not exists timescaledb;
create table if not exists sensor_data (
	time timestamptz not null default now(),
	mac_address text not null,
	baromabsin double precision not null,
	baromrelin double precision not null,
	battout double precision not null,
	dailyrainin double precision not null,
	dew_point double precision not null,
	dew_pointin double precision not null,
	eventrainin double precision not null,
	feels_like double precision not null,
	feels_likein double precision not null,
	hourlyrainin double precision not null,
	humidity double precision not null,
	humidityin double precision not null,
	maxdailygust double precision not null,
	monthlyrainin double precision not null,
	solarradiation double precision not null,
	tempf double precision not null,
	tempinf double precision not null,
	totalrainin double precision not null,
	uv double precision not null,
	weeklyrainin double precision not null,
	winddir double precision not null,
	windgustmph double precision not null,
	windspeedmph double precision not null,
	yearlyrainin double precision not null
);
create unique index if not exists idx_sensor_data_time_mac_address_idx on sensor_data (mac_address, time);
select create_hypertable('sensor_data', by_range('time'));
select add_dimension('sensor_data', by_hash('mac_address', 4));
