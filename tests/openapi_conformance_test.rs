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

/// Every API path the CLI uses. This must stay in sync with the command modules.
fn cli_api_paths() -> Vec<(String, String)> {
    vec![
        // Link endpoints
        ("GET".into(), "/v2/link/providers".into()),
        ("POST".into(), "/v2/link/token".into()),
        ("POST".into(), "/v2/link/token/isValid".into()),
        ("POST".into(), "/v2/link/code/create".into()),
        ("GET".into(), "/v2/link/state".into()),
        (
            "GET".into(),
            "/v2/link/provider/oauth/{oauth_provider}".into(),
        ),
        (
            "POST".into(),
            "/v2/link/provider/password/{provider}".into(),
        ),
        (
            "POST".into(),
            "/v2/link/provider/password/{provider}/complete_mfa".into(),
        ),
        ("POST".into(), "/v2/link/provider/email/{provider}".into()),
        ("POST".into(), "/v2/link/provider/manual/{provider}".into()),
        ("POST".into(), "/v2/link/connect/demo".into()),
        ("GET".into(), "/v2/link/bulk_op".into()),
        ("POST".into(), "/v2/link/bulk_import".into()),
        ("POST".into(), "/v2/link/bulk_export".into()),
        ("POST".into(), "/v2/link/bulk_pause".into()),
        (
            "POST".into(),
            "/v2/link/bulk_trigger_historical_pull".into(),
        ),
        // Summary endpoints
        ("GET".into(), "/v2/summary/sleep/{user_id}".into()),
        ("GET".into(), "/v2/summary/activity/{user_id}".into()),
        ("GET".into(), "/v2/summary/workouts/{user_id}".into()),
        ("GET".into(), "/v2/summary/body/{user_id}".into()),
        ("GET".into(), "/v2/summary/meal/{user_id}".into()),
        ("GET".into(), "/v2/summary/profile/{user_id}".into()),
        (
            "GET".into(),
            "/v2/summary/electrocardiogram/{user_id}".into(),
        ),
        ("GET".into(), "/v2/summary/menstrual_cycle/{user_id}".into()),
        ("GET".into(), "/v2/summary/sleep_cycle/{user_id}".into()),
        ("GET".into(), "/v2/summary/sleep/{user_id}/raw".into()),
        ("GET".into(), "/v2/summary/activity/{user_id}/raw".into()),
        ("GET".into(), "/v2/summary/workouts/{user_id}/raw".into()),
        ("GET".into(), "/v2/summary/body/{user_id}/raw".into()),
        ("GET".into(), "/v2/summary/profile/{user_id}/raw".into()),
        ("GET".into(), "/v2/summary/devices/{user_id}/raw".into()),
        // User endpoints
        ("GET".into(), "/v2/user".into()),
        ("POST".into(), "/v2/user".into()),
        ("GET".into(), "/v2/user/{user_id}".into()),
        ("PATCH".into(), "/v2/user/{user_id}".into()),
        ("DELETE".into(), "/v2/user/{user_id}".into()),
        ("GET".into(), "/v2/user/resolve/{client_user_id}".into()),
        ("GET".into(), "/v2/user/{user_id}/device".into()),
        ("GET".into(), "/v2/user/{user_id}/device/{device_id}".into()),
        ("GET".into(), "/v2/user/providers/{user_id}".into()),
        ("GET".into(), "/v2/user/{user_id}/info/latest".into()),
        ("PATCH".into(), "/v2/user/{user_id}/info".into()),
        ("GET".into(), "/v2/user/{user_id}/insurance/latest".into()),
        ("POST".into(), "/v2/user/{user_id}/insurance".into()),
        ("POST".into(), "/v2/user/refresh/{user_id}".into()),
        ("POST".into(), "/v2/user/undo_delete".into()),
        ("DELETE".into(), "/v2/user/{user_id}/{provider}".into()),
        ("POST".into(), "/v2/user/{user_id}/sign_in_token".into()),
        ("POST".into(), "/v2/user/{user_id}/create_portal_url".into()),
        ("GET".into(), "/v2/user/metrics".into()),
        // Timeseries (spot-check representative endpoints)
        (
            "GET".into(),
            "/v2/timeseries/sleep/{sleep_id}/stream".into(),
        ),
        (
            "GET".into(),
            "/v2/timeseries/workouts/{workout_id}/stream".into(),
        ),
        // Order endpoints
        ("POST".into(), "/v3/order".into()),
        ("POST".into(), "/v3/order/import".into()),
        ("GET".into(), "/v3/order/{order_id}".into()),
        ("GET".into(), "/v3/orders".into()),
        ("POST".into(), "/v3/order/{order_id}/cancel".into()),
        ("PATCH".into(), "/v3/order/{order_id}/draw_completed".into()),
        ("POST".into(), "/v3/order/{order_id}/test".into()),
        ("POST".into(), "/v3/order/resend_events".into()),
        ("GET".into(), "/v3/order/area/info".into()),
        ("GET".into(), "/v3/order/{order_id}/result".into()),
        ("GET".into(), "/v3/order/{order_id}/result/metadata".into()),
        ("GET".into(), "/v3/order/{order_id}/result/pdf".into()),
        ("GET".into(), "/v3/order/{order_id}/abn_pdf".into()),
        (
            "GET".into(),
            "/v3/order/{order_id}/collection_instruction_pdf".into(),
        ),
        ("GET".into(), "/v3/order/{order_id}/labels/pdf".into()),
        ("GET".into(), "/v3/order/{order_id}/requisition/pdf".into()),
        ("POST".into(), "/v3/order/testkit".into()),
        ("POST".into(), "/v3/order/testkit/register".into()),
        (
            "GET".into(),
            "/v3/order/{order_id}/phlebotomy/appointment".into(),
        ),
        (
            "POST".into(),
            "/v3/order/{order_id}/phlebotomy/appointment/book".into(),
        ),
        (
            "POST".into(),
            "/v3/order/{order_id}/phlebotomy/appointment/request".into(),
        ),
        (
            "PATCH".into(),
            "/v3/order/{order_id}/phlebotomy/appointment/cancel".into(),
        ),
        (
            "PATCH".into(),
            "/v3/order/{order_id}/phlebotomy/appointment/reschedule".into(),
        ),
        (
            "POST".into(),
            "/v3/order/phlebotomy/appointment/availability".into(),
        ),
        (
            "GET".into(),
            "/v3/order/phlebotomy/appointment/cancellation-reasons".into(),
        ),
        ("GET".into(), "/v3/order/{order_id}/psc/appointment".into()),
        ("GET".into(), "/v3/order/{order_id}/psc/info".into()),
        ("GET".into(), "/v3/order/psc/info".into()),
        (
            "POST".into(),
            "/v3/order/{order_id}/psc/appointment/book".into(),
        ),
        (
            "PATCH".into(),
            "/v3/order/{order_id}/psc/appointment/cancel".into(),
        ),
        (
            "PATCH".into(),
            "/v3/order/{order_id}/psc/appointment/reschedule".into(),
        ),
        (
            "POST".into(),
            "/v3/order/psc/appointment/availability".into(),
        ),
        (
            "GET".into(),
            "/v3/order/psc/appointment/cancellation-reasons".into(),
        ),
        // Order transactions
        (
            "GET".into(),
            "/v3/order_transaction/{transaction_id}".into(),
        ),
        (
            "GET".into(),
            "/v3/order_transaction/{transaction_id}/result".into(),
        ),
        (
            "GET".into(),
            "/v3/order_transaction/{transaction_id}/result/pdf".into(),
        ),
        // Lab tests
        ("GET".into(), "/v3/lab_tests".into()),
        ("POST".into(), "/v3/lab_tests".into()),
        ("GET".into(), "/v3/lab_tests/{lab_test_id}".into()),
        ("PATCH".into(), "/v3/lab_tests/{lab_test_id}".into()),
        ("GET".into(), "/v3/lab_tests/labs".into()),
        ("GET".into(), "/v3/lab_tests/markers".into()),
        (
            "GET".into(),
            "/v3/lab_tests/{lab_id}/markers/{provider_id}".into(),
        ),
        ("GET".into(), "/v3/lab_tests/{lab_test_id}/markers".into()),
        (
            "GET".into(),
            "/v3/lab_test/{lab_test_id}/collection_instruction_pdf".into(),
        ),
        ("POST".into(), "/v3/lab_tests/list_order_set_markers".into()),
        ("GET".into(), "/v3/lab_test".into()),
        // Lab account
        ("GET".into(), "/v3/lab_test/lab_account".into()),
        // Lab report
        ("POST".into(), "/lab_report/v1/parser/job".into()),
        ("GET".into(), "/lab_report/v1/parser/job/{job_id}".into()),
        // Team
        ("GET".into(), "/v2/team/{team_id}".into()),
        ("GET".into(), "/v2/team/link/config".into()),
        ("GET".into(), "/v2/team/source/priorities".into()),
        ("PATCH".into(), "/v2/team/source/priorities".into()),
        ("GET".into(), "/v2/team/svix/url".into()),
        ("GET".into(), "/v2/team/users/search".into()),
        ("GET".into(), "/v2/team/{team_id}/physicians".into()),
        // Insurance
        ("GET".into(), "/v3/insurance/search/payor".into()),
        ("POST".into(), "/v3/insurance/search/payor".into()),
        ("GET".into(), "/v3/insurance/search/diagnosis".into()),
        ("POST".into(), "/v3/insurance/validate_icd_codes".into()),
        // Payor
        ("POST".into(), "/v3/payor".into()),
        // Compendium
        ("POST".into(), "/v3/compendium/search".into()),
        ("POST".into(), "/v3/compendium/convert".into()),
        // Aggregate
        ("POST".into(), "/aggregate/v1/user/{user_id}/query".into()),
        (
            "GET".into(),
            "/aggregate/v1/user/{user_id}/continuous_query/{query_id_or_slug}/result_table".into(),
        ),
        (
            "GET".into(),
            "/aggregate/v1/user/{user_id}/continuous_query/{query_id_or_slug}/task_history".into(),
        ),
        // Introspect
        ("GET".into(), "/v2/introspect/historical_pull".into()),
        ("GET".into(), "/v2/introspect/resources".into()),
        // Providers
        ("GET".into(), "/v2/providers".into()),
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
        "/v2/summary/electrocardiogram/{user_id}",
        "/v2/summary/menstrual_cycle/{user_id}",
        "/v2/summary/sleep_cycle/{user_id}",
    ];

    for path in summary_paths {
        let params = &spec["paths"][path]["get"]["parameters"];
        let has_start_date = params
            .as_array()
            .map(|arr| arr.iter().any(|p| p["name"].as_str() == Some("start_date")))
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
    let paths = spec["paths"].as_object().unwrap();
    let first_path = paths.values().next().unwrap();
    let first_method = first_path.as_object().unwrap().values().next().unwrap();

    let has_security = first_method.get("security").is_some()
        || spec.get("security").is_some()
        || spec["components"]["securitySchemes"].is_object();
    assert!(
        has_security,
        "OpenAPI spec should define authentication scheme"
    );
}

#[test]
fn order_endpoints_exist_in_spec() {
    let spec = load_openapi_spec();
    let spec_paths = get_spec_paths(&spec);

    let order_paths = vec![
        ("POST", "/v3/order"),
        ("GET", "/v3/order/{order_id}"),
        ("GET", "/v3/orders"),
        ("POST", "/v3/order/{order_id}/cancel"),
        ("GET", "/v3/order/{order_id}/result"),
        ("POST", "/v3/order/testkit"),
        ("POST", "/v3/order/testkit/register"),
        ("GET", "/v3/order/{order_id}/phlebotomy/appointment"),
        ("POST", "/v3/order/{order_id}/phlebotomy/appointment/book"),
        ("GET", "/v3/order/{order_id}/psc/appointment"),
        ("POST", "/v3/order/{order_id}/psc/appointment/book"),
    ];

    for (method, path) in order_paths {
        assert!(
            spec_paths.contains(&(method.to_string(), path.to_string())),
            "Order endpoint {method} {path} should exist in spec"
        );
    }
}

#[test]
fn user_crud_endpoints_exist_in_spec() {
    let spec = load_openapi_spec();
    let spec_paths = get_spec_paths(&spec);

    let user_paths = vec![
        ("GET", "/v2/user"),
        ("POST", "/v2/user"),
        ("GET", "/v2/user/{user_id}"),
        ("PATCH", "/v2/user/{user_id}"),
        ("DELETE", "/v2/user/{user_id}"),
        ("GET", "/v2/user/resolve/{client_user_id}"),
        ("POST", "/v2/user/refresh/{user_id}"),
    ];

    for (method, path) in user_paths {
        assert!(
            spec_paths.contains(&(method.to_string(), path.to_string())),
            "User endpoint {method} {path} should exist in spec"
        );
    }
}

#[test]
fn lab_test_endpoints_exist_in_spec() {
    let spec = load_openapi_spec();
    let spec_paths = get_spec_paths(&spec);

    let lab_paths = vec![
        ("GET", "/v3/lab_tests"),
        ("POST", "/v3/lab_tests"),
        ("GET", "/v3/lab_tests/{lab_test_id}"),
        ("PATCH", "/v3/lab_tests/{lab_test_id}"),
        ("GET", "/v3/lab_tests/labs"),
        ("GET", "/v3/lab_tests/markers"),
        ("GET", "/v3/lab_test/lab_account"),
    ];

    for (method, path) in lab_paths {
        assert!(
            spec_paths.contains(&(method.to_string(), path.to_string())),
            "Lab test endpoint {method} {path} should exist in spec"
        );
    }
}

#[test]
fn full_coverage_check() {
    let spec = load_openapi_spec();
    let spec_paths = get_spec_paths(&spec);
    let cli_paths: HashSet<(String, String)> = cli_api_paths().into_iter().collect();

    // Timeseries endpoints are handled dynamically (user passes metric name),
    // so we exclude them from the coverage check
    let uncovered: Vec<_> = spec_paths
        .iter()
        .filter(|(_, path)| !path.starts_with("/v2/timeseries/{user_id}/"))
        .filter(|entry| !cli_paths.contains(*entry))
        .collect();

    // Print uncovered for visibility, but don't fail — timeseries streams are covered,
    // and deprecated endpoints may be intentionally skipped
    if !uncovered.is_empty() {
        println!(
            "\nEndpoints in spec but not explicitly in CLI test list ({}):",
            uncovered.len()
        );
        let mut sorted: Vec<_> = uncovered.iter().map(|(m, p)| format!("{m} {p}")).collect();
        sorted.sort();
        for p in &sorted {
            println!("  {p}");
        }
    }

    // The only uncovered should be the dynamic timeseries endpoints
    // and deprecated link endpoints (start, auth, auth/email)
    let non_timeseries_uncovered: Vec<_> = uncovered
        .iter()
        .filter(|(_, path)| {
            !path.contains("/timeseries/")
                && !path.contains("/v2/link/start")
                && !path.contains("/v2/link/auth")
        })
        .collect();

    assert!(
        non_timeseries_uncovered.is_empty(),
        "Non-timeseries, non-deprecated endpoints missing from CLI:\n  {}",
        non_timeseries_uncovered
            .iter()
            .map(|(m, p)| format!("{m} {p}"))
            .collect::<Vec<_>>()
            .join("\n  ")
    );
}
