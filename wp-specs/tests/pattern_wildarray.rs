use serde::{Deserialize, Serialize};
use wp_specs::WildArray;

#[test]
fn wildarray_toml_roundtrip() {
    let wa = WildArray::new1(vec!["a*", "b?"]);

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct Wrapper {
        w: WildArray,
    }

    let w = Wrapper { w: wa.clone() };
    let s = toml::to_string(&w).expect("serialize to toml (wrapped)");
    let w2: Wrapper = toml::from_str(&s).expect("deserialize from toml (wrapped)");

    assert_eq!(w, w2);
}

#[test]
fn wildarray_matching_semantics() {
    let wa = WildArray::new1(vec!["oml/example_*", "rule_*"]);

    assert!(wa.0[0].matches("oml/example_1"));
    assert!(wa.0[1].matches("rule_abc"));
    assert!(!wa.0[0].matches("oml/other"));
}
