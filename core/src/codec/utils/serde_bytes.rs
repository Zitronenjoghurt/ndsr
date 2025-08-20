use serde::de::{Error, Visitor};
use serde::{Deserializer, Serializer};
use std::fmt;

pub fn serialize<S, const N: usize>(bytes: &[u8; N], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_bytes(bytes)
}

pub fn deserialize<'de, D, const N: usize>(deserializer: D) -> Result<[u8; N], D::Error>
where
    D: Deserializer<'de>,
{
    struct ByteArrayVisitor<const N: usize>;

    impl<'de, const N: usize> Visitor<'de> for ByteArrayVisitor<N> {
        type Value = [u8; N];

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "a byte array of length {}", N)
        }

        fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
        where
            E: Error,
        {
            let mut array = [0u8; N];
            if value.len() != N {
                return Err(E::custom(format!(
                    "expected {} bytes, got {}",
                    N,
                    value.len()
                )));
            }
            array.copy_from_slice(value);
            Ok(array)
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::SeqAccess<'de>,
        {
            let mut array = [0u8; N];
            for (i, byte) in array.iter_mut().enumerate() {
                *byte = seq
                    .next_element()?
                    .ok_or_else(|| Error::custom(format!("expected {} bytes, got {}", N, i)))?;
            }
            Ok(array)
        }
    }

    deserializer.deserialize_bytes(ByteArrayVisitor::<N>)
}
