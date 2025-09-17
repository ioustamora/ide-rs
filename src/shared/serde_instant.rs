use serde::{Serializer, Deserializer, Deserialize};
use std::time::{Instant, Duration, SystemTime, UNIX_EPOCH};

/// Serialization support for Instant by converting to duration since a fixed reference point.
/// Since Instant cannot be serialized directly (it's relative to an arbitrary epoch),
/// we convert it to a SystemTime for serialization purposes.
///
/// Note: This is a best-effort approach. Deserialized Instant values may not be exactly
/// the same as the original due to the conversion through SystemTime.

pub fn serialize<S>(instant: &Instant, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // Convert Instant to SystemTime for serialization
    // This is an approximation since Instant is relative to an arbitrary epoch
    let now_instant = Instant::now();
    let now_system = SystemTime::now();

    let system_time = if *instant <= now_instant {
        let duration_ago = now_instant.duration_since(*instant);
        now_system.checked_sub(duration_ago).unwrap_or(now_system)
    } else {
        let duration_ahead = instant.duration_since(now_instant);
        now_system.checked_add(duration_ahead).unwrap_or(now_system)
    };

    let duration_since_epoch = system_time.duration_since(UNIX_EPOCH)
        .map_err(serde::ser::Error::custom)?;
    let millis = duration_since_epoch.as_millis() as u64;
    serializer.serialize_u64(millis)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Instant, D::Error>
where
    D: Deserializer<'de>,
{
    let millis = u64::deserialize(deserializer)?;
    let system_time = UNIX_EPOCH + Duration::from_millis(millis);

    // Convert SystemTime back to Instant
    // This is an approximation since Instant is relative to an arbitrary epoch
    let now_system = SystemTime::now();
    let now_instant = Instant::now();

    let instant = if system_time <= now_system {
        let duration_ago = now_system.duration_since(system_time)
            .map_err(serde::de::Error::custom)?;
        now_instant.checked_sub(duration_ago).unwrap_or(now_instant)
    } else {
        let duration_ahead = system_time.duration_since(now_system)
            .map_err(serde::de::Error::custom)?;
        now_instant.checked_add(duration_ahead).unwrap_or(now_instant)
    };

    Ok(instant)
}

pub mod option {
    use serde::{Serializer, Deserializer, Deserialize};
    use std::time::{Instant, Duration, SystemTime, UNIX_EPOCH};

    pub fn serialize<S>(opt: &Option<Instant>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match opt {
            Some(instant) => super::serialize(instant, serializer),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Instant>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let opt: Option<u64> = Option::deserialize(deserializer)?;
        match opt {
            Some(millis) => {
                let system_time = UNIX_EPOCH + Duration::from_millis(millis);

                // Convert SystemTime back to Instant
                let now_system = SystemTime::now();
                let now_instant = Instant::now();

                let instant = if system_time <= now_system {
                    let duration_ago = now_system.duration_since(system_time)
                        .map_err(serde::de::Error::custom)?;
                    now_instant.checked_sub(duration_ago).unwrap_or(now_instant)
                } else {
                    let duration_ahead = system_time.duration_since(now_system)
                        .map_err(serde::de::Error::custom)?;
                    now_instant.checked_add(duration_ahead).unwrap_or(now_instant)
                };

                Ok(Some(instant))
            }
            None => Ok(None),
        }
    }
}