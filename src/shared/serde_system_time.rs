use serde::{Serializer, Deserializer, Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn serialize<S>(system_time: &SystemTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let duration_since_epoch = system_time.duration_since(UNIX_EPOCH)
        .map_err(serde::ser::Error::custom)?;
    let millis = duration_since_epoch.as_millis() as u64;
    serializer.serialize_u64(millis)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<SystemTime, D::Error>
where
    D: Deserializer<'de>,
{
    let millis = u64::deserialize(deserializer)?;
    Ok(UNIX_EPOCH + std::time::Duration::from_millis(millis))
}

pub mod option {
    use serde::{Serializer, Deserializer, Deserialize};
    use std::time::SystemTime;

    pub fn serialize<S>(opt: &Option<SystemTime>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match opt {
            Some(system_time) => super::serialize(system_time, serializer),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<SystemTime>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Option::<SystemTime>::deserialize(deserializer)
    }
}
