use serde::{Deserialize, Deserializer, Serializer};
use time::{format_description, OffsetDateTime, PrimitiveDateTime};

const FORMAT: &str = "[year]-[month]-[day] [hour]:[minute]:[second]";

pub fn deserialize<'de, D>(deserializer: D) -> Result<OffsetDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    let format = format_description::parse(FORMAT).map_err(serde::de::Error::custom)?;

    let primitive = PrimitiveDateTime::parse(&s, &format).map_err(serde::de::Error::custom)?;

    // assume system offset
    // it's pretty safe to use the offset of the current dt, as we're deserializing the response right away
    let offset = OffsetDateTime::now_utc().offset();

    Ok(primitive.assume_offset(offset))
}

 // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(
        date: &OffsetDateTime,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {        
        // let format = format_description::parse(
        //     "[year]-[month]-[day] [hour]:[minute]:[second] [offset_hour \
        //          sign:mandatory]:[offset_minute]:[offset_second]",
        // )?;
        let s = format!("{}", date);
        serializer.serialize_str(&s)
    }