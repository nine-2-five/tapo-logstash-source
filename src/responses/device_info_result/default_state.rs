use serde::Deserialize;
use serde::Serialize;

/// The default state of a device to be used when internet connectivity is lost after a power cut.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum DefaultState<T> {
    Custom(T),
    LastStates(T),
}
