use serde::de::{Deserialize, Deserializer, MapAccess, Visitor};
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
            match map.next_entry::<String, String>() {
                Ok(Some((ident, decl))) => {
                    let decl = decl.split_whitespace().collect::<Vec<_>>();
                    if decl.len() == 1 {
                        v.push((String::new(), ident, decl[0].to_string()));
                    } else {
                        v.push((decl[0].to_string(), ident, decl[1].to_string()));
                    }
                }
                Ok(None) => break Ok(Fields(v)),
                Err(e) => break Err(e),
            }
        }
    }
}
