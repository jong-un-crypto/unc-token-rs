use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

use crate::UncToken;

impl Serialize for UncToken {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::Error;
        let mut buf = [0u8; 40];
        let remainder = {
            use std::io::Write;
            let mut w: &mut [u8] = &mut buf;
            write!(w, "{}", self.inner)
                .map_err(|err| Error::custom(format!("Failed to serialize: {}", err)))?;
            w.len()
        };
        let len = buf.len() - remainder;

        let s = std::str::from_utf8(&buf[..len])
            .map_err(|err| Error::custom(format!("Failed to serialize: {}", err)))?;
        serializer.serialize_str(s)
    }
}

impl<'de> Deserialize<'de> for UncToken {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        s.parse::<u128>()
            .map(UncToken::from_attounc)
            .map_err(|err| de::Error::custom(err.to_string()))
    }
}

#[cfg(test)]
mod test {
    use crate::UncToken;

    #[test]
    fn json_ser() {
        fn test_json_ser(val: u128) {
            let gas = UncToken::from_attounc(val);
            let ser = serde_json::to_string(&gas).unwrap();
            assert_eq!(ser, format!("\"{}\"", val));
            let de: UncToken = serde_json::from_str(&ser).unwrap();
            assert_eq!(de.as_attounc(), val);
        }

        test_json_ser(u128::MAX);
        test_json_ser(8);
        test_json_ser(0);
    }
}
