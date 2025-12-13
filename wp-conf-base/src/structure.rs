use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use wp_model_core::model::TagSet;

use orion_conf::error::OrionConfResult;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Clone)]
pub enum Protocol {
    #[serde(rename = "tcp")]
    TCP,
    #[default]
    #[serde(rename = "udp")]
    UDP,
}

pub trait TagParse {
    fn take_tag(&self) -> TagSet;
}
pub trait GetTagStr {
    fn tag_vec_str(&self) -> &Vec<String>;
}

impl FromStr for Protocol {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "tcp" => Ok(Protocol::TCP),
            "udp" => Ok(Protocol::UDP),
            _ => Err(anyhow::anyhow!(
                "Unsupported protocol '{}' for syslog server. Supported protocols are: tcp, udp",
                s
            )),
        }
    }
}
impl<T> TagParse for T
where
    T: GetTagStr,
{
    fn take_tag(&self) -> TagSet {
        let mut tags = TagSet::default();
        for s in self.tag_vec_str() {
            if let Some((k, v)) = s.split_once(':') {
                let key = k.trim();
                if key.is_empty() {
                    log::warn!("ignore tag with empty key: {}", s);
                    continue;
                }
                tags.append(key, v.trim())
            }
        }
        tags
    }
}

impl Display for Protocol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Protocol::TCP => {
                write!(f, "tcp")?;
            }
            Protocol::UDP => {
                write!(f, "udp")?;
            }
        }
        Ok(())
    }
}

/// 统一的配置对象操作接口
pub trait ConfStdOperation {
    fn try_load(path: &str) -> OrionConfResult<Option<Self>>
    where
        Self: Sized,
    {
        if std::path::Path::new(path).exists() {
            match Self::load(path) {
                Ok(conf) => Ok(Some(conf)),
                Err(e) => {
                    log::warn!("load conf error: {}", e);
                    Err(e)
                }
            }
        } else {
            Ok(None)
        }
    }

    fn load(path: &str) -> OrionConfResult<Self>
    where
        Self: Sized;
    fn init(path: &str) -> OrionConfResult<Self>
    where
        Self: Sized;
    fn safe_clean(path: &str) -> OrionConfResult<()>;
}

// Unified validate hook for configuration structs
pub trait Validate {
    fn validate(&self) -> OrionConfResult<()> {
        Ok(())
    }
}

/// Backward‑compatible boolean deserializer that accepts:
/// - native bool (true/false)
/// - strings: "on"/"off", "true"/"false", "1"/"0", "yes"/"no" (case‑insensitive)
///
/// # Examples
///
/// ```json
/// { "enabled": true }      // OK
/// { "enabled": "on" }      // OK
/// { "enabled": "1" }       // OK
/// { "enabled": "yes" }     // OK
/// { "enabled": "invalid" } // Error
/// ```
pub fn de_bool_onoff<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Error as DeError;
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum In {
        B(bool),
        S(String),
        I(i64),
    }
    match In::deserialize(deserializer)? {
        In::B(b) => Ok(b),
        In::I(i) => match i {
            0 => Ok(false),
            1 => Ok(true),
            other => Err(D::Error::custom(format!(
                "invalid boolean value: {} (expect 0 or 1)",
                other
            ))),
        },
        In::S(s) => {
            let v = s.trim().to_ascii_lowercase();
            match v.as_str() {
                "on" | "true" | "1" | "yes" | "y" => Ok(true),
                "off" | "false" | "0" | "no" | "n" => Ok(false),
                other => Err(D::Error::custom(format!(
                    "invalid boolean value: {} (expect on/off or true/false)",
                    other
                ))),
            }
        }
    }
}
