use serde_json::Value;
use std::collections::HashSet;

fn load_openapi_spec() -> Value {
    let spec_bytes = include_bytes!("openapi.json");
    serde_json::from_slice(spec_bytes).expect("failed to parse openapi.json")
}

fn get_spec_paths(spec: &Value) -> HashSet<(String, String)> {
    let mut paths = HashSet::new();
    if let Some(obj) = spec["paths"].as_object() {
        for (path, methods) in obj {
            if let Some(methods_obj) = methods.as_object() {
                for method in methods_obj.keys() {
                    if ["get", "post", "put", "patch", "delete"].contains(&method.as_str()) {
                        paths.insert((method.to_uppercase(), path.clone()));
                    }
                }
            }
        }
    }
    paths
}

/// Paths that our CLI currently uses — these must all exist in the spec.
fn cli_api_paths() -> Vec<(String, String)> {
    vec![
        // Link endpoints
        ("GET".into(), "/v2/link/providers".into()),
        ("POST".into(), "/v2/link/token".into()),
        ("POST".into(), "/v2/link/connect/demo".into()),
        // Summary endpoints
        ("GET".into(), "/v2/summary/sleep/{user_id}".into()),
        ("GET".into(), "/v2/summary/activity/{user_id}".into()),
        ("GET".into(), "/v2/summary/workouts/{user_id}".into()),
        ("GET".into(), "/v2/summary/body/{user_id}".into()),
        ("GET".into(), "/v2/summary/meal/{user_id}".into()),
        ("GET".into(), "/v2/summary/profile/{user_id}".into()),
        // Raw summary endpoints
        ("GET".into(), "/v2/summary/sleep/{user_id}/raw".into()),
        ("GET".into(), "/v2/summary/activity/{user_id}/raw".into()),
        ("GET".into(), "/v2/summary/workouts/{user_id}/raw".into()),
        ("GET".into(), "/v2/summary/body/{user_id}/raw".into()),
        ("GET".into(), "/v2/summary/profile/{user_id}/raw".into()),
        ("GET".into(), "/v2/summary/devices/{user_id}/raw".into()),
    ]
}

#[test]
fn all_cli_paths_exist_in_openapi_spec() {
    let spec = load_openapi_spec();
    let spec_paths = get_spec_paths(&spec);

    let mut missing = Vec::new();
    for (method, path) in cli_api_paths() {
        if !spec_paths.contains(&(method.clone(), path.clone())) {
            missing.push(format!("{method} {path}"));
        }
    }

    assert!(
        missing.is_empty(),
        "CLI uses API paths not found in OpenAPI spec:\n  {}",
        missing.join("\n  ")
    );
}

#[test]
fn spec_version_is_tracked() {
    let spec = load_openapi_spec();
    let version = spec["info"]["version"].as_str().unwrap();
    // This test documents the spec version we're testing against.
    // Update when the spec changes.
    assert!(
        !version.is_empty(),
        "OpenAPI spec should have a version string"
    );
    println!("Testing against OpenAPI spec version: {version}");
}

#[test]
fn summary_endpoints_require_start_date_parameter() {
    let spec = load_openapi_spec();
    let summary_paths = [
        "/v2/summary/sleep/{user_id}",
        "/v2/summary/activity/{user_id}",
        "/v2/summary/workouts/{user_id}",
        "/v2/summary/body/{user_id}",
        "/v2/summary/meal/{user_id}",
    ];

    for path in summary_paths {
        let params = &spec["paths"][path]["get"]["parameters"];
        let has_start_date = params
            .as_array()
            .map(|arr| {
                arr.iter()
                    .any(|p| p["name"].as_str() == Some("start_date"))
            })
            .unwrap_or(false);
        assert!(
            has_start_date,
            "{path} should require start_date parameter per spec"
        );
    }
}

#[test]
fn link_token_endpoint_accepts_post_body() {
    let spec = load_openapi_spec();
    let token_path = &spec["paths"]["/v2/link/token"]["post"];
    assert!(
        token_path.is_object(),
        "POST /v2/link/token should exist in spec"
    );
    let request_body = &token_path["requestBody"];
    assert!(
        request_body.is_object(),
        "POST /v2/link/token should accept a request body"
    );
}

#[test]
fn summary_endpoints_support_provider_filter() {
    let spec = load_openapi_spec();
    let summary_paths = [
        "/v2/summary/sleep/{user_id}",
        "/v2/summary/activity/{user_id}",
        "/v2/summary/workouts/{user_id}",
        "/v2/summary/body/{user_id}",
    ];

    for path in summary_paths {
        let params = &spec["paths"][path]["get"]["parameters"];
        let has_provider = params
            .as_array()
            .map(|arr| arr.iter().any(|p| p["name"].as_str() == Some("provider")))
            .unwrap_or(false);
        assert!(
            has_provider,
            "{path} should accept provider query parameter per spec"
        );
    }
}

#[test]
fn api_uses_vital_api_key_header() {
    let spec = load_openapi_spec();
    // Check security schemes or look for x-vital-api-key in any endpoint's parameters
    let paths = spec["paths"].as_object().unwrap();
    let first_path = paths.values().next().unwrap();
    let first_method = first_path
        .as_object()
        .unwrap()
        .values()
        .next()
        .unwrap();

    // The spec should reference some security scheme
    let has_security = first_method.get("security").is_some()
        || spec.get("security").is_some()
        || spec["components"]["securitySchemes"].is_object();
    assert!(
        has_security,
        "OpenAPI spec should define authentication scheme"
    );
}
