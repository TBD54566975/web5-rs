use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serializer};
use std::time::SystemTime;

pub(crate) fn serialize_system_time<S>(
    time: &SystemTime,
    serializer: S,
) -> std::result::Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let datetime: chrono::DateTime<Utc> = (*time).into();
    let s = datetime.to_rfc3339();
    serializer.serialize_str(&s)
}

pub(crate) fn deserialize_system_time<'de, D>(
    deserializer: D,
) -> std::result::Result<SystemTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let datetime = chrono::DateTime::parse_from_rfc3339(&s).map_err(serde::de::Error::custom)?;
    Ok(datetime.with_timezone(&Utc).into())
}

pub(crate) fn serialize_optional_system_time<S>(
    time: &Option<SystemTime>,
    serializer: S,
) -> std::result::Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match time {
        Some(time) => serialize_system_time(time, serializer),
        None => serializer.serialize_none(),
    }
}

pub(crate) fn deserialize_optional_system_time<'de, D>(
    deserializer: D,
) -> std::result::Result<Option<SystemTime>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    match opt {
        Some(s) => {
            let datetime = DateTime::parse_from_rfc3339(&s).map_err(serde::de::Error::custom)?;
            Ok(Some(datetime.with_timezone(&Utc).into()))
        }
        None => Ok(None),
    }
}
