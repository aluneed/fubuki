use anyhow::{anyhow, Result};
use chrono::{DateTime, Local, NaiveDateTime, Utc};

pub mod cipher;
pub mod net;
pub mod routing_table;
pub mod allocator;

pub fn utc_to_str(t: i64) -> Result<String> {
    let utc: DateTime<Utc> = DateTime::from_utc(
        NaiveDateTime::from_timestamp_opt(t, 0).ok_or_else(|| anyhow!("can't convert timestamp"))?,
        Utc,
    );

    let local_time: DateTime<Local> = DateTime::from(utc);
    let str = local_time.format("%Y-%m-%d %H:%M:%S").to_string();

    Ok(str)
}