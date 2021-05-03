use proc_macro2::TokenStream;
use serde::de::{Deserialize, Deserializer, MapAccess, Visitor};
use serde_derive::Deserialize;
use std::fmt;

#[derive(Deserialize, Debug)]
pub struct Template {
    pub fields: Fields,
    pub behaviors: Vec<String>,
    pub weak_name: Option<String>,
    pub doc: String,
    pub weak_doc: Option<String>,
}

#[derive(Debug)]
pub struct Fields(pub Vec<FieldData>);

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
            match map.next_entry::<String, FieldDataRaw>() {
                Ok(Some((name, data))) => v.push(FieldData::new(name, data)),
                Ok(None) => break Ok(Fields(v)),
                Err(e) => break Err(e),
            }
        }
    }
}

#[derive(Debug)]
pub struct FieldData {
    pub ts: TokenStream,
    pub wts: Option<TokenStream>,
}

impl FieldData {
    pub fn new(name: String, data: FieldDataRaw) -> FieldData {
        let vis = data.vis.as_ref().map(String::as_str).unwrap_or("");
        let ts = format!("{} {}: {},", vis, name, data.ty).parse().unwrap();
        let wts = data
            .weak_ty
            .as_ref()
            .map(|t| format!("{} {}: {},", vis, name, t).parse().unwrap());
        FieldData { ts, wts }
    }
}

#[derive(Debug, Deserialize)]
pub struct FieldDataRaw {
    pub vis: Option<String>,
    pub ty: String,
    pub weak_ty: Option<String>,
}
