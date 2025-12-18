# wp-infras

![CI](https://github.com/wp-labs/wp-infras/workflows/CI/badge.svg)
[![codecov](https://codecov.io/gh/wp-labs/wp-infras/graph/badge.svg?token=6SVCXBHB6B)](https://codecov.io/gh/wp-labs/wp-infras)
![License](https://img.shields.io/badge/license-Elastic%20License%202.0-blue.svg)
![Rust](https://img.shields.io/badge/rust-stable%2Bbeta-orange.svg)

## Workspace Overview

| Crate | Description |
| --- | --- |
| `wp-conf-base` | Common configuration helpers: `ConfStdOperation`, `Validate`, tag parsing utilities, and backwards-compatible boolean deserializers. |
| `wp-data-fmt` | Output formatting adapters for JSON/CSV/KV/Raw/ProtoText/SQL with snapshot tests (e.g., nginx log samples). |
| `wp-error` | Unified domain error types, sys-error-code (SysErrorCode) mapping, HTTP status mapping, and error-response builders. |
| `wp-log` | Logging bootstrap (console/file/rolling), structured level configs, validation helpers. |
| `wp-specs` | Warp Parse specifications, sample configs, and integration suites. |

## Development Guide

1. **Environment**: Rust stable toolchain, `cargo`, and git LFS if you need sample data.
2. **Build/Test**: Run `cargo test` at the workspace root to build everything, or `cargo test -p <crate>` for an individual crate.
3. **Formatting/Lint**: Use `cargo fmt`; optionally run `cargo clippy` for extra linting.
4. **Contribution Flow**:
   - Sync main before changes: `git pull origin main`.
   - Create a topic branch per feature/fix; ensure `cargo fmt` and `cargo test -p <crate>` pass before committing.
   - Update relevant docs (`README.md`, `CODES.md`, etc.) to keep contracts in sync.
   - Open a PR for review once everything is green.

## License

This project is licensed under the Elastic License 2.0. See the [LICENSE](LICENSE) file for details.

**Important restrictions of the Elastic License 2.0:**

- You may not provide the software to third parties as a hosted or managed service
- You may not move, change, disable, or circumvent the license key functionality
- You may not alter, remove, or obscure any licensing, copyright, or other notices

For the full license text, please visit: https://www.elastic.co/licensing/elastic-license

## Contributing

Issues and PRs are welcome. Check each crate's README for in-depth usage. When adding new error codes, formats, or config traits, remember to update the docs and add test coverage.
