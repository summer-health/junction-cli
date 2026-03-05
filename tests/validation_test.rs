use junction::validate;

#[test]
fn valid_dates() {
    assert!(validate::date("2024-01-01").is_ok());
    assert!(validate::date("2024-12-31").is_ok());
    assert!(validate::date("1999-06-15").is_ok());
    assert!(validate::date("2026-03-05").is_ok());
}

#[test]
fn invalid_date_format() {
    assert!(validate::date("20240101").is_err());
    assert!(validate::date("2024/01/01").is_err());
    assert!(validate::date("not-a-date").is_err());
    assert!(validate::date("").is_err());
    assert!(validate::date("2024-1-1").is_err());
}

#[test]
fn invalid_date_values() {
    assert!(validate::date("2024-13-01").is_err()); // month > 12
    assert!(validate::date("2024-00-01").is_err()); // month 0
    assert!(validate::date("2024-01-32").is_err()); // day > 31
    assert!(validate::date("2024-01-00").is_err()); // day 0
}

#[test]
fn valid_uuids() {
    assert!(validate::uuid("550e8400-e29b-41d4-a716-446655440000").is_ok());
    assert!(validate::uuid("550e8400e29b41d4a716446655440000").is_ok());
    assert!(validate::uuid("ABCDEF00-1234-5678-9ABC-DEF012345678").is_ok());
}

#[test]
fn invalid_uuids() {
    assert!(validate::uuid("not-a-uuid").is_err());
    assert!(validate::uuid("").is_err());
    assert!(validate::uuid("550e8400-e29b-41d4-a716-44665544000").is_err()); // too short
    assert!(validate::uuid("550e8400-e29b-41d4-a716-4466554400000").is_err()); // too long
    assert!(validate::uuid("550e8400-e29b-41d4-a716-44665544000g").is_err()); // invalid hex
}

#[test]
fn valid_json() {
    assert!(validate::json("{}").is_ok());
    assert!(validate::json(r#"{"key": "value"}"#).is_ok());
    assert!(validate::json("[1, 2, 3]").is_ok());
    assert!(validate::json(r#"{"nested": {"a": 1}}"#).is_ok());
    assert!(validate::json("null").is_ok());
    assert!(validate::json("true").is_ok());
}

#[test]
fn invalid_json() {
    assert!(validate::json("not json").is_err());
    assert!(validate::json("{missing: quotes}").is_err());
    assert!(validate::json("").is_err());
    assert!(validate::json("{").is_err());
}

#[test]
fn output_path_valid() {
    assert!(validate::output_path("output.pdf").is_ok());
    assert!(validate::output_path("/tmp/output.pdf").is_ok());
}

#[test]
fn output_path_invalid_parent() {
    assert!(validate::output_path("/nonexistent/deeply/nested/dir/output.pdf").is_err());
}
