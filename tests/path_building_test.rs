use junction::commands::summary::build_summary_path;

#[test]
fn build_path_with_all_params() {
    let path = build_summary_path(
        "sleep",
        "user-123",
        "2024-01-01",
        Some("2024-01-31"),
        Some("oura"),
    );
    assert_eq!(
        path,
        "/v2/summary/sleep/user-123?start_date=2024-01-01&end_date=2024-01-31&provider=oura"
    );
}

#[test]
fn build_path_without_end_date() {
    let path = build_summary_path("activity", "user-456", "2024-03-01", None, Some("fitbit"));
    assert_eq!(
        path,
        "/v2/summary/activity/user-456?start_date=2024-03-01&provider=fitbit"
    );
}

#[test]
fn build_path_without_provider() {
    let path = build_summary_path(
        "workouts",
        "user-789",
        "2024-06-01",
        Some("2024-06-30"),
        None,
    );
    assert_eq!(
        path,
        "/v2/summary/workouts/user-789?start_date=2024-06-01&end_date=2024-06-30"
    );
}

#[test]
fn build_path_minimal() {
    let path = build_summary_path("body", "u1", "2024-01-01", None, None);
    assert_eq!(path, "/v2/summary/body/u1?start_date=2024-01-01");
}

#[test]
fn build_path_with_uuid_user_id() {
    let path = build_summary_path(
        "sleep",
        "550e8400-e29b-41d4-a716-446655440000",
        "2024-01-01",
        None,
        None,
    );
    assert_eq!(
        path,
        "/v2/summary/sleep/550e8400-e29b-41d4-a716-446655440000?start_date=2024-01-01"
    );
}

#[test]
fn build_path_all_resource_types() {
    for resource in &["sleep", "activity", "workouts", "body", "meal"] {
        let path = build_summary_path(resource, "u1", "2024-01-01", None, None);
        assert!(
            path.starts_with(&format!("/v2/summary/{resource}/u1")),
            "Path for {resource} should start correctly"
        );
        assert!(
            path.contains("start_date=2024-01-01"),
            "Path for {resource} should contain start_date"
        );
    }
}
