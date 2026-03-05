# junction-cli

A command-line interface for the [Junction API](https://docs.junction.com/api-reference/), built in Rust.

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

# Get a user's profile
junction user get USER_ID
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

Fetch health data summaries for a user.

```sh
junction summary sleep <USER_ID> --start-date 2024-01-01 [--end-date 2024-01-31] [--provider oura]
junction summary activity <USER_ID> --start-date 2024-01-01
junction summary workouts <USER_ID> --start-date 2024-01-01
junction summary body <USER_ID> --start-date 2024-01-01
junction summary meal <USER_ID> --start-date 2024-01-01
junction summary profile <USER_ID>
```

Use `junction summary raw` to get raw provider data for any summary type:

```sh
junction summary raw sleep <USER_ID> --start-date 2024-01-01
junction summary raw devices <USER_ID>
```

### `junction link`

Manage provider connections.

```sh
junction link providers           # list all available providers
junction link token --user-id ID  # generate a link token
junction link demo --user-id ID   # create a demo connection
```

### `junction user`

User profile and device information.

```sh
junction user get <USER_ID>      # get user profile
junction user devices <USER_ID>  # list connected devices
```

## Testing

```sh
cargo test
```

Tests include:

- **CLI integration tests** — argument parsing, help output, required parameters
- **HTTP client tests** — mock server tests for auth headers, request/response handling, error cases
- **Config tests** — serialization roundtrip, defaults, API key resolution
- **OpenAPI conformance tests** — validates that all CLI endpoints exist in the Junction OpenAPI spec and match expected parameter requirements
- **Path building tests** — URL construction with various parameter combinations

### OpenAPI spec validation

The test suite includes a snapshot of the [Junction OpenAPI spec](https://api.tryvital.io/openapi.json) at `tests/openapi.json`. Tests verify that every API path the CLI uses actually exists in the spec.

CI runs a daily check to detect when the upstream spec changes, so the CLI can be updated accordingly.

## CI

GitHub Actions runs on every push and PR:

- Build + test
- Clippy linting
- Format check
- Daily OpenAPI spec freshness check

## License

MIT
