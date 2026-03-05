use assert_cmd::Command;
use predicates::prelude::*;

fn junction_cmd() -> Command {
    #[allow(deprecated)]
    Command::cargo_bin("junction").unwrap()
}

#[test]
fn help_flag_shows_usage() {
    junction_cmd()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("CLI for the Junction API"))
        .stdout(predicate::str::contains("configure"))
        .stdout(predicate::str::contains("link"))
        .stdout(predicate::str::contains("summary"))
        .stdout(predicate::str::contains("timeseries"))
        .stdout(predicate::str::contains("user"))
        .stdout(predicate::str::contains("order"))
        .stdout(predicate::str::contains("lab-tests"))
        .stdout(predicate::str::contains("lab-report"))
        .stdout(predicate::str::contains("team"))
        .stdout(predicate::str::contains("insurance"))
        .stdout(predicate::str::contains("aggregate"))
        .stdout(predicate::str::contains("introspect"))
        .stdout(predicate::str::contains("providers"));
}

#[test]
fn version_flag() {
    junction_cmd()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("junction"));
}

#[test]
fn no_args_shows_help() {
    junction_cmd()
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
}

#[test]
fn summary_subcommand_help() {
    junction_cmd()
        .args(["summary", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("sleep"))
        .stdout(predicate::str::contains("activity"))
        .stdout(predicate::str::contains("workouts"))
        .stdout(predicate::str::contains("body"))
        .stdout(predicate::str::contains("meal"))
        .stdout(predicate::str::contains("profile"))
        .stdout(predicate::str::contains("electrocardiogram"))
        .stdout(predicate::str::contains("menstrual-cycle"))
        .stdout(predicate::str::contains("sleep-cycle"))
        .stdout(predicate::str::contains("raw"));
}

#[test]
fn link_subcommand_help() {
    junction_cmd()
        .args(["link", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("providers"))
        .stdout(predicate::str::contains("token"))
        .stdout(predicate::str::contains("demo"))
        .stdout(predicate::str::contains("bulk-ops"))
        .stdout(predicate::str::contains("bulk-import"))
        .stdout(predicate::str::contains("bulk-export"))
        .stdout(predicate::str::contains("oauth"));
}

#[test]
fn user_subcommand_help() {
    junction_cmd()
        .args(["user", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("create"))
        .stdout(predicate::str::contains("get"))
        .stdout(predicate::str::contains("update"))
        .stdout(predicate::str::contains("delete"))
        .stdout(predicate::str::contains("devices"))
        .stdout(predicate::str::contains("resolve"))
        .stdout(predicate::str::contains("refresh"));
}

#[test]
fn timeseries_subcommand_help() {
    junction_cmd()
        .args(["timeseries", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("get"))
        .stdout(predicate::str::contains("grouped"))
        .stdout(predicate::str::contains("sleep-stream"))
        .stdout(predicate::str::contains("workout-stream"));
}

#[test]
fn order_subcommand_help() {
    junction_cmd()
        .args(["order", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("create"))
        .stdout(predicate::str::contains("get"))
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("cancel"))
        .stdout(predicate::str::contains("testkit"))
        .stdout(predicate::str::contains("phlebotomy"))
        .stdout(predicate::str::contains("psc"))
        .stdout(predicate::str::contains("transaction"));
}

#[test]
fn order_phlebotomy_subcommand_help() {
    junction_cmd()
        .args(["order", "phlebotomy", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("book"))
        .stdout(predicate::str::contains("cancel"))
        .stdout(predicate::str::contains("reschedule"))
        .stdout(predicate::str::contains("availability"));
}

#[test]
fn lab_tests_subcommand_help() {
    junction_cmd()
        .args(["lab-tests", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"))
        .stdout(predicate::str::contains("create"))
        .stdout(predicate::str::contains("labs"))
        .stdout(predicate::str::contains("markers"));
}

#[test]
fn insurance_subcommand_help() {
    junction_cmd()
        .args(["insurance", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("search-payor"))
        .stdout(predicate::str::contains("search-diagnosis"))
        .stdout(predicate::str::contains("validate-icd-codes"));
}

#[test]
fn team_subcommand_help() {
    junction_cmd()
        .args(["team", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("get"))
        .stdout(predicate::str::contains("link-config"))
        .stdout(predicate::str::contains("source-priorities"))
        .stdout(predicate::str::contains("physicians"));
}

#[test]
fn aggregate_subcommand_help() {
    junction_cmd()
        .args(["aggregate", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("query"))
        .stdout(predicate::str::contains("result-table"))
        .stdout(predicate::str::contains("task-history"));
}

#[test]
fn summary_sleep_requires_start_date() {
    junction_cmd()
        .args(["summary", "sleep", "some-user-id"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("--start-date"));
}

#[test]
fn summary_sleep_requires_user_id() {
    junction_cmd()
        .args(["summary", "sleep", "--start-date", "2024-01-01"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("<USER_ID>").or(predicate::str::contains("user_id")));
}

#[test]
fn link_token_requires_user_id() {
    junction_cmd()
        .args(["link", "token"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("--user-id"));
}

#[test]
fn unknown_subcommand_fails() {
    junction_cmd()
        .arg("foobar")
        .assert()
        .failure()
        .stderr(predicate::str::contains("foobar").or(predicate::str::contains("invalid")));
}

#[test]
fn configure_without_args_shows_config() {
    junction_cmd()
        .arg("configure")
        .assert()
        .success()
        .stdout(predicate::str::contains("Config path:"))
        .stdout(predicate::str::contains("Base URL:"));
}

#[test]
fn timeseries_get_requires_start_date() {
    junction_cmd()
        .args(["timeseries", "get", "user-id", "heartrate"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("--start-date"));
}

#[test]
fn order_create_requires_data() {
    junction_cmd()
        .args(["order", "create"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("--data"));
}

#[test]
fn user_create_requires_client_user_id() {
    junction_cmd()
        .args(["user", "create"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("--client-user-id"));
}
