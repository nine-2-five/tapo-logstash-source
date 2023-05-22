use serde::Deserialize;
use serde::Serialize;
use time::OffsetDateTime;

use crate::responses::TapoResponseExt;
// use crate::tapo_date_format;

/// Contains local time, current power and the energy usage and runtime over multiple periods of time.
#[derive(Debug, Deserialize, Serialize)]
pub struct EnergyUsageResult {
    /// Local time, with the UTC offset assumed from the machine this call is made on
    #[serde(deserialize_with = "crate::tapo_date_format::deserialize", serialize_with = "crate::tapo_date_format::serialize")]
    pub local_time: OffsetDateTime,
    /// Current power in milliwatts (mW)
    pub current_power: u64,
    /// Today runtime in minutes
    pub today_runtime: u64,
    /// Today energy usage in watts (W)
    pub today_energy: u64,
    /// Past 30 days runtime in minutes
    pub month_runtime: u64,
    /// Past 30 days energy usage in watts (W)
    pub month_energy: u64,
    /// Hourly energy usage for the past 24 hours in watts (W)
    #[deprecated(
        since = "0.4.0",
        note = "P110 firmware v1.1.6 no longer returns this field. Use `ApiClient::<P110>::get_energy_data` instead."
    )]
    pub past24h: Option<Vec<u64>>,
    /// Hourly energy usage by day for the past 7 days in watts (W)
    #[deprecated(
        since = "0.4.0",
        note = "P110 firmware v1.1.6 no longer returns this field. Use `ApiClient::<P110>::get_energy_data` instead."
    )]
    pub past7d: Option<Vec<Vec<u64>>>,
    /// Daily energy usage for the past 30 days in watts (W)
    #[deprecated(
        since = "0.4.0",
        note = "P110 firmware v1.1.6 no longer returns this field. Use `ApiClient::<P110>::get_energy_data` instead."
    )]
    pub past30d: Option<Vec<u64>>,
    /// Monthly energy usage for the past year in watts (W)
    #[deprecated(
        since = "0.4.0",
        note = "P110 firmware v1.1.6 no longer returns this field. Use `ApiClient::<P110>::get_energy_data` instead."
    )]
    pub past1y: Option<Vec<u64>>,
}
impl TapoResponseExt for EnergyUsageResult {}
