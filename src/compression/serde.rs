use serde::{Deserialize, Deserializer, Serializer};
use base64::{engine::general_purpose::STANDARD, Engine};
use crate::PixelSet;

pub(crate) fn serialize<S>(pixel_set: &PixelSet, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let compressed = super::compress_to_bytes(pixel_set)
        .map_err(serde::ser::Error::custom)?;

    if serializer.is_human_readable() {
        let encoded = STANDARD.encode(&compressed);
        serializer.serialize_str(&encoded)
    } else {
        serializer.serialize_bytes(&compressed)
    }
}

pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<PixelSet, D::Error>
where
    D: Deserializer<'de>,
{
    if deserializer.is_human_readable() {
        let encoded: String = Deserialize::deserialize(deserializer)?;
        let bytes = STANDARD.decode(encoded)
            .map_err(serde::de::Error::custom)?;
        super::decompress_from_bytes(&bytes)
            .map_err(serde::de::Error::custom)
    } else {
        let bytes: Vec<u8> = Deserialize::deserialize(deserializer)?;
        super::decompress_from_bytes(&bytes)
            .map_err(serde::de::Error::custom)
    }
}
