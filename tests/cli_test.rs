use assert_cmd::Command;
use predicates::prelude::*;

fn junction_cmd() -> Command {
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
        .stdout(predicate::str::contains("user"));
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
        .stdout(predicate::str::contains("demo"));
}

#[test]
fn user_subcommand_help() {
    junction_cmd()
        .args(["user", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("get"))
        .stdout(predicate::str::contains("devices"));
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
    // Should succeed (shows current config) even without an API key
    junction_cmd()
        .arg("configure")
        .assert()
        .success()
        .stdout(predicate::str::contains("Config path:"))
        .stdout(predicate::str::contains("Base URL:"));
}
