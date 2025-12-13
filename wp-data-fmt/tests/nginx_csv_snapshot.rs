use chrono::NaiveDateTime;
use std::net::{IpAddr, Ipv4Addr};
use wp_data_fmt::{Csv, DataFormat};
use wp_model_core::model::{DataField, DataRecord};

// 生成 CSV 文本的快照测试，参考 nginx_proto_txt_snapshot.rs
// 关注点：
// - 仅在包含分隔符/换行/引号时加引号；引号以重复一个引号进行转义
// - ip/time 输出为纯文本（无引号，除非命中转义条件）
// - 字段之间使用逗号分隔，不带多余空格
#[test]
fn nginx_access_log_csv_snapshot() {
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

    let f = Csv::new();
    let out = f.format_record(&record);

    // 注意：User-Agent 字段包含逗号，需要整体加引号
    let expected = r#"192.168.1.2,2019-08-06 12:12:19,GET /nginx-logo.png HTTP/1.1,200,368,http://119.122.1.4/,"Mozilla/5.0(Macintosh; Intel Mac OS X 10_14_5) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/75.0.3770.142 Safari/537.36 ",_"#;
    assert_eq!(out, expected);
}

#[test]
fn csv_string_escape() {
    // 验证引号与分隔符触发正确转义
    let record = DataRecord {
        items: vec![
            DataField::from_chars("msg", "He said \"hi\""),
            DataField::from_chars("note", "a,b"),
            DataField::from_digit("n", 1),
        ],
    };
    let f = Csv::new();
    let out = f.format_record(&record);
    // 双引号使用两个连续引号进行转义；包含逗号的字段整体需要加引号
    let expected = r#""He said ""hi""","a,b",1"#;
    assert_eq!(out, expected);
}
