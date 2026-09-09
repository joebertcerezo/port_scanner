# Port Scanner

A fast, asynchronous TCP port scanner written in Rust.

The scanner checks a target IPv4 or IPv6 address over a configurable port range and reports open ports together with their detected service names.

## Features

- Asynchronous TCP connection scanning with Tokio
- Configurable target IPv4 or IPv6 address
- Custom start and end ports
- Configurable concurrency limit
- Configurable connection timeout
- Open, closed, filtered, and error port states
- Common service detection based on port numbers
- Deterministic output sorted by port number
- Input validation for port ranges, concurrency, and timeout values
- Unit and asynchronous integration-style tests

## Requirements

- Rust and Cargo
- A supported Rust toolchain

Install Rust using [rustup](https://rustup.rs/) if it is not already installed.

Verify the installation:

```bash
rustc --version
cargo --version
```

## Installation

Clone the project and enter its directory:

```bash
git clone https://github.com/joebertcerezo/port_scanner.git
cd port_scanner
```

Build the project:

```bash
cargo build
```

## Usage

Run the scanner with the default configuration:

```bash
cargo run
```

Run a scan against a specific address and port range:

```bash
cargo run -- --address 127.0.0.1 --start 1 --end 1024 --concurrency 250 --timeout 500
```

The options are:

| Option | Short | Description | Default |
|---|---:|---|---:|
| `--address` | `-a` | Target IPv4 or IPv6 address | `127.0.0.1` |
| `--start` | `-s` | First port to scan | `1` |
| `--end` | `-e` | Last port to scan | `65535` |
| `--concurrency` | `-c` | Maximum number of simultaneous connections | `100` |
| `--timeout` | `-t` | Connection timeout in milliseconds | `1500` |
| `--help` |  | Display command-line help |  |
| `--version` |  | Display the application version |  |

Port zero is not accepted as a scan target.

## Sample Run

```text
$ cargo run -- --address 127.0.0.1 --start 1 --end 1024 --concurrency 250 --timeout 500
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/port_scanner --address 127.0.0.1 --start 1 --end 1024 --concurrency 250 --timeout 500`

Port Scanner
──────────────────────────────────────────────────
Target       127.0.0.1
Ports        1-1024
Concurrency  250
Timeout      500ms
──────────────────────────────────────────────────

Scan finished in 50.26ms

TARGET          PORT       STATE      SERVICE
-------------------------------------------------------
127.0.0.1       22         OPEN       SSH
127.0.0.1       631        OPEN       UNKNOWN
```

Only open ports are displayed in the final command-line output. Internally, each scanned port can be classified as:

- `OPEN`: A TCP connection was established.
- `CLOSED`: The target actively refused the connection.
- `FILTERED`: The connection timed out.
- `ERROR`: Another connection error occurred.

## Service Detection

Service names are inferred from well-known port numbers:

| Port | Service |
|---:|---|
| 21 | FTP |
| 22 | SSH |
| 23 | Telnet |
| 25 | SMTP |
| 53 | DNS |
| 80 | HTTP |
| 110 | POP3 |
| 143 | IMAP |
| 443 | HTTPS |
| 445 | SMB |
| 3306 | MySQL |
| 3389 | RDP |
| 5432 | PostgreSQL |
| 6379 | Redis |
| 8080 | HTTP Proxy |
| 27017 | MongoDB |

Ports without a configured mapping are reported as `UNKNOWN`.

## Project Structure

```text
port_scanner/
├── Cargo.toml
├── README.md
├── `rust-toolchain.toml`
├── rustfmt.toml
└── src/
    ├── app.rs              # Application workflow and output formatting
    ├── args.rs             # Command-line argument parsing
    ├── config.rs           # Scanner configuration and validation
    ├── error.rs            # Application and configuration errors
    ├── lib.rs              # Library module exports
    ├── main.rs             # Binary entry point and tests
    ├── models/
    │   ├── mod.rs
    │   ├── result.rs       # ScanResult
    │   └── state.rs        # PortState
    └── scanner/
        ├── engine.rs       # Concurrent scan orchestration
        ├── mod.rs
        ├── service.rs      # Port-to-service mappings
        └── target.rs       # Individual TCP port scanning
```

## How It Works

1. Command-line arguments are parsed with `bpaf`.
2. The configuration is validated.
3. The requested port range is converted into asynchronous scan tasks.
4. Tokio attempts a TCP connection to each target port.
5. `buffer_unordered` limits the number of concurrent scans.
6. Each connection result is converted into a `ScanResult`.
7. Open ports are collected and sorted by port number.
8. The results are printed to the terminal.

## Testing

Run all tests:

```bash
cargo test
```

Run tests with output enabled:

```bash
cargo test -- --nocapture
```

The test suite covers:

- Configuration validation
- Port range validation
- Concurrency validation
- Ephemeral open TCP ports
- Closed TCP port detection
- IPv6 localhost scanning
- Service mappings

## Formatting and Linting

Check formatting:

```bash
cargo fmt --check
```

Format the project:

```bash
cargo fmt
```

Run Clippy:

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

Run the complete quality checks:

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

## Performance Notes

Higher concurrency can reduce scan duration, but excessive concurrency may:

- Consume more system resources
- Trigger operating-system socket limits
- Overload the target
- Cause more connection errors

The scanner limits concurrency through the `--concurrency` option. Choose a value appropriate for the target and the local system.

## Responsible Use

Only scan systems and networks that you own or have explicit permission to test.

Port scanning unauthorized systems may violate laws, policies, or terms of service.
