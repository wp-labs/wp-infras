use chrono::NaiveDateTime;
use std::net::{IpAddr, Ipv4Addr};
use wp_data_fmt::{DataFormat, Raw};
use wp_model_core::model::types::value::ObjectValue;
use wp_model_core::model::{DataField, DataRecord};

// 生成 Raw 文本的快照测试，参考 nginx_proto_txt_snapshot.rs
// 关注点：
// - 字符串不加引号，按原样输出（包括末尾空格）
// - 字段之间以单个空格拼接；若字段自身以空格结尾，可能出现连续空格
#[test]
fn nginx_access_log_raw_snapshot() {
    let ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 2));
    let ts = NaiveDateTime::parse_from_str("2019-08-06 12:12:19", "%Y-%m-%d %H:%M:%S").unwrap();

    let record = DataRecord {
        items: vec![
            DataField::from_ip("ip", ip),
            DataField::from_time("time", ts),
            DataField::from_chars("http/request", "GET /nginx-logo.png HTTP/1.1"),
            DataField::from_digit("http/status", 200),
            DataField::from_digit("length", 368),
            DataField::from_chars("chars", "http://119.122.1.4/"),
            DataField::from_chars(
                "http/agent",
                "Mozilla/5.0(Macintosh; Intel Mac OS X 10_14_5) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/75.0.3770.142 Safari/537.36 ",
            ),
            DataField::from_chars("src_key", "_"),
        ],
    };

    let f = Raw::new();
    let out = f.format_record(&record);

    // 注意：User-Agent 字段原始值以空格结尾，拼接空格后形成两个连续空格
    let expected = r#"192.168.1.2 2019-08-06 12:12:19 GET /nginx-logo.png HTTP/1.1 200 368 http://119.122.1.4/ Mozilla/5.0(Macintosh; Intel Mac OS X 10_14_5) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/75.0.3770.142 Safari/537.36  _"#;
    assert_eq!(out, expected);
}

#[test]
fn raw_keeps_nested_values() {
    let mut obj = ObjectValue::new();
    obj.insert("inner".to_string(), DataField::from_digit("inner", 7));
    let array = vec![
        DataField::from_chars("a", "foo"),
        DataField::from_digit("b", 9),
    ];

    let record = DataRecord {
        items: vec![
            DataField::from_obj("payload", obj),
            DataField::from_arr("list", array),
        ],
    };

    let out = Raw::new().format_record(&record);
    assert!(out.contains("{inner=7}"), "output: {}", out);
    assert!(out.contains("[foo, 9]"), "output: {}", out);
}
