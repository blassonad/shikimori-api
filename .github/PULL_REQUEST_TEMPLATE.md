## Summary

<!-- Explain the problem, the solution and user-visible impact. Link the issue with `Fixes #123` when applicable. -->

## Public API and compatibility

- [ ] This change does not alter public API, or the SemVer impact is explained below.
- [ ] New/changed public items have Russian Rustdoc and appropriate intra-doc links.
- [ ] New/changed REST contract is reflected in `docs/endpoint-matrix.md` and API reference where applicable.
- [ ] New/changed serialization or response shape has a deterministic fixture/wire-format test.
- [ ] The change does not add GraphQL or OAuth authorization/refresh flow.

## Validation

- [ ] `cargo fmt --check`
- [ ] `cargo doc --no-deps --all-features`
- [ ] `cargo test --doc --all-features`
- [ ] `cargo test --all-targets --all-features --locked`
- [ ] `cargo clippy --all-targets --all-features -- -D warnings`
- [ ] `cargo publish --dry-run --locked` when package metadata/source contents changed
- [ ] MSRV Rust 1.75 check when code/dependencies changed

## Security and release safety

- [ ] The diff contains no token, credential, personal data, `target/` artifact or generated package archive.
- [ ] Workflow changes use least-privilege permissions and do not expose `CARGO_REGISTRY_TOKEN` to CI/PR jobs.
- [ ] If this changes `Cargo.toml` version, the release automation behavior is explained.
