use ql_wiki::{OKF_WIKI_PROFILE, parse_okf_wiki};

const FOREIGN: &str = include_str!("../../../fixtures/qw1/foreign-wiki.md");

#[test]
fn foreign_open_wiki_node_parses_without_ql_mef_ownership_or_type_rewrite() {
    let document = parse_okf_wiki(FOREIGN).unwrap();
    assert_eq!(document.wiki.profile, OKF_WIKI_PROFILE);
    assert_eq!(
        document.wiki.canonical_ref,
        "example:knowledge:foreign-node"
    );
    assert_eq!(document.wiki.revision, 3);
    assert_eq!(document.wiki.string("type"), Some("Research Note"));
    assert_eq!(document.ql_mef_profile(), None);
    assert_eq!(document.okf["producer_extension"]["kept"], true);
    assert_eq!(
        document.wiki.raw["producer_extension"]["unknown"],
        "preserve-me"
    );
}
