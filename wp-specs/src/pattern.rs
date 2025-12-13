use serde::de::SeqAccess;
use serde::ser::SerializeSeq;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use wildmatch::WildMatch;

/// Wildcard pattern array, serialized as array of strings.
/// This type is used in spec/config layers to express match lists
/// without depending on the language/runtime crates.
#[derive(PartialEq, Default, Clone)]
pub struct WildArray(pub Vec<WildMatch>);

impl AsRef<Vec<WildMatch>> for WildArray {
    fn as_ref(&self) -> &Vec<WildMatch> {
        &self.0
    }
}

impl Debug for WildArray {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl Display for WildArray {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for val in &self.0 {
            write!(f, "{}", val)?;
        }
        Ok(())
    }
}

impl WildArray {
    pub fn new(val: &str) -> WildArray {
        WildArray(vec![WildMatch::new(val)])
    }

    pub fn new1<T: Into<String>>(val: Vec<T>) -> WildArray {
        let val: Vec<WildMatch> = val.into_iter().map(|v| WildMatch::new(&v.into())).collect();
        WildArray(val)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl Serialize for WildArray {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
        for e in &self.0 {
            seq.serialize_element(&e.to_string())?;
        }
        seq.end()
    }
}

impl<'de> Deserialize<'de> for WildArray {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct WildMatchArray;

        impl<'de> serde::de::Visitor<'de> for WildMatchArray {
            type Value = WildArray;

            fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
                formatter.write_str("a sequence of wildcard patterns")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut vec = Vec::new();
                while let Ok(Some(element)) = seq.next_element::<String>() {
                    vec.push(WildMatch::new(&element));
                }
                Ok(WildArray(vec))
            }
        }

        deserializer.deserialize_seq(WildMatchArray)
    }
}
