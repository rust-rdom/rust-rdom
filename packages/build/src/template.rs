use serde::de::{Deserialize, Deserializer, MapAccess, SeqAccess, Visitor};
use serde_derive::Deserialize;
use std::fmt;

#[derive(Deserialize, Debug)]
pub(crate) struct Template {
    pub fields: Fields,
    pub behaviors: Vec<String>,
}

// vis, ident, ty
#[derive(Debug)]
pub(crate) struct Fields(pub Vec<(String, String, String)>);

impl<'de> Deserialize<'de> for Fields {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(FieldsVisitor)
    }
}

struct FieldsVisitor;

impl<'de> Visitor<'de> for FieldsVisitor {
    type Value = Fields;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a map of structs to their typenames")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut v = vec![];
        loop {
            match map.next_entry::<String, TEntry>() {
                Ok(Some((ident, te))) => v.push((te.0, ident, te.1)),
                Ok(None) => break Ok(Fields(v)),
                Err(e) => break Err(e),
            }
        }
    }
}

#[derive(Debug)]
pub(crate) struct TEntry(pub String, pub String);

impl<'de> Deserialize<'de> for TEntry {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(TEntryVisitor)
    }
}

struct TEntryVisitor;

impl<'de> Visitor<'de> for TEntryVisitor {
    type Value = TEntry;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a pair or a string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(TEntry(String::new(), v.to_string()))
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let vis = seq.next_element()?.unwrap();
        let ty = seq.next_element()?.unwrap();
        Ok(TEntry(vis, ty))
    }
}
