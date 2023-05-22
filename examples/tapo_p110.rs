/// P110 Example
use std::{env, thread, time as stdtime};
use chrono::{Local, Datelike};
use log::{info, LevelFilter};
use tapo::{requests::EnergyDataInterval, ApiClient, P110};
use time::{macros::{date, datetime}, Date, Duration as TimeDuration};
use time::{Month as TimeMonth, OffsetDateTime};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let log_level = env::var("RUST_LOG")
        .unwrap_or_else(|_| "info".to_string())
        .parse()
        .unwrap_or(LevelFilter::Info);

    pretty_env_logger::formatted_timed_builder()
        .filter(Some("tapo"), log_level)
        .init();

    let ip_address = env::var("IP_ADDRESS")?;
    let tapo_username = env::var("TAPO_USERNAME")?;
    let tapo_password = env::var("TAPO_PASSWORD")?;
    let timeout = env::var("POLL_INTERVAL_SECONDS")?;

    let device = ApiClient::<P110>::new(ip_address, tapo_username, tapo_password, true).await?;

    let delay = stdtime::Duration::from_secs(timeout.parse::<u64>().unwrap());    

    // info!("Turning device on...");
    // device.on().await?;

    // info!("Waiting 2 seconds...");
    // thread::sleep(Duration::from_secs(2));

    // info!("Turning device off...");
    // device.off().await?;    

    loop{
        let today = Local::now();        
        let first_of_quarter = get_first_day_of_quarter();
        let first_of_year = Date::from_calendar_date(
            today.year() as i32,
            TimeMonth::January,
            1,
        )
        .unwrap();
        let seven_days_ago = get_7_days_ago();
      
        let device_info = device.get_device_info().await?;
        let device_info_json = serde_json::to_string(&device_info).unwrap();

        info!("Device info: {}", device_info_json);

        let device_usage = device.get_device_usage().await?;
        let device_usage_json = serde_json::to_string(&device_usage).unwrap();

        info!("Device usage: {}", device_usage_json);    

        let energy_usage = device.get_energy_usage().await?;
        let energy_usage_json = serde_json::to_string(&energy_usage).unwrap();

        info!("Energy usage: {}", energy_usage_json);

        let energy_data_hourly = device
            .get_energy_data(EnergyDataInterval::Hourly {
                start_datetime: seven_days_ago,
                end_datetime: OffsetDateTime::now_utc(),
            })
            .await?;
        let energy_data_hourly_json: String = serde_json::to_string(&energy_data_hourly).unwrap();

        info!("Energy data (hourly): {}", energy_data_hourly_json);

        let energy_data_daily = device
            .get_energy_data(EnergyDataInterval::Daily {
                start_date: first_of_quarter,
            })
            .await?;
        let energy_data_daily_json: String = serde_json::to_string(&energy_data_daily).unwrap();

        info!("Energy data (daily): {}", energy_data_daily_json);        
        
        let energy_data_monthly = device
            .get_energy_data(EnergyDataInterval::Monthly {
                start_date: first_of_year,
            })
            .await?;
        let energy_data_monthly_json: String = serde_json::to_string(&energy_data_monthly).unwrap();

        info!("Energy data (monthly): {}", energy_data_monthly_json);
        thread::sleep(delay);
    }

    Ok(())
}

fn create_time_month(month_num: u32) -> Result<TimeMonth, String> {
    match month_num {
        1 => Ok(TimeMonth::January),
        2 => Ok(TimeMonth::February),
        3 => Ok(TimeMonth::March),
        4 => Ok(TimeMonth::April),
        5 => Ok(TimeMonth::May),
        6 => Ok(TimeMonth::June),
        7 => Ok(TimeMonth::July),
        8 => Ok(TimeMonth::August),
        9 => Ok(TimeMonth::September),
        10 => Ok(TimeMonth::October),
        11 => Ok(TimeMonth::November),
        12 => Ok(TimeMonth::December),
        _ => Err(String::from("Invalid month number")),
    }
}

fn get_first_day_of_quarter() -> time::Date {
    let local_date = Local::now();
    let quarter_month = ((local_date.month() - 1) / 3) * 3 + 1;

    Date::from_calendar_date(local_date.year(), create_time_month(quarter_month).unwrap(), 1).unwrap()
}

fn get_7_days_ago() -> time::OffsetDateTime {
    // Subtract 7 days to get the time 7 days ago
    return OffsetDateTime::now_utc() - TimeDuration::days(7);
}