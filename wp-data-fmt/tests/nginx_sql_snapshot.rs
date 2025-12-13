use chrono::NaiveDateTime;
use std::net::{IpAddr, Ipv4Addr};
use wp_data_fmt::{DataFormat, SqlInsert};
use wp_model_core::model::{DataField, DataRecord};

// 生成 SQL INSERT 文本的快照测试
// 关注点：
// - 列名使用双引号包裹，值中的字符串使用单引号，单引号需要以重复一个单引号转义
// - ip/time 作为字符串输出（带单引号）
#[test]
fn nginx_access_log_sql_insert_snapshot() {
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

    let f = SqlInsert::new_with_json("nginx_access");
    let out = f.format_record(&record);

    let expected = r#"INSERT INTO "nginx_access" ("ip", "time", "http/request", "http/status", "length", "chars", "http/agent", "src_key") VALUES ('192.168.1.2', '2019-08-06 12:12:19', 'GET /nginx-logo.png HTTP/1.1', 200, 368, 'http://119.122.1.4/', 'Mozilla/5.0(Macintosh; Intel Mac OS X 10_14_5) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/75.0.3770.142 Safari/537.36 ', '_');"#;
    assert_eq!(out, expected);
}

#[test]
fn sql_string_escape() {
    // 验证单引号转义（重复一个单引号）
    let record = DataRecord {
        items: vec![
            DataField::from_chars("msg", "O'Reilly"),
            DataField::from_digit("n", 1),
        ],
    };
    let f = SqlInsert::new_with_json("t");
    let out = f.format_record(&record);
    let expected = r#"INSERT INTO "t" ("msg", "n") VALUES ('O''Reilly', 1);"#;
    assert_eq!(out, expected);
}

#[test]
fn sql_upsert_quotes_conflict_columns() {
    let record = DataRecord {
        items: vec![
            DataField::from_chars("http/request", "GET /"),
            DataField::from_chars("user", "alice"),
        ],
    };

    let f = SqlInsert::new_with_json("nginx/access");
    let sql = f.format_upsert(&record, &["http/request"]);

    assert!(sql.contains("INSERT INTO \"nginx/access\""));
    assert!(
        sql.contains("ON CONFLICT (\"http/request\") DO UPDATE SET \"user\" = EXCLUDED.\"user\";")
    );
}
