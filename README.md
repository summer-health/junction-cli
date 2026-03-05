# junction-cli

A command-line interface for the [Junction API](https://docs.junction.com/api-reference/), built in Rust. Covers the full API surface including health data, lab testing, orders, and team management.

## Installation

### From source

```sh
cargo install --path .
```

### Prerequisites

- [Rust](https://rustup.rs/) 1.85+

## Quick start

```sh
# Configure your API key
junction configure --api-key YOUR_API_KEY

# Or use an environment variable
export JUNCTION_API_KEY=YOUR_API_KEY

# List available providers
junction link providers

# Get a user's sleep data
junction summary sleep USER_ID --start-date 2024-01-01

# Get heart rate timeseries
junction timeseries get USER_ID heartrate --start-date 2024-01-01

# Create a lab order
junction order create --data '{"user_id": "...", "lab_test_id": "..."}'

# Get order results
junction order result ORDER_ID
```

## Configuration

Configuration is stored at `~/.config/junction/config.toml` (macOS/Linux).

```sh
# Set API key
junction configure --api-key sk_live_...

# Set a custom base URL
junction configure --base-url https://api.tryvital.io

# View current configuration
junction configure
```

You can also set `JUNCTION_API_KEY` as an environment variable. The config file takes precedence over the environment variable.

## Commands

### `junction configure`

Set or view API key and base URL.

```sh
junction configure --api-key <KEY> --base-url <URL>
junction configure  # show current config
```

### `junction summary`

Fetch health data summaries for a user. All date-accepting commands validate `YYYY-MM-DD` format.

```sh
junction summary sleep <USER_ID> --start-date 2024-01-01 [--end-date 2024-01-31] [--provider oura]
junction summary activity <USER_ID> --start-date 2024-01-01
junction summary workouts <USER_ID> --start-date 2024-01-01
junction summary body <USER_ID> --start-date 2024-01-01
junction summary meal <USER_ID> --start-date 2024-01-01
junction summary electrocardiogram <USER_ID> --start-date 2024-01-01
junction summary menstrual-cycle <USER_ID> --start-date 2024-01-01
junction summary sleep-cycle <USER_ID> --start-date 2024-01-01
junction summary profile <USER_ID>
junction summary raw <TYPE> <USER_ID> [--start-date ...]
```

### `junction timeseries`

Fetch timeseries health metrics. Supports 50+ metrics including `heartrate`, `hrv`, `steps`, `blood_oxygen`, `glucose`, `cholesterol/hdl`, and more.

```sh
junction timeseries get <USER_ID> <METRIC> --start-date 2024-01-01
junction timeseries grouped <USER_ID> <METRIC> --start-date 2024-01-01
junction timeseries sleep-stream <SLEEP_ID>
junction timeseries workout-stream <WORKOUT_ID>
```

### `junction user`

Full user lifecycle management.

```sh
junction user list
junction user create --client-user-id my-user-123
junction user get <USER_ID>
junction user update <USER_ID> --data '{"fallback_time_zone": "US/Eastern"}'
junction user delete <USER_ID>
junction user resolve <CLIENT_USER_ID>
junction user devices <USER_ID>
junction user providers <USER_ID>
junction user info <USER_ID>
junction user insurance <USER_ID>
junction user refresh <USER_ID>
junction user sign-in-token <USER_ID>
junction user portal-url <USER_ID>
junction user metrics
```

### `junction link`

Manage provider connections, OAuth flows, and bulk operations.

```sh
junction link providers
junction link token --user-id <ID>
junction link oauth <PROVIDER> --vital-link-token <TOKEN>
junction link connect-password <PROVIDER> --data '...'
junction link connect-email <PROVIDER> --data '...'
junction link demo --user-id <ID>
junction link bulk-ops
junction link bulk-import --data '...'
junction link bulk-export --data '...'
junction link bulk-pause --data '...'
junction link bulk-historical-pull --data '...'
```

### `junction order`

Lab test order management with full lifecycle support.

```sh
# Orders
junction order create --data '...'
junction order get <ORDER_ID>
junction order list
junction order cancel <ORDER_ID>
junction order import --data '...'
junction order add-test <ORDER_ID> --data '...'
junction order result <ORDER_ID>
junction order result-metadata <ORDER_ID>
junction order area-info

# PDFs
junction order result-pdf <ORDER_ID> --output result.pdf
junction order abn-pdf <ORDER_ID> --output abn.pdf
junction order requisition-pdf <ORDER_ID>
junction order labels-pdf <ORDER_ID>
junction order collection-instruction-pdf <ORDER_ID>

# Testkits
junction order testkit create --data '...'
junction order testkit register --data '...'

# Phlebotomy appointments
junction order phlebotomy get <ORDER_ID>
junction order phlebotomy book <ORDER_ID> --data '...'
junction order phlebotomy request <ORDER_ID> --data '...'
junction order phlebotomy cancel <ORDER_ID>
junction order phlebotomy reschedule <ORDER_ID> --data '...'
junction order phlebotomy availability --data '...'
junction order phlebotomy cancellation-reasons

# PSC appointments
junction order psc get <ORDER_ID>
junction order psc info <ORDER_ID>
junction order psc general-info
junction order psc book <ORDER_ID> --data '...'
junction order psc cancel <ORDER_ID>
junction order psc reschedule <ORDER_ID> --data '...'
junction order psc availability --data '...'

# Transactions
junction order transaction get <TRANSACTION_ID>
junction order transaction result <TRANSACTION_ID>
junction order transaction result-pdf <TRANSACTION_ID>
```

### `junction lab-tests`

Lab test definitions, markers, and labs.

```sh
junction lab-tests list
junction lab-tests get <LAB_TEST_ID>
junction lab-tests create --data '...'
junction lab-tests update <LAB_TEST_ID> --data '...'
junction lab-tests labs
junction lab-tests markers
junction lab-tests lab-markers <LAB_ID> <PROVIDER_ID>
junction lab-tests test-markers <LAB_TEST_ID>
junction lab-tests order-set-markers --data '...'
junction lab-tests collection-instruction-pdf <LAB_TEST_ID> -o instruction.pdf
```

### `junction lab-report`

Parse lab reports.

```sh
junction lab-report create --data '...'
junction lab-report get <JOB_ID>
```

### `junction lab-accounts`

List lab accounts.

```sh
junction lab-accounts
```

### `junction team`

Team and org management.

```sh
junction team get <TEAM_ID>
junction team link-config
junction team source-priorities
junction team update-source-priorities --data '...'
junction team svix-url
junction team search-users
junction team physicians <TEAM_ID>
```

### `junction insurance`

Insurance payor search and validation.

```sh
junction insurance search-payor [--query <QUERY>]
junction insurance search-diagnosis [--query <QUERY>]
junction insurance validate-icd-codes --data '...'
```

### `junction payor-create`

Create a custom payor.

```sh
junction payor-create --data '...'
```

### `junction compendium`

Compendium search and conversion.

```sh
junction compendium search --data '...'
junction compendium convert --data '...'
```

### `junction aggregate`

Horizon AI query engine.

```sh
junction aggregate query <USER_ID> --data '{"select": [...]}'
junction aggregate result-table <USER_ID> <QUERY_ID>
junction aggregate task-history <USER_ID> <QUERY_ID>
```

### `junction introspect`

API introspection.

```sh
junction introspect historical-pull
junction introspect resources
```

### `junction providers`

List all available data providers.

```sh
junction providers
```

## Input validation

The CLI validates inputs before making API calls:

- **Dates** must be in `YYYY-MM-DD` format with valid month/day ranges
- **JSON bodies** (`--data`) are parsed and validated before sending
- **Output paths** are checked to ensure parent directories exist

## Testing

```sh
cargo test
```

76 tests across 7 test suites:

- **CLI integration tests** (21) — argument parsing, help output, required parameters for all command groups
- **HTTP client tests** (6) — mock server tests for auth headers, GET/POST/DELETE/PATCH, error handling
- **Config tests** (10) — serialization roundtrip, defaults, API key resolution
- **OpenAPI conformance tests** (10) — validates all CLI endpoints exist in the spec, checks parameter requirements, verifies full API coverage
- **Path building tests** (6) — URL construction with various parameter combinations
- **Validation tests** (9) — date format, UUID format, JSON parsing, output path checks
- **Unit tests** (14) — inline validation module tests

### OpenAPI spec validation

The test suite includes a snapshot of the [Junction OpenAPI spec](https://api.tryvital.io/openapi.json) at `tests/openapi.json`. Tests verify that every API path the CLI uses actually exists in the spec, and a full coverage check ensures no non-deprecated endpoints are missed.

CI runs a daily check to detect when the upstream spec changes.

## CI

GitHub Actions runs on every push and PR:

- Build + test
- Clippy linting
- Format check
- Daily OpenAPI spec freshness check

## License

MIT
