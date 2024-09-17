use chrono::{DateTime, Utc};
use serde::{de::Error as DeError, ser::Error as SerError};
use serde::{Deserialize, Deserializer, Serializer};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub(crate) fn serialize_rfc3339<S>(
    time: &SystemTime,
    serializer: S,
) -> std::result::Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let datetime: chrono::DateTime<Utc> = (*time).into();
    let s = datetime.to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
    serializer.serialize_str(&s)
}

pub(crate) fn deserialize_rfc3339<'de, D>(
    deserializer: D,
) -> std::result::Result<SystemTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let datetime = chrono::DateTime::parse_from_rfc3339(&s).map_err(serde::de::Error::custom)?;
    Ok(datetime.with_timezone(&Utc).into())
}

pub(crate) fn serialize_optional_rfc3339<S>(
    time: &Option<SystemTime>,
    serializer: S,
) -> std::result::Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match time {
        Some(time) => serialize_rfc3339(time, serializer),
        None => serializer.serialize_none(),
    }
}

pub(crate) fn deserialize_optional_rfc3339<'de, D>(
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

pub(crate) fn serialize_optional_unix_timestamp<S>(
    time: &Option<SystemTime>,
    serializer: S,
) -> std::result::Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match time {
        Some(time) => {
            let duration = time
                .duration_since(UNIX_EPOCH)
                .map_err(|_| S::Error::custom("SystemTime is before the UNIX epoch"))?;
            serializer.serialize_u64(duration.as_secs())
        }
        None => serializer.serialize_none(),
    }
}

pub(crate) fn deserialize_optional_unix_timestamp<'de, D>(
    deserializer: D,
) -> std::result::Result<Option<SystemTime>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<u64>::deserialize(deserializer)?;
    match opt {
        Some(timestamp) => {
            let system_time = UNIX_EPOCH
                .checked_add(Duration::from_secs(timestamp))
                .ok_or_else(|| D::Error::custom("Timestamp overflow when adding to UNIX_EPOCH"))?;
            Ok(Some(system_time))
        }
        None => Ok(None),
    }
}
