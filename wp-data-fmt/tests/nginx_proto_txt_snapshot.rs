use chrono::NaiveDateTime;
use std::net::{IpAddr, Ipv4Addr};
use wp_data_fmt::{DataFormat, ProtoTxt};
use wp_model_core::model::{DataField, DataRecord};

// 生成 proto-text 文本的快照测试，参考 nginx_kv_snapshot.rs
// 关注点：
// - 字符串需要使用双引号并转义内部的引号
// - ip/time 以字符串输出（带引号）
// - 字段之间使用换行分隔，不带逗号
#[test]
fn nginx_access_log_proto_text_snapshot() {
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

    let f = ProtoTxt::new();
    let out = f.format_record(&record);

    let expected = format!("{{ {} }}", [
        "ip: \"192.168.1.2\"",
        "time: \"2019-08-06 12:12:19\"",
        "http/request: \"GET /nginx-logo.png HTTP/1.1\"",
        "http/status: 200",
        "length: 368",
        "chars: \"http://119.122.1.4/\"",
        "http/agent: \"Mozilla/5.0(Macintosh; Intel Mac OS X 10_14_5) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/75.0.3770.142 Safari/537.36 \"",
        "src_key: \"_\"",
    ]
    .join(" "));

    assert_eq!(out, expected);
}

#[test]
fn proto_text_string_escape() {
    // 验证内部双引号被正确转义
    let record = DataRecord {
        items: vec![
            DataField::from_chars("msg", "He said \"hi\""),
            DataField::from_digit("n", 1),
        ],
    };
    let f = ProtoTxt::new();
    let out = f.format_record(&record);
    let expected = format!(
        "{{ {} }}",
        ["msg: \"He said \\\"hi\\\"\"", "n: 1",].join(" ")
    );
    assert_eq!(out, expected);
}
