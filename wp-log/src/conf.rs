use getset::Getters;
use getset::WithSetters;
use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::rolling_file::RollingFileAppender;
use log4rs::append::rolling_file::policy::compound::CompoundPolicy;
use log4rs::append::rolling_file::policy::compound::roll::fixed_window::FixedWindowRoller;
use log4rs::append::rolling_file::policy::compound::trigger::size::SizeTrigger;
use log4rs::config::{Appender, Config, Logger, Root};
use log4rs::encode::pattern::PatternEncoder;
use orion_conf::ErrorWith;
use orion_conf::ToStructError;
use orion_conf::error::ConfIOReason;
#[cfg(feature = "std")]
use orion_conf::error::OrionConfResult;
use serde::de::{Deserializer, Error as DeError};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use std::str::FromStr;
use strum_macros::Display;
#[derive(PartialEq, Deserialize, Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct FileLogConf {
    pub path: String,
}

#[derive(PartialEq, Deserialize, Serialize, Clone, Debug, WithSetters, Getters)]
#[serde(deny_unknown_fields)]
#[get = "pub"]
pub struct LogConf {
    pub level: String,
    #[serde(default)]
    pub levels: Option<BTreeMap<String, String>>, // structured levels: { global="warn", ctrl="info", ... }
    #[set_with = "pub"]
    pub output: Output,
    #[serde(default)]
    pub file: Option<FileLogConf>, // required when output has File/Both
    // Emit a clear error when legacy field is present in config
    #[serde(
        rename = "output_path",
        default,
        deserialize_with = "reject_output_path",
        skip_serializing
    )]
    _deprecated_output_path: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize, Display)]
pub enum Output {
    Console,
    File,
    Both,
}

impl Default for LogConf {
    fn default() -> Self {
        LogConf {
            // Production-oriented default:
            // - ctrl/launch keep info for operability
            // - source/sink/stat at info to observe data plane health
            // - runtime (ex-run_stg) at warn to reduce noise
            // - model libs at warn; external libs (orion_*) tightened
            level: String::from(
                "warn,ctrl=info,launch=info,source=info,sink=info,stat=info,runtime=warn,\
oml=warn,wpl=warn,klib=warn,orion_error=error,orion_sens=warn",
            ),
            levels: None,
            output: Output::File,
            file: Some(FileLogConf {
                path: "./data/logs/".to_string(),
            }),
            _deprecated_output_path: None,
        }
    }
}

impl Display for LogConf {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(map) = &self.levels {
            // print structured levels first if present
            writeln!(f, "levels: {:?}", map)?;
        } else {
            writeln!(f, "level: {}", self.level)?;
        }
        writeln!(f, "output: {}", self.output)?;
        writeln!(f, "path: {:?}", self.file.as_ref().map(|x| x.path.clone()))
    }
}
// chk_default 已移除：不再区分校验日志与运行日志

impl FromStr for LogConf {
    type Err = anyhow::Error;
    fn from_str(debug: &str) -> Result<Self, Self::Err> {
        Ok(LogConf {
            level: debug.to_string(),
            levels: None,
            output: Output::File,
            file: Some(FileLogConf {
                path: "./logs".to_string(),
            }),
            _deprecated_output_path: None,
        })
    }
}

impl LogConf {
    pub fn log_to_console(debug: &str) -> Self {
        LogConf {
            level: debug.to_string(),
            levels: None,
            output: Output::Console,
            file: None,
            _deprecated_output_path: None,
        }
    }
}

fn reject_output_path<'de, D>(_de: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    Err(D::Error::custom(
        "log_conf.output_path 已移除；请改用 [log_conf.file].dir",
    ))
}

pub const PRINT_STAT: &str = "PRINT_STAT";

#[cfg(feature = "std")]
pub fn log_init(conf: &LogConf) -> OrionConfResult<()> {
    use orion_conf::ErrorOwe;

    let (root_level, target_levels) = parse_level_spec(&conf.level)?;

    // Encoder: timestamp + [LEVEL] + [target] + message; no module path/line
    let enc = PatternEncoder::new("{d(%Y-%m-%d %H:%M:%S.%f)} [{l:5}] [{t:7}] {m}{n}");

    let mut config = Config::builder();
    let mut root = Root::builder();

    // structured levels（levels）在显示时已覆盖；解析仍走 level 字符串
    match conf.output {
        Output::Console => {
            let stdout = ConsoleAppender::builder().encoder(Box::new(enc)).build();
            config = config.appender(Appender::builder().build("stdout", Box::new(stdout)));
            root = root.appender("stdout");
        }
        Output::File => {
            use orion_conf::ErrorOwe;

            let file_path = resolve_log_file(conf)?;
            // Ensure parent dir exists
            if let Some(p) = std::path::Path::new(&file_path).parent() {
                let _ = std::fs::create_dir_all(p);
            }
            // Rolling: 10MB, keep 10 files, gzip
            let pattern = format!("{}.{{}}.gz", &file_path);
            let roller = FixedWindowRoller::builder()
                .base(0)
                .build(&pattern, 10)
                .owe_logic()
                .with(pattern.as_str())?;
            let trigger = SizeTrigger::new(10 * 1024 * 1024);
            let policy = CompoundPolicy::new(Box::new(trigger), Box::new(roller));
            let file = RollingFileAppender::builder()
                .encoder(Box::new(enc))
                .build(&file_path, Box::new(policy))
                .owe_res()
                .with(file_path.as_str())?;
            config = config.appender(Appender::builder().build("file", Box::new(file)));
            root = root.appender("file");
        }
        Output::Both => {
            use orion_conf::ErrorOwe;

            let file_path = resolve_log_file(conf)?;
            if let Some(p) = std::path::Path::new(&file_path).parent() {
                let _ = std::fs::create_dir_all(p);
            }
            let stdout = ConsoleAppender::builder()
                .encoder(Box::new(enc.clone()))
                .build();
            config = config.appender(Appender::builder().build("stdout", Box::new(stdout)));
            let pattern = format!("{}.{{}}.gz", &file_path);
            let roller = FixedWindowRoller::builder()
                .base(0)
                .build(&pattern, 10)
                .owe_logic()
                .want(pattern.as_str())?;
            let trigger = SizeTrigger::new(10 * 1024 * 1024);
            let policy = CompoundPolicy::new(Box::new(trigger), Box::new(roller));
            let file = RollingFileAppender::builder()
                .encoder(Box::new(enc))
                .build(&file_path, Box::new(policy))
                .owe_res()
                .with(file_path.as_str())?;
            config = config.appender(Appender::builder().build("file", Box::new(file)));
            root = root.appender("stdout").appender("file");
        }
    }

    for (name, lv) in target_levels {
        config = config.logger(Logger::builder().build(name.as_str(), lv));
    }

    let cfg = config
        .build(root.build(root_level))
        .owe_logic()
        .want("build log cfg")?;
    log4rs::init_config(cfg)
        .owe_logic()
        .want("init log config")?;
    Ok(())
}

#[cfg(feature = "std")]
pub fn log_for_test() -> OrionConfResult<()> {
    let conf = LogConf {
        level: "debug".into(),
        levels: None,
        output: Output::Console,
        file: None,
        _deprecated_output_path: None,
    };
    log_init(&conf)
}

#[cfg(feature = "std")]
pub fn log_for_test_level(level: &str) -> OrionConfResult<()> {
    let conf = LogConf {
        level: level.into(),
        levels: None,
        output: Output::Console,
        file: None,
        _deprecated_output_path: None,
    };
    log_init(&conf)
}

fn parse_level_spec(spec: &str) -> OrionConfResult<(LevelFilter, Vec<(String, LevelFilter)>)> {
    let mut root = LevelFilter::Info;
    let mut targets = Vec::new();
    for part in spec.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()) {
        if let Some((k, v)) = part.split_once('=') {
            targets.push((k.trim().to_string(), parse_lv(v.trim())?));
        } else {
            root = parse_lv(part)?;
        }
    }
    Ok((root, targets))
}

fn resolve_log_file(conf: &LogConf) -> OrionConfResult<String> {
    let dir = conf
        .file
        .as_ref()
        .map(|f| f.path.clone())
        .unwrap_or_else(|| "./logs".to_string());
    let arg0 = std::env::args().next().unwrap_or_else(|| "app".to_string());
    let stem = std::path::Path::new(&arg0)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("app");
    let mut p = PathBuf::from(dir);
    p.push(format!("{}.log", stem));
    Ok(p.display().to_string())
}

fn parse_lv(s: &str) -> OrionConfResult<LevelFilter> {
    match s.to_ascii_lowercase().as_str() {
        "off" => Ok(LevelFilter::Off),
        "error" => Ok(LevelFilter::Error),
        "warn" => Ok(LevelFilter::Warn),
        "info" => Ok(LevelFilter::Info),
        "debug" => Ok(LevelFilter::Debug),
        "trace" => Ok(LevelFilter::Trace),
        _ => ConfIOReason::Other("unknow log level".into())
            .err_result()
            .with(s),
    }
}
