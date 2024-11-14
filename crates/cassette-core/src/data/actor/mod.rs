pub mod boolean;
pub mod number;
pub mod string;

use std::{fmt, str::FromStr};

use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use serde_json::{Map, Value};
use thiserror::Error;

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchemaActor {
    #[serde(default)]
    pub create: Option<SchemaArray>,

    #[serde(default)]
    pub update: Option<SchemaArray>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SchemaArray(pub Vec<Schema>);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema {
    pub name: String,
    pub path: SchemaPath,
    #[serde(flatten)]
    pub ty: SchemaType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SchemaType {
    Boolean(self::boolean::SchemaSpec),
    Number(self::number::SchemaSpec),
    String(self::string::SchemaSpec),
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SchemaPath(pub Vec<SchemaPathItem>);

impl FromStr for SchemaPath {
    type Err = SchemaPathParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with("/") {
            return Err(SchemaPathParseError::NoRootSlashPrefix);
        }

        Ok(Self(
            s[1..]
                .split('/')
                // ('~1' to '/') => ('~0' to '~') => keep
                // Please see: https://datatracker.ietf.org/doc/html/rfc6901#section-4
                .map(|item| item.trim().replace("~1", "/").replace("~0", "~"))
                .filter(|item| !item.is_empty())
                .map(|item| {
                    item.parse()
                        .map(SchemaPathItem::List)
                        .unwrap_or(SchemaPathItem::Object(item))
                })
                .collect(),
        ))
    }
}

impl fmt::Display for SchemaPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "/".fmt(f)?;
        for (index, item) in self.0.iter().enumerate() {
            if index > 0 {
                "/".fmt(f)?;
            }
            item.fmt(f)?;
        }
        Ok(())
    }
}

impl Serialize for SchemaPath {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for SchemaPath {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SchemaPathVisitor;

        impl de::Visitor<'_> for SchemaPathVisitor {
            type Value = SchemaPath;

            #[inline]
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                v.parse().map_err(E::custom)
            }

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "a json path")
            }
        }

        deserializer.deserialize_string(SchemaPathVisitor)
    }
}

impl SchemaPath {
    pub fn get<'a>(&self, target: &'a Value) -> &'a Value {
        let mut target = target;
        for item in &self.0 {
            target = match item {
                &SchemaPathItem::List(i) => match target {
                    Value::Array(children) => match children.get(i) {
                        Some(child) => child,
                        None => return &Value::Null,
                    },
                    Value::Object(children) => match children.get(&i.to_string()) {
                        Some(child) => child,
                        None => return &Value::Null,
                    },
                    _ => return &Value::Null,
                },
                SchemaPathItem::Object(i) => match target {
                    Value::Object(children) => match children.get(i) {
                        Some(child) => child,
                        None => return &Value::Null,
                    },
                    _ => return &Value::Null,
                },
            };
        }
        target
    }

    fn get_mut<'a>(&self, target: &'a mut Value) -> &'a mut Value {
        let mut target = target;
        for item in &self.0 {
            target = match item {
                &SchemaPathItem::List(i) => match target {
                    Value::Array(children) => {
                        if i >= children.len() {
                            children.resize(i + 1, Value::Null);
                        }
                        &mut children[i]
                    }
                    Value::Object(children) => children.entry(i.to_string()).or_insert(Value::Null),
                    _ => {
                        *target = Value::Array(vec![Value::Null; i + 1]);
                        &mut target[i]
                    }
                },
                SchemaPathItem::Object(i) => match target {
                    Value::Object(children) => children.entry(i.clone()).or_insert(Value::Null),
                    _ => {
                        let mut map = Map::default();
                        map.insert(i.clone(), Value::Null);
                        *target = Value::Object(map);
                        target.get_mut(i).unwrap()
                    }
                },
            };
        }
        target
    }

    pub fn set(&self, target: &mut Value, value: Value) {
        *self.get_mut(target) = value
    }
}

#[derive(Clone, Debug, PartialEq, Error)]
pub enum SchemaPathParseError {
    #[error("No root slash(/) prefix")]
    NoRootSlashPrefix,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SchemaPathItem {
    List(usize),
    Object(String),
}

impl fmt::Display for SchemaPathItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SchemaPathItem::List(value) => value.fmt(f),
            SchemaPathItem::Object(value) => value.fmt(f),
        }
    }
}
